import contextlib
from http.client import IncompleteRead
import importlib.util
import io
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
from xml.sax.saxutils import escape
from zipfile import ZipFile

spec = importlib.util.spec_from_file_location(
    'updater', Path(__file__).resolve().parents[1] / 'scripts/atualizar_resultados.py'
)
updater = importlib.util.module_from_spec(spec)
spec.loader.exec_module(updater)

NS = 'http://schemas.openxmlformats.org/spreadsheetml/2006/main'
HEADERS = ['Concurso', 'Data Sorteio', 'Bola1', 'Bola2', 'Bola3', 'Bola4', 'Bola5', 'Bola6', 'Bola7', 'Mês da Sorte']


def draw(number, date='19/05/2018', month='Maio'):
    return [number, date, 3, 5, 8, 9, 19, 21, 30, month]


def workbook(*draws):
    data = io.BytesIO()
    shared = ''.join(f'<si><t>{escape(value)}</t></si>' for value in HEADERS)
    header = ''.join(f'<c t="s"><v>{i}</v></c>' for i in range(len(HEADERS)))
    body = ''.join(
        '<row>' + ''.join(f'<c><v>{escape(str(value))}</v></c>' for value in row) + '</row>'
        for row in draws
    )
    with ZipFile(data, 'w') as book:
        book.writestr('[Content_Types].xml', '<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"/>')
        book.writestr('_rels/.rels', '<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>')
        book.writestr('xl/workbook.xml', f'<workbook xmlns="{NS}" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Resultados" sheetId="1" r:id="rId1"/></sheets></workbook>')
        book.writestr('xl/_rels/workbook.xml.rels', '<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/></Relationships>')
        book.writestr('xl/sharedStrings.xml', f'<sst xmlns="{NS}">{shared}</sst>')
        book.writestr('xl/worksheets/sheet1.xml', f'<worksheet xmlns="{NS}"><sheetData><row>{header}</row>{body}</sheetData></worksheet>')
    return data.getvalue()


class Updates(unittest.TestCase):
    def setUp(self):
        stack = contextlib.ExitStack()
        self.addCleanup(stack.close)
        directory = Path(stack.enter_context(tempfile.TemporaryDirectory()))
        self.output = directory / 'current.xlsx'
        self.state = directory / 'state.json'
        self.original = directory / 'original.xlsx'
        stack.enter_context(patch.multiple(updater, OUTPUT=self.output, STATE=self.state, ORIGINAL=self.original))
        self.network = stack.enter_context(patch.object(updater, 'urlopen'))
        self.network.side_effect = lambda *args, **kwargs: io.BytesIO(workbook(draw(1, month='Mar&ccedil;o')))

    def test_success_skip_and_force(self):
        updater.update()
        updater.update()
        self.assertEqual(self.network.call_count, 1)
        self.network.side_effect = lambda *args, **kwargs: io.BytesIO(workbook(draw(1), draw(2, month='12')))
        updater.update(force=True)
        self.assertEqual(self.network.call_count, 2)
        self.assertEqual(updater.workbook_latest(self.output.read_bytes()), 2)
        self.assertEqual(json.loads(self.state.read_text())['latest'], 2)

    def test_invalid_downloads_preserve_previous_files(self):
        updater.update()
        prior = self.output.read_bytes(), self.state.read_bytes()
        damaged_zip = workbook(draw(1)).replace(b'<workbook xmlns', b'<workbook!xmlns')
        invalid = (
            b'<html>failure</html>',
            damaged_zip,
            workbook(draw(1, month='13')),
            workbook(draw(1, date='30/02/2018')),
            workbook(draw(1), draw(3)),
            workbook(draw(1), draw(1)),
            workbook(draw(1)[:-1]),
        )
        for content in invalid:
            with self.subTest(size=len(content)):
                self.network.side_effect = lambda *args, **kwargs: io.BytesIO(content)
                with self.assertRaises(ValueError):
                    updater.update(force=True)
                self.assertEqual((self.output.read_bytes(), self.state.read_bytes()), prior)

    def test_boolean_required_cell_preserves_previous_files(self):
        updater.update()
        prior = self.output.read_bytes(), self.state.read_bytes()
        original = workbook(draw(1))
        result = io.BytesIO()
        with ZipFile(io.BytesIO(original)) as source, ZipFile(result, 'w') as target:
            for name in source.namelist():
                data = source.read(name)
                if name == 'xl/worksheets/sheet1.xml':
                    data = data.replace(b'<c><v>Maio</v></c>', b'<c t="b"><v>1</v></c>')
                target.writestr(name, data)
        self.network.side_effect = lambda *args, **kwargs: io.BytesIO(result.getvalue())
        with self.assertRaises(ValueError):
            updater.update(force=True)
        self.assertEqual((self.output.read_bytes(), self.state.read_bytes()), prior)

    def test_missing_workbook_relationship_preserves_previous_files(self):
        updater.update()
        prior = self.output.read_bytes(), self.state.read_bytes()
        result = io.BytesIO()
        with ZipFile(io.BytesIO(workbook(draw(1)))) as source, ZipFile(result, 'w') as target:
            for name in source.namelist():
                if name != 'xl/_rels/workbook.xml.rels':
                    target.writestr(name, source.read(name))
        self.network.side_effect = lambda *args, **kwargs: io.BytesIO(result.getvalue())
        with self.assertRaises(ValueError):
            updater.update(force=True)
        self.assertEqual((self.output.read_bytes(), self.state.read_bytes()), prior)

    def test_missing_package_relationship_preserves_previous_files(self):
        updater.update()
        prior = self.output.read_bytes(), self.state.read_bytes()
        result = io.BytesIO()
        with ZipFile(io.BytesIO(workbook(draw(1)))) as source, ZipFile(result, 'w') as target:
            for name in source.namelist():
                if name != '_rels/.rels':
                    target.writestr(name, source.read(name))
        self.network.side_effect = lambda *args, **kwargs: io.BytesIO(result.getvalue())
        with self.assertRaises(ValueError):
            updater.update(force=True)
        self.assertEqual((self.output.read_bytes(), self.state.read_bytes()), prior)

    def test_duplicate_header_preserves_previous_files(self):
        updater.update()
        prior = self.output.read_bytes(), self.state.read_bytes()
        result = io.BytesIO()
        with ZipFile(io.BytesIO(workbook(draw(1)))) as source, ZipFile(result, 'w') as target:
            for name in source.namelist():
                data = source.read(name)
                if name == 'xl/worksheets/sheet1.xml':
                    data = data.replace(b'</row>', b'<c t="s"><v>0</v></c></row>', 1)
                target.writestr(name, data)
        self.network.side_effect = lambda *args, **kwargs: io.BytesIO(result.getvalue())
        with self.assertRaisesRegex(ValueError, 'duplicada'):
            updater.update(force=True)
        self.assertEqual((self.output.read_bytes(), self.state.read_bytes()), prior)

    def test_network_error_preserves_previous_files(self):
        updater.update()
        prior = self.output.read_bytes(), self.state.read_bytes()
        self.network.side_effect = OSError('offline')
        with self.assertRaises(OSError):
            updater.update(force=True)
        self.assertEqual((self.output.read_bytes(), self.state.read_bytes()), prior)

    def test_incomplete_http_response_exits_cleanly(self):
        updater.update()
        prior = self.output.read_bytes(), self.state.read_bytes()
        self.network.side_effect = IncompleteRead(b'incomplete')
        with patch.object(updater.sys, 'argv', ['atualizar_resultados.py', '--force']):
            with contextlib.redirect_stderr(io.StringIO()) as error:
                self.assertEqual(updater.main(), 1)
            self.assertIn('Não foi possível concluir', error.getvalue())
        self.assertEqual((self.output.read_bytes(), self.state.read_bytes()), prior)

    def test_recovers_corrupt_workbook_and_non_object_state(self):
        self.output.write_bytes(b'invalid')
        self.state.write_text('[]')
        self.original.write_bytes(workbook(draw(1)))
        original = self.original.read_bytes()
        updater.update()
        self.assertEqual(updater.workbook_latest(self.output.read_bytes()), 1)
        self.assertEqual(self.original.read_bytes(), original)

    def test_rejects_older_download_even_when_current_is_damaged(self):
        self.output.write_bytes(workbook(draw(1), draw(2)))
        with self.assertRaisesRegex(ValueError, 'mais antigo'):
            updater.update()
        self.assertEqual(updater.workbook_latest(self.output.read_bytes()), 2)
        self.state.write_text(json.dumps({'latest': 2, 'source': updater.URL}))
        self.output.write_bytes(b'invalid')
        with self.assertRaisesRegex(ValueError, 'mais antigo'):
            updater.update()
        self.assertEqual(self.output.read_bytes(), b'invalid')

    def test_previous_day_triggers_a_download(self):
        updater.update()
        state = json.loads(self.state.read_text())
        state['date'] = '2000-01-01'
        self.state.write_text(json.dumps(state))
        updater.update()
        self.assertEqual(self.network.call_count, 2)

    def test_metadata_write_failure_retries_next_time(self):
        write = updater.atomic_write

        def fail_state(path, content):
            if path == self.state:
                raise OSError('state is read-only')
            write(path, content)

        with patch.object(updater, 'atomic_write', side_effect=fail_state):
            with self.assertRaisesRegex(OSError, 'registro diário falhou'):
                updater.update()
        self.assertEqual(updater.workbook_latest(self.output.read_bytes()), 1)
        self.assertFalse(self.state.exists())
        updater.update()
        self.assertEqual(self.network.call_count, 2)
        self.assertTrue(self.state.exists())

    def test_lock_conflict_does_not_start_another_download(self):
        with updater.update_lock(self.state.with_suffix('.lock')):
            with self.assertRaisesRegex(OSError, 'bloqueio'):
                updater.update()
        self.network.assert_not_called()


if __name__ == '__main__':
    unittest.main()
