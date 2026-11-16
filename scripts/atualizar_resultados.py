#!/usr/bin/env python3
"""Update the official workbook once per local working day."""
import argparse
from contextlib import contextmanager
import datetime as dt
import hashlib
from html import unescape
from http.client import HTTPException
import io
import json
import os
import re
from pathlib import Path
import sys
import tempfile
from urllib.request import Request, urlopen
import xml.etree.ElementTree as ET
from zipfile import BadZipFile, ZipFile
import zlib

URL = 'https://servicebus3.caixa.gov.br/portaldeloterias/api/resultados/download?modalidade=Dia%20de%20Sorte'
ROOT = Path(__file__).resolve().parent.parent
OUTPUT = ROOT / 'docs' / 'Dia de Sorte-atual.xlsx'
ORIGINAL = ROOT / 'docs' / 'Dia de Sorte.xlsx'
STATE = ROOT / 'docs' / '.dia-de-sorte-atualizacao.json'
NS = {'x': 'http://schemas.openxmlformats.org/spreadsheetml/2006/main'}
MAX_DOWNLOAD = 15 * 1024 * 1024
MAX_UNCOMPRESSED = 50 * 1024 * 1024
STATE_VERSION = 2
HEADERS = ['Concurso', 'Data Sorteio', *(f'Bola{i}' for i in range(1, 8)), 'Mês da Sorte']
MONTHS = {
    'janeiro', 'fevereiro', 'março', 'marco', 'abril', 'maio', 'junho', 'julho',
    'agosto', 'setembro', 'outubro', 'novembro', 'dezembro', *(str(i) for i in range(1, 13)),
}


def workbook_latest(content):
    try:
        with ZipFile(io.BytesIO(content)) as book:
            entries = book.infolist()
            if sum(entry.file_size for entry in entries) > MAX_UNCOMPRESSED:
                raise ValueError('planilha excede o limite de descompactação')
            if len({entry.filename for entry in entries}) != len(entries) or book.testzip():
                raise ValueError('arquivo Excel corrompido')
            types = ET.fromstring(book.read('[Content_Types].xml'))
            workbook = ET.fromstring(book.read('xl/workbook.xml'))
            if (types.tag != '{http://schemas.openxmlformats.org/package/2006/content-types}Types'
                    or workbook.tag != f"{{{NS['x']}}}workbook"):
                raise ValueError('estrutura interna do Excel inválida')
            package = ET.fromstring(book.read('_rels/.rels'))
            rel_ns = '{http://schemas.openxmlformats.org/package/2006/relationships}'
            if not any(rel.get('Type', '').endswith('/relationships/officeDocument')
                       and rel.get('Target') in ('xl/workbook.xml', '/xl/workbook.xml')
                       for rel in package.findall(rel_ns + 'Relationship')):
                raise ValueError('referência da pasta de trabalho inválida')
            rels = ET.fromstring(book.read('xl/_rels/workbook.xml.rels'))
            sheets = workbook.findall('x:sheets/x:sheet', NS)
            rel_id = '{http://schemas.openxmlformats.org/officeDocument/2006/relationships}id'
            rel_ns = '{http://schemas.openxmlformats.org/package/2006/relationships}'
            if not sheets or not sheets[0].get(rel_id):
                raise ValueError('planilha inicial não identificada')
            first = next((rel for rel in rels.findall(rel_ns + 'Relationship')
                          if rel.get('Id') == sheets[0].get(rel_id)), None)
            if (first is None or first.get('Target') not in ('worksheets/sheet1.xml', '/xl/worksheets/sheet1.xml')
                    or not first.get('Type', '').endswith('/worksheet')):
                raise ValueError('referência da planilha inicial inválida')
            shared = ET.fromstring(book.read('xl/sharedStrings.xml'))
            strings = [''.join(item.itertext()) for item in shared]
            sheet = ET.fromstring(book.read('xl/worksheets/sheet1.xml'))
    except (BadZipFile, KeyError, ET.ParseError, RuntimeError, NotImplementedError,
            EOFError, zlib.error) as exc:
        raise ValueError('arquivo Excel inválido ou formato alterado') from exc

    for cell in sheet.iterfind('.//x:c', NS):
        address = cell.get('r')
        if address:
            match = re.fullmatch(r'([A-Z]+)([0-9]+)', address)
            if not match or int(match.group(2)) > 100000:
                raise ValueError('coordenada de célula inválida')
            col = 0
            for char in match.group(1):
                col = col * 26 + ord(char) - ord('A') + 1
            if col > 64:
                raise ValueError('coordenada de célula inválida')
    rows = sheet.findall('x:sheetData/x:row', NS)
    if len(rows) < 2:
        raise ValueError('planilha sem resultados')

    def values(row):
        cells = row.findall('x:c', NS)[:len(HEADERS)]
        if len(cells) != len(HEADERS):
            raise ValueError('linha incompleta')
        result = []
        for column, cell in enumerate(cells):
            address = cell.get('r')
            if address:
                match = re.fullmatch(r'([A-Z]+)([0-9]+)', address)
                if not match or int(match.group(2)) > 100000 or len(match.group(1)) > 2:
                    raise ValueError('coordenada de célula inválida')
            if address and address.rstrip('0123456789') != chr(ord('A') + column):
                raise ValueError('coluna de resultado ausente ou fora de posição')
            if cell.get('t') in ('b', 'e'):
                raise ValueError('tipo de célula obrigatório inválido')
            value = cell.findtext('x:v', namespaces=NS)
            if value is None or cell.find('x:f', NS) is not None:
                raise ValueError('célula de resultado inválida')
            if cell.get('t') == 's':
                index = int(value)
                if not 0 <= index < len(strings):
                    raise ValueError('referência de texto inválida')
                value = strings[index]
            result.append(value)
        return result

    header_cells = rows[0].findall('x:c', NS)
    header_values = []
    for cell in header_cells:
        value = cell.findtext('x:v', namespaces=NS)
        if cell.get('t') == 's' and value is not None:
            index = int(value)
            if not 0 <= index < len(strings):
                raise ValueError('referência de texto inválida')
            header_values.append(strings[index])
        elif cell.get('t') == 'inlineStr':
            inline = cell.find('x:is', NS)
            if inline is not None:
                header_values.append(''.join(inline.itertext()))
    if any(header_values.count(name) > 1 for name in header_values):
        raise ValueError('coluna duplicada no cabeçalho')
    if values(rows[0]) != HEADERS:
        raise ValueError('cabeçalho inesperado')
    draws = {}
    for row in rows[1:]:
        fields = values(row)
        number = int(fields[0])
        date = dt.datetime.strptime(fields[1], '%d/%m/%Y').date()
        balls = [int(value) for value in fields[2:9]]
        month = unescape(fields[9]).strip().casefold()
        if number < 1 or balls != sorted(set(balls)) or not 1 <= balls[0] <= balls[-1] <= 31:
            raise ValueError(f'concurso ou dezenas inválidos na linha do concurso {number}')
        if month not in MONTHS:
            raise ValueError(f'mês da sorte inválido no concurso {number}')
        if number in draws:
            raise ValueError(f'concurso duplicado: {number}')
        draws[number] = date

    ordered = sorted(draws)
    if ordered[-1] != len(draws):
        raise ValueError('faltam concursos no histórico')
    if any(draws[a] > draws[b] for a, b in zip(ordered, ordered[1:])):
        raise ValueError('datas incompatíveis com a ordem dos concursos')
    return ordered[-1]


def atomic_write(path, content):
    with tempfile.NamedTemporaryFile(dir=path.parent, delete=False) as f:
        temp = Path(f.name)
        try:
            f.write(content)
            f.close()
            os.replace(temp, path)
        finally:
            f.close()
            temp.unlink(missing_ok=True)


@contextmanager
def update_lock(path):
    # Both OS locks are released when the descriptor closes, including on process exit.
    with path.open('ab') as lock:
        try:
            if os.name == 'nt':
                import msvcrt
                lock.seek(0)
                msvcrt.locking(lock.fileno(), msvcrt.LK_NBLCK, 1)
            else:
                import fcntl
                fcntl.flock(lock.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
        except OSError as exc:
            raise OSError(f'outra atualização pode estar em andamento. Falha no bloqueio: {exc}') from exc
        yield


def read_state():
    try:
        state = json.loads(STATE.read_text(encoding='utf-8'))
    except (OSError, ValueError):
        return {}
    if not isinstance(state, dict) or state.get('source') != URL:
        return {}
    return state


def previous_contest(path):
    try:
        return workbook_latest(path.read_bytes())
    except FileNotFoundError:
        return 0
    except ValueError as exc:
        print(f'Cópia local inválida em {path.name}: {exc}.', file=sys.stderr)
        return 0


def update(force=False):
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    with update_lock(STATE.with_suffix('.lock')):
        refresh(force)


def refresh(force):
    today = dt.date.today().isoformat()
    state = read_state()
    recorded_latest = state.get('latest')
    if type(recorded_latest) is not int or recorded_latest < 1:
        recorded_latest = 0
    if (not force and state.get('version') == STATE_VERSION and state.get('date') == today
            and recorded_latest and OUTPUT.is_file()):
        current_hash = hashlib.sha256(OUTPUT.read_bytes()).hexdigest()
        if state.get('sha256') == current_hash:
            print(f'Já atualizada hoje: concurso {recorded_latest} ({OUTPUT}). Use --force para baixar novamente.')
            return

    request = Request(URL, headers={'User-Agent': 'ChuteGPT-results-updater/1.0'})
    with urlopen(request, timeout=45) as response:
        content = response.read(MAX_DOWNLOAD + 1)
    if len(content) > MAX_DOWNLOAD:
        raise ValueError('download excede o limite de tamanho')
    latest = workbook_latest(content)
    local_latest = previous_contest(OUTPUT)
    if not local_latest:
        local_latest = previous_contest(ORIGINAL)
    if latest < max(local_latest, recorded_latest):
        raise ValueError('download contém concurso mais antigo que o último conhecido')

    checksum = hashlib.sha256(content).hexdigest()
    if not OUTPUT.is_file() or hashlib.sha256(OUTPUT.read_bytes()).hexdigest() != checksum:
        atomic_write(OUTPUT, content)
    state = {
        'version': STATE_VERSION,
        'date': today,
        'downloaded_at': dt.datetime.now().astimezone().isoformat(),
        'latest': latest,
        'sha256': checksum,
        'source': URL,
    }
    try:
        atomic_write(STATE, (json.dumps(state, indent=2, ensure_ascii=False) + '\n').encode())
    except OSError as exc:
        raise OSError(f'planilha validada até o concurso {latest}, mas o registro diário falhou: {exc}') from exc
    print(f'Planilha validada: concurso {latest}; arquivo: {OUTPUT}')


def main():
    parser = argparse.ArgumentParser(description='Atualiza a planilha oficial do Dia de Sorte uma vez por dia local.')
    parser.add_argument('--force', action='store_true', help='baixa novamente mesmo após atualização hoje')
    args = parser.parse_args()
    try:
        update(args.force)
    except (OSError, ValueError, HTTPException) as exc:
        print(f'Não foi possível concluir a atualização dos resultados: {exc}.', file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())
