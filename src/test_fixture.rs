//! Synthetic Dia de Sorte workbooks for tests that need a real file.
//!
//! These fixtures replace the gitignored official workbook for local checks.
//! They are compiled only for the test build and are never shipped.

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const HEADERS: [&str; 10] = [
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

fn xml_cell(column: char, row: usize, value: &str) -> String {
    format!("<c r=\"{column}{row}\" t=\"inlineStr\"><is><t>{value}</t></is></c>")
}

/// Deterministic, reproducible draw rows: strictly increasing numbers,
/// a month and a valid contest sequence, all defined for any `count`.
pub(crate) fn synthetic_rows(count: usize) -> Vec<Vec<String>> {
    let base = chrono::NaiveDate::from_ymd_opt(2018, 5, 19).expect("valid base date");
    (1..=count)
        .map(|contest| {
            let start = (((contest - 1) * 7) % 25) + 1;
            let mut row = vec![
                contest.to_string(),
                (base + chrono::Duration::days((contest - 1) as i64))
                    .format("%d/%m/%Y")
                    .to_string(),
            ];
            row.extend((0..7).map(|offset| (start + offset).to_string()));
            row.push((((contest - 1) % 12) + 1).to_string());
            row
        })
        .collect()
}

/// Write a valid inline-string workbook with `count` contests and return its path.
/// Each call writes a new file to keep tests independent.
pub(crate) fn write_synthetic_workbook(count: usize) -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let file = std::env::temp_dir().join(format!(
        "chutegpt-synthetic-{}-{}.xlsx",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let options = zip::write::SimpleFileOptions::default();
    let mut zip = zip::ZipWriter::new(File::create(&file).expect("create fixture workbook"));
    for (name, body) in [
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
    ] {
        zip.start_file(name, options).expect("add zip part");
        zip.write_all(body.as_bytes()).expect("write zip part");
    }
    let header: String = HEADERS
        .iter()
        .enumerate()
        .map(|(idx, value)| xml_cell(char::from(b'A' + idx as u8), 1, value))
        .collect();
    let mut sheet = format!(
        "<?xml version=\"1.0\"?><worksheet xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\"><sheetData><row r=\"1\">{header}</row>"
    );
    for (offset, row) in synthetic_rows(count).iter().enumerate() {
        let number = offset + 2;
        sheet.push_str(&format!("<row r=\"{number}\">"));
        for (column, value) in row.iter().enumerate() {
            sheet.push_str(&xml_cell(char::from(b'A' + column as u8), number, value));
        }
        sheet.push_str("</row>");
    }
    sheet.push_str("</sheetData></worksheet>");
    zip.start_file("xl/worksheets/sheet1.xml", options)
        .expect("add worksheet");
    zip.write_all(sheet.as_bytes()).expect("write worksheet");
    zip.finish().expect("finish fixture workbook");
    file
}

/// Convenience wrapper that writes, loads and then removes the fixture file.
pub(crate) fn with_synthetic_workbook<T>(
    count: usize,
    run: impl FnOnce(&std::path::Path) -> T,
) -> T {
    let path = write_synthetic_workbook(count);
    let result = run(&path);
    let _ = std::fs::remove_file(&path);
    result
}
