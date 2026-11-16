use std::path::{Path, PathBuf};
fn main() {
    if let Err(error) = run() {
        eprintln!("Erro: {error}");
        std::process::exit(1);
    }
}
fn run() -> Result<(), String> {
    let mut args = std::env::args_os().skip(1);
    let mut selected: Option<PathBuf> = None;
    while let Some(arg) = args.next() {
        if arg == "--help" || arg == "-h" {
            println!(
                "ChuteGPT — análise histórica do Dia de Sorte\nUso: chutegpt [--planilha CAMINHO]"
            );
            return Ok(());
        }
        if arg == "--planilha" {
            if selected.is_some() {
                return Err("Informe --planilha apenas uma vez.".into());
            }
            selected = Some(PathBuf::from(
                args.next().ok_or("Informe o caminho após --planilha.")?,
            ));
        } else {
            return Err(format!("Argumento desconhecido: {}", arg.to_string_lossy()));
        }
    }
    let path = chutegpt::data::workbook_path(selected.as_deref().map(Path::new))?;
    chutegpt::ui::run(&path)
}
