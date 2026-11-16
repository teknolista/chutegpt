use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

use calamine::{Data, DataType, Reader, Xlsx, open_workbook_from_rs};
use chrono::{Local, NaiveDate};
use sha2::{Digest, Sha256};
use zip::ZipArchive;

const MAX_WORKBOOK: u64 = 15 * 1024 * 1024;
const MAX_UNPACKED: u64 = 50 * 1024 * 1024;
const SOURCE: &str = "https://servicebus3.caixa.gov.br/portaldeloterias/api/resultados/download?modalidade=Dia%20de%20Sorte";
const STATE: &str = ".dia-de-sorte-atualizacao.json";
const REQUIRED: [&str; 10] = [
    "Concurso",
    "Data Sorteio",
    "Bola1",
    "Bola2",
    "Bola3",
    "Bola4",
    "Bola5",
    "Bola6",
    "Bola7",
    "Mês da Sorte",
];
const TIERS: [(&str, &str); 5] = [
    ("Ganhadores 7 acertos", "Rateio 7 acertos"),
    ("Ganhadores 6 acertos", "Rateio 6 acertos"),
    ("Ganhadores 5 acertos", "Rateio 5 acertos"),
    ("Ganhadores 4 acertos", "Rateio 4 acertos"),
    ("Ganhadores mês da sorte", "Rateio mês da sorte"),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prize {
    pub winners: Option<u64>,
    pub cents: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Draw {
    pub contest: u32,
    pub date: NaiveDate,
    pub numbers: [u8; 7],
    pub month: u8,
    pub prizes: [Prize; 5],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Freshness {
    Current { checked: NaiveDate },
    Stale { checked: NaiveDate },
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub path: PathBuf,
    pub sha256: String,
    pub draws: Vec<Draw>,
    pub freshness: Freshness,
    pub missing_prize_fields: [usize; 10],
}

/// Resolve the workbook selected by the user without accessing the network.
pub fn workbook_path(explicit: Option<&Path>) -> Result<PathBuf, String> {
    if let Some(path) = explicit {
        return Ok(path.to_path_buf());
    }
    #[cfg(target_os = "windows")]
    let base = std::env::var_os("LOCALAPPDATA")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from);
    #[cfg(target_os = "macos")]
    let base = std::env::var_os("HOME")
        .filter(|v| !v.is_empty())
        .map(|v| PathBuf::from(v).join("Library/Application Support"));
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    let base = std::env::var_os("XDG_DATA_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .filter(|v| v.is_absolute())
        .or_else(|| {
            std::env::var_os("HOME")
                .filter(|v| !v.is_empty())
                .map(|v| PathBuf::from(v).join(".local/share"))
        });
    base.map(|v| v.join("ChuteGPT").join("Dia de Sorte-atual.xlsx"))
        .ok_or_else(|| "Diretório de dados indisponível; informe --planilha CAMINHO".into())
}

fn read_limited(path: &Path, max: u64) -> Result<Vec<u8>, String> {
    let file =
        File::open(path).map_err(|e| format!("Não foi possível abrir {}: {e}", path.display()))?;
    let mut bytes = Vec::new();
    file.take(max + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| format!("Não foi possível ler {}: {e}", path.display()))?;
    if bytes.len() as u64 > max {
        return Err(format!(
            "Arquivo excede o limite permitido: {}",
            path.display()
        ));
    }
    Ok(bytes)
}

fn check_zip(bytes: &[u8]) -> Result<(), String> {
    let mut archive =
        ZipArchive::new(Cursor::new(bytes)).map_err(|e| format!("Planilha Excel inválida: {e}"))?;
    let mut names = HashSet::new();
    let mut total = 0_u64;
    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| format!("Entrada ZIP inválida: {e}"))?;
        let name = entry
            .name()
            .map_err(|e| format!("Nome ZIP inválido: {e}"))?
            .into_owned();
        if entry.enclosed_name().is_none() || !names.insert(name) {
            return Err("Planilha contém caminho ZIP inseguro ou entrada duplicada".into());
        }
        total = total
            .checked_add(entry.size())
            .ok_or("Tamanho ZIP excedeu o limite")?;
        if total > MAX_UNPACKED {
            return Err("Planilha excede o limite de descompactação".into());
        }
        std::io::copy(&mut entry, &mut std::io::sink())
            .map_err(|e| format!("Planilha corrompida: {e}"))?;
    }
    Ok(())
}

fn integer(cell: &Data) -> Option<u64> {
    match cell {
        Data::Int(v) => u64::try_from(*v).ok(),
        Data::Float(v)
            if v.is_finite() && *v >= 0.0 && v.fract() == 0.0 && *v < u64::MAX as f64 =>
        {
            Some(*v as u64)
        }
        Data::String(v) => v.trim().parse().ok(),
        _ => None,
    }
}

fn cents(cell: &Data) -> Option<u64> {
    match cell {
        Data::Int(v) => u64::try_from(*v).ok()?.checked_mul(100),
        Data::Float(v) if v.is_finite() && *v >= 0.0 && *v * 100.0 < u64::MAX as f64 => {
            let scaled = (*v * 100.0).round();
            if scaled / 100.0 == *v {
                Some(scaled as u64)
            } else {
                None
            }
        }
        Data::String(text) => {
            let text = text.trim().strip_prefix("R$").unwrap_or(text.trim()).trim();
            let (whole, fractional) = text.split_once(',').unwrap_or((text, ""));
            if fractional.len() > 2 || !fractional.chars().all(|c| c.is_ascii_digit()) {
                return None;
            }
            let groups: Vec<_> = whole.split('.').collect();
            if groups[0].is_empty()
                || (groups.len() > 1 && groups[0].len() > 3)
                || groups
                    .iter()
                    .any(|g| !g.chars().all(|c| c.is_ascii_digit()))
                || groups.iter().skip(1).any(|g| g.len() != 3)
            {
                return None;
            }
            let whole: String = groups.concat();
            let whole: u64 = whole.parse().ok()?;
            let fraction = match fractional.len() {
                0 => 0,
                1 => fractional.parse::<u64>().ok()?.checked_mul(10)?,
                _ => fractional.parse().ok()?,
            };
            whole.checked_mul(100)?.checked_add(fraction)
        }
        _ => None,
    }
}

fn decode_month_entities(input: &str) -> String {
    let mut remaining = input;
    let mut output = String::new();
    while let Some(start) = remaining.find('&') {
        output.push_str(&remaining[..start]);
        let entity = &remaining[start + 1..];
        let Some(end) = entity.find(';') else {
            output.push_str(&remaining[start..]);
            return output;
        };
        let name = &entity[..end];
        let decoded = match name {
            "ccedil" => Some('ç'),
            name if name.starts_with("#x") => u32::from_str_radix(&name[2..], 16)
                .ok()
                .and_then(char::from_u32),
            name if name.starts_with('#') => name[1..].parse::<u32>().ok().and_then(char::from_u32),
            _ => None,
        };
        if let Some(value) = decoded {
            output.push(value);
        } else {
            output.push_str(&remaining[start..start + end + 2]);
        }
        remaining = &entity[end + 1..];
    }
    output.push_str(remaining);
    output
}

fn month(cell: &Data) -> Option<u8> {
    if let Some(v) = integer(cell) {
        return u8::try_from(v).ok().filter(|v| (1..=12).contains(v));
    }
    let Data::String(value) = cell else {
        return None;
    };
    let normalized = decode_month_entities(&value.trim().to_lowercase());
    match normalized.as_str() {
        "janeiro" => Some(1),
        "fevereiro" => Some(2),
        "março" | "marco" => Some(3),
        "abril" => Some(4),
        "maio" => Some(5),
        "junho" => Some(6),
        "julho" => Some(7),
        "agosto" => Some(8),
        "setembro" => Some(9),
        "outubro" => Some(10),
        "novembro" => Some(11),
        "dezembro" => Some(12),
        _ => None,
    }
}

fn date(cell: &Data) -> Option<NaiveDate> {
    match cell {
        Data::String(value) => NaiveDate::parse_from_str(value.trim(), "%d/%m/%Y").ok(),
        Data::DateTime(_) => cell.as_date(),
        Data::DateTimeIso(value) => NaiveDate::parse_from_str(value, "%Y-%m-%d").ok(),
        _ => None,
    }
}

fn freshness(raw: Option<&[u8]>, hash: &str, latest: u32, today: NaiveDate) -> Freshness {
    let Some(value) = raw.and_then(|raw| serde_json::from_slice::<serde_json::Value>(raw).ok())
    else {
        return Freshness::Unknown;
    };
    if value.get("version").and_then(|v| v.as_u64()) != Some(2)
        || value.get("source").and_then(|v| v.as_str()) != Some(SOURCE)
        || value.get("sha256").and_then(|v| v.as_str()) != Some(hash)
        || value.get("latest").and_then(|v| v.as_u64()) != Some(u64::from(latest))
    {
        return Freshness::Unknown;
    }
    let Some(checked) = value
        .get("date")
        .and_then(|v| v.as_str())
        .and_then(|v| NaiveDate::parse_from_str(v, "%Y-%m-%d").ok())
    else {
        return Freshness::Unknown;
    };
    if checked > today {
        Freshness::Unknown
    } else if checked == today {
        Freshness::Current { checked }
    } else {
        Freshness::Stale { checked }
    }
}

pub fn load(path: &Path) -> Result<Snapshot, String> {
    let bytes = read_limited(path, MAX_WORKBOOK)?;
    let hash = format!("{:x}", Sha256::digest(&bytes));
    check_zip(&bytes)?;
    let mut workbook: Xlsx<_> = open_workbook_from_rs(Cursor::new(&bytes))
        .map_err(|e| format!("Planilha Excel inválida: {e}"))?;
    let name = workbook
        .sheet_names()
        .first()
        .cloned()
        .ok_or("Planilha não possui abas")?;
    // Check coordinates before Calamine materializes a dense range.
    {
        let mut reader = workbook
            .worksheet_cells_reader(&name)
            .map_err(|e| format!("Não foi possível ler os resultados: {e}"))?;
        while let Some(cell) = reader
            .next_cell()
            .map_err(|e| format!("Célula inválida: {e}"))?
        {
            let (row, col) = cell.get_position();
            if row >= 100_000 || col >= 64 {
                return Err("Planilha possui dimensões acima do limite permitido".into());
            }
        }
    }
    {
        let mut reader = workbook
            .worksheet_cells_reader(&name)
            .map_err(|e| format!("Não foi possível ler fórmulas: {e}"))?;
        while let Some(cell) = reader
            .next_formula()
            .map_err(|e| format!("Fórmula inválida: {e}"))?
        {
            let (row, col) = cell.get_position();
            if row >= 100_000 || col >= 64 {
                return Err("Planilha possui dimensões acima do limite permitido".into());
            }
        }
    }
    let range = workbook
        .worksheet_range(&name)
        .map_err(|e| format!("Não foi possível ler os resultados: {e}"))?;
    let formulas = workbook
        .worksheet_formula(&name)
        .map_err(|e| format!("Não foi possível conferir fórmulas: {e}"))?;
    let start = range.start().ok_or("Planilha sem resultados")?;
    let mut rows = range.rows();
    let header = rows.next().ok_or("Planilha sem cabeçalho")?;
    let mut columns: HashMap<&str, usize> = HashMap::new();
    for (idx, cell) in header.iter().enumerate() {
        if let Data::String(name) = cell {
            let name = name.as_str();
            if (REQUIRED.contains(&name) || TIERS.iter().any(|(a, b)| *a == name || *b == name))
                && columns.insert(name, idx).is_some()
            {
                return Err(format!("Coluna duplicada: {name}"));
            }
        }
    }
    for name in REQUIRED {
        if !columns.contains_key(name) {
            return Err(format!("Coluna obrigatória ausente: {name}"));
        }
    }
    let required: Vec<usize> = REQUIRED.iter().map(|name| columns[name]).collect();
    let mut ordered = BTreeMap::new();
    let mut missing = [0_usize; 10];
    let mut empty_row = false;
    for (offset, row) in rows.enumerate() {
        if row.iter().all(DataType::is_empty) {
            empty_row = true;
            continue;
        }
        if empty_row {
            return Err("Linha vazia no interior do histórico".into());
        }
        let row_number = start.0 + 1 + offset as u32;
        let cell = |idx: usize| row.get(idx).unwrap_or(&Data::Empty);
        if required.iter().any(|idx| {
            formulas
                .get_value((row_number, start.1 + *idx as u32))
                .is_some_and(|f| !f.is_empty())
        }) {
            return Err(format!(
                "Fórmula em campo obrigatório na linha {}",
                row_number + 1
            ));
        }
        let invalid = || format!("Dados obrigatórios inválidos na linha {}", row_number + 1);
        let contest = integer(cell(required[0]))
            .and_then(|n| u32::try_from(n).ok())
            .filter(|n| *n > 0)
            .ok_or_else(invalid)?;
        let date = date(cell(required[1])).ok_or_else(invalid)?;
        let mut numbers = [0_u8; 7];
        for (i, n) in numbers.iter_mut().enumerate() {
            *n = integer(cell(required[i + 2]))
                .and_then(|v| u8::try_from(v).ok())
                .filter(|v| (1..=31).contains(v))
                .ok_or_else(invalid)?;
        }
        numbers.sort_unstable();
        if numbers.windows(2).any(|v| v[0] == v[1]) {
            return Err(format!("Dezena duplicada no concurso {contest}"));
        }
        let month = month(cell(required[9])).ok_or_else(invalid)?;
        let prizes = std::array::from_fn(|i| {
            let (w, p) = TIERS[i];
            let winners = columns.get(w).and_then(|idx| integer(cell(*idx)));
            let cents = columns.get(p).and_then(|idx| cents(cell(*idx)));
            if winners.is_none() {
                missing[2 * i] += 1;
            }
            if cents.is_none() {
                missing[2 * i + 1] += 1;
            }
            Prize { winners, cents }
        });
        if ordered
            .insert(
                contest,
                Draw {
                    contest,
                    date,
                    numbers,
                    month,
                    prizes,
                },
            )
            .is_some()
        {
            return Err(format!("Concurso duplicado: {contest}"));
        }
    }
    if ordered.is_empty() {
        return Err("Planilha sem resultados".into());
    }
    let draws: Vec<_> = ordered.into_values().collect();
    for (idx, draw) in draws.iter().enumerate() {
        if usize::try_from(draw.contest).ok() != Some(idx + 1) {
            return Err(format!("Concurso ausente antes de {}", draw.contest));
        }
        if idx > 0 && draws[idx - 1].date > draw.date {
            return Err(format!("Datas fora de ordem no concurso {}", draw.contest));
        }
    }
    let latest = draws.last().ok_or("Planilha sem resultados")?.contest;
    let raw = path
        .parent()
        .and_then(|p| read_limited(&p.join(STATE), 64 * 1024).ok());
    let freshness = freshness(raw.as_deref(), &hash, latest, Local::now().date_naive());
    Ok(Snapshot {
        path: path.to_owned(),
        sha256: hash,
        draws,
        freshness,
        missing_prize_fields: missing,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_amounts_and_months_without_conflating_missing_with_zero() {
        assert_eq!(
            cents(&Data::String("R$957.949,23".into())),
            Some(95_794_923)
        );
        assert_eq!(cents(&Data::String("R$0,00".into())), Some(0));
        assert_eq!(cents(&Data::String("R$1,234".into())), None);
        assert_eq!(cents(&Data::String("R$1..2,00".into())), None);
        assert_eq!(cents(&Data::String("R$12.34,00".into())), None);
        assert_eq!(
            workbook_path(Some(Path::new("outro.xlsx"))).unwrap(),
            PathBuf::from("outro.xlsx")
        );
        assert_eq!(cents(&Data::Empty), None);
        assert_eq!(month(&Data::String("Mar&ccedil;o".into())), Some(3));
        assert_eq!(month(&Data::String("Mar&#xE7;o".into())), Some(3));
        assert_eq!(month(&Data::String("Mar&#231;o".into())), Some(3));
        assert_eq!(month(&Data::String("13".into())), None);
    }

    #[test]
    fn sidecar_must_bind_contents_source_latest_and_local_date() {
        let today = NaiveDate::from_ymd_opt(2026, 11, 16).unwrap();
        let state = serde_json::json!({"version":2,"source":SOURCE,"sha256":"abc","latest":2,"date":"2026-11-16"});
        let bytes = serde_json::to_vec(&state).unwrap();
        assert!(matches!(
            freshness(Some(&bytes), "abc", 2, today),
            Freshness::Current { .. }
        ));
        assert_eq!(
            freshness(Some(&bytes), "different", 2, today),
            Freshness::Unknown
        );
        assert_eq!(freshness(Some(&bytes), "abc", 3, today), Freshness::Unknown);
        let next = NaiveDate::from_ymd_opt(2026, 11, 17).unwrap();
        assert!(matches!(
            freshness(Some(&bytes), "abc", 2, next),
            Freshness::Stale { .. }
        ));
        assert_eq!(
            freshness(
                Some(&bytes),
                "abc",
                2,
                NaiveDate::from_ymd_opt(2026, 11, 15).unwrap()
            ),
            Freshness::Unknown
        );
        assert_eq!(freshness(Some(b"{"), "abc", 2, today), Freshness::Unknown);
    }

    fn fixture(row_values: &[&str], omitted_header: Option<usize>) -> PathBuf {
        use std::io::Write;
        use std::sync::atomic::{AtomicUsize, Ordering};
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let file = std::env::temp_dir().join(format!(
            "chutegpt-fixture-{}-{}.xlsx",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let mut zip = zip::ZipWriter::new(File::create(&file).unwrap());
        let options = zip::write::SimpleFileOptions::default();
        let parts = [
            (
                "[Content_Types].xml",
                r#"<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/><Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/></Types>"#,
            ),
            (
                "_rels/.rels",
                r#"<?xml version="1.0"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>"#,
            ),
            (
                "xl/workbook.xml",
                r#"<?xml version="1.0"?><workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Resultados" sheetId="1" r:id="rId1"/></sheets></workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<?xml version="1.0"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/></Relationships>"#,
            ),
        ];
        for (name, body) in parts {
            zip.start_file(name, options).unwrap();
            zip.write_all(body.as_bytes()).unwrap();
        }
        let headers: String = REQUIRED
            .iter()
            .enumerate()
            .filter(|(i, _)| Some(*i) != omitted_header)
            .map(|(i, v)| {
                format!(
                    "<c r=\"{}1\" t=\"inlineStr\"><is><t>{v}</t></is></c>",
                    char::from(b'A' + i as u8)
                )
            })
            .collect();
        let cells: String = row_values
            .iter()
            .enumerate()
            .map(|(i, v)| {
                format!(
                    "<c r=\"{}2\" t=\"inlineStr\"><is><t>{v}</t></is></c>",
                    char::from(b'A' + i as u8)
                )
            })
            .collect();
        let sheet = format!(
            "<?xml version=\"1.0\"?><worksheet xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\"><sheetData><row r=\"1\">{headers}</row><row r=\"2\">{cells}</row></sheetData></worksheet>"
        );
        zip.start_file("xl/worksheets/sheet1.xml", options).unwrap();
        zip.write_all(sheet.as_bytes()).unwrap();
        zip.finish().unwrap();
        file
    }

    #[test]
    fn synthetic_workbook_validates_draws_and_optional_columns() {
        let valid = [
            "1",
            "19/05/2018",
            "1",
            "2",
            "3",
            "4",
            "5",
            "6",
            "7",
            "março",
        ];
        let path = fixture(&valid, None);
        let data = load(&path).unwrap();
        assert_eq!(data.draws[0].numbers, [1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(data.draws[0].month, 3);
        assert_eq!(data.draws[0].prizes[0].winners, None);
        assert_eq!(data.missing_prize_fields, [1; 10]);
        assert_eq!(data.freshness, Freshness::Unknown);
        std::fs::remove_file(path).unwrap();
        for (row, missing_header) in [
            (
                vec!["1", "19/05/2018", "1", "1", "3", "4", "5", "6", "7", "3"],
                None,
            ),
            (
                vec!["2", "19/05/2018", "1", "2", "3", "4", "5", "6", "7", "3"],
                None,
            ),
            (
                vec!["1", "99/99/2018", "1", "2", "3", "4", "5", "6", "7", "3"],
                None,
            ),
            (valid.to_vec(), Some(2)),
        ] {
            let path = fixture(&row, missing_header);
            assert!(load(&path).is_err());
            std::fs::remove_file(path).unwrap();
        }
    }

    #[test]
    fn current_official_workbook_loads_and_corrupt_input_fails() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let path = root.join("docs/Dia de Sorte-atual.xlsx");
        assert!(check_zip(b"not an xlsx").is_err());
        assert!(load(&root.join("not-found.xlsx")).is_err());
        if !path.exists() {
            eprintln!("Planilha oficial local ausente; teste da cópia real não executado");
            return;
        }
        let snapshot = load(&path).unwrap();
        assert!(snapshot.draws.len() > 1000);
        assert_eq!(snapshot.draws[0].contest, 1);
        assert_eq!(snapshot.draws[0].numbers, [3, 5, 8, 9, 19, 21, 30]);
        assert_eq!(snapshot.draws[0].prizes[0].winners, Some(1));
        assert_eq!(snapshot.draws[0].prizes[0].cents, Some(95_794_923));
        assert!(check_zip(b"not an xlsx").is_err());
        assert!(load(&root.join("not-found.xlsx")).is_err());
    }
}
