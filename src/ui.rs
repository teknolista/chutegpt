use crate::{
    data::{self, Freshness, Snapshot},
    presentation::{self, STRATEGIES, Section, View},
    series,
};
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{BarChart, Block, Borders, Gauge, List, ListItem, ListState, Paragraph, Wrap},
};
use std::{
    io::{self, IsTerminal, Stdout},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver},
    },
    thread,
    time::Duration,
};

enum Message {
    Preflight(Result<Snapshot, String>),
    Loaded(Result<Snapshot, String>),
    Progress(usize, usize),
    Done(Result<View, String>),
}
#[derive(PartialEq, Eq)]
enum Screen {
    Menu,
    Loading,
    Confirm,
    Working,
    Result,
    Series,
    Error,
}
struct App {
    path: PathBuf,
    menu: usize,
    menu_scroll: usize,
    screen: Screen,
    snapshot: Option<Arc<Snapshot>>,
    /// Outcome of the background source check: `None` while pending, `Err` when
    /// the source was read and rejected. A rejected source must never be
    /// reported as "not yet verified".
    verify: Option<Result<(), String>>,
    view: Option<View>,
    section: usize,
    category: usize,
    scroll: usize,
    details: Vec<String>,
    error: String,
    progress: (usize, usize),
    tick: usize,
    terminal_width: u16,
    terminal_height: u16,
    receiver: Option<Receiver<Message>>,
    cancel: Arc<AtomicBool>,
}
impl App {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            menu: 0,
            menu_scroll: 0,
            screen: Screen::Menu,
            snapshot: None,
            verify: None,
            view: None,
            section: 0,
            category: 0,
            scroll: 0,
            details: Vec::new(),
            error: String::new(),
            progress: (0, 0),
            tick: 0,
            terminal_width: 100,
            terminal_height: 24,
            receiver: None,
            cancel: Arc::new(AtomicBool::new(false)),
        }
    }
    fn stop(&mut self) {
        self.cancel.store(true, Ordering::Relaxed);
        self.receiver = None;
        self.screen = Screen::Menu;
        self.progress = (0, 0);
    }
    fn preflight(&mut self) {
        let path = self.path.clone();
        let (tx, rx) = mpsc::sync_channel(1);
        thread::spawn(move || {
            let _ = tx.send(Message::Preflight(data::load(&path)));
        });
        self.receiver = Some(rx);
    }
    fn load(&mut self) {
        self.cancel = Arc::new(AtomicBool::new(false));
        let path = self.path.clone();
        let (tx, rx) = mpsc::sync_channel(2);
        thread::spawn(move || {
            let _ = tx.send(Message::Loaded(data::load(&path)));
        });
        self.receiver = Some(rx);
        self.screen = Screen::Loading;
    }
    fn calculate(&mut self) {
        let Some(snapshot) = self.snapshot.clone() else {
            self.error = "Nenhuma planilha válida selecionada".into();
            self.screen = Screen::Error;
            return;
        };
        self.cancel = Arc::new(AtomicBool::new(false));
        let cancel = self.cancel.clone();
        let index = self.menu;
        let (tx, rx) = mpsc::sync_channel(4);
        thread::spawn(move || {
            let callback = |done, total| {
                let _ = tx.try_send(Message::Progress(done, total));
                !cancel.load(Ordering::Relaxed)
            };
            let result = presentation::build(index, &snapshot.draws, callback);
            let _ = tx.send(Message::Done(result));
        });
        self.progress = (0, 0);
        self.receiver = Some(rx);
        self.screen = Screen::Working;
    }
    fn poll(&mut self) {
        // Bound work per frame so a busy progress producer cannot starve redraws or input.
        for _ in 0..8 {
            let message = match self.receiver.as_ref().map(Receiver::try_recv) {
                Some(Ok(message)) => message,
                Some(Err(mpsc::TryRecvError::Disconnected)) => {
                    self.error = "O processamento terminou inesperadamente.".into();
                    self.receiver = None;
                    self.screen = Screen::Error;
                    break;
                }
                Some(Err(mpsc::TryRecvError::Empty)) | None => break,
            };
            match message {
                Message::Preflight(Ok(snapshot)) => {
                    self.snapshot = Some(Arc::new(snapshot));
                    self.verify = Some(Ok(()));
                    self.receiver = None;
                }
                Message::Preflight(Err(error)) => {
                    self.snapshot = None;
                    self.verify = Some(Err(error));
                    self.receiver = None;
                }
                Message::Loaded(Ok(snapshot)) => {
                    let current = matches!(snapshot.freshness, Freshness::Current { .. });
                    self.snapshot = Some(Arc::new(snapshot));
                    self.verify = Some(Ok(()));
                    self.receiver = None;
                    if current {
                        self.calculate();
                    } else {
                        self.screen = Screen::Confirm;
                    }
                }
                Message::Loaded(Err(error)) => {
                    self.snapshot = None;
                    self.verify = Some(Err(error.clone()));
                    self.error = error;
                    self.receiver = None;
                    self.screen = Screen::Error;
                }
                Message::Done(Err(error)) => {
                    self.error = error;
                    self.receiver = None;
                    self.screen = Screen::Error;
                }
                Message::Progress(done, total) => self.progress = (done, total),
                Message::Done(Ok(view)) => {
                    self.view = Some(view);
                    self.section = 0;
                    self.category = 0;
                    self.scroll = 0;
                    self.receiver = None;
                    self.screen = Screen::Result;
                }
            }
        }
    }
    fn active_section(&self) -> Option<&Section> {
        self.view.as_ref()?.sections.get(self.section)
    }
    fn category_count(&self) -> usize {
        self.active_section().map_or(0, |s| s.bins.len())
    }
    fn series(&mut self) {
        let Some(sec) = self.active_section() else {
            return;
        };
        let Some(values) = sec.values.as_ref() else {
            self.error = "Detalhe de série não aplicável a esta tabela".into();
            return;
        };
        let Some((&category, _)) = sec.bins.iter().nth(self.category) else {
            self.error = "Não há categoria para analisar".into();
            return;
        };
        let (values, excluded) = series_population(self.menu, self.section, values);
        match series::category(values, category, |_, _| true) {
            Ok(result) => {
                let display = presentation::format_value(category, sec.scale);
                let mut lines = vec![format!("Categoria {display} — {}", sec.title)];
                if excluded > 0 {
                    lines.push(format!(
                        "Posições iniciais sem estatística definida, fora da população da série: {excluded}"
                    ));
                }
                lines.extend([
                    format!(
                        "Eventos: {}; desconhecidos: {}; periodicidades indisponíveis: {}",
                        result.events, result.unknown, result.missing_periodicities
                    ),
                    format!("Periodicidades: {:?}", result.periodicities),
                    format!(
                        "Intervalos concluídos: {:?}; indisponíveis: {}",
                        result.closed_waits, result.missing_waits
                    ),
                    format!(
                        "Atraso aberto (censurado): {}",
                        result
                            .open_delay
                            .map_or_else(|| "indisponível".into(), |v| v.to_string())
                    ),
                ]);
                for (title, s) in [
                    ("Periodicidades", &result.periodicity_stats),
                    ("Intervalos concluídos", &result.wait_stats),
                ] {
                    lines.push(format!("--- {title} (n={}) ---", s.count));
                    macro_rules! show {
                        ($label:expr,$v:expr) => {
                            show!($label, $v, "amostra vazia ou resultado não finito")
                        };
                        ($label:expr,$v:expr,$why:expr) => {
                            lines.push(format!(
                                "{}: {}",
                                $label,
                                $v.map_or_else(
                                    || format!("indisponível ({})", $why),
                                    |v| format!("{v:.2}")
                                )
                            ))
                        };
                    }
                    show!("Mínimo", s.minimum);
                    show!("Máximo", s.maximum);
                    show!("Amplitude", s.range);
                    lines.push(if s.modes.is_empty() {
                        "Moda(s): indisponível (amostra vazia)".into()
                    } else {
                        format!("Moda(s): {:?}", s.modes)
                    });
                    show!("Média", s.mean);
                    show!("Média geométrica", s.geometric_mean);
                    show!("Média harmônica", s.harmonic_mean, "exige amostra positiva");
                    show!(
                        "Média circular (radianos)",
                        s.circular_mean_radians,
                        "amostra vazia ou resultante indeterminada"
                    );
                    show!("Mediana empírica", s.median);
                    show!("Média das raízes", s.mean_roots);
                    show!("RMS", s.rms);
                    show!("Desvio absoluto médio", s.mad);
                    show!("Variância populacional", s.population_variance);
                    show!("Desvio padrão populacional", s.population_sd);
                    show!(
                        "Variância amostral",
                        s.sample_variance,
                        "exige pelo menos duas observações"
                    );
                    show!(
                        "Desvio padrão amostral",
                        s.sample_sd,
                        "exige pelo menos duas observações"
                    );
                    show!(
                        "Coeficiente de variação (%)",
                        s.cv_percent,
                        "média zero ou amostra vazia"
                    );
                    show!("Assimetria", s.skewness, "variância zero ou amostra vazia");
                    show!(
                        "Excesso de curtose",
                        s.excess_kurtosis,
                        "variância zero ou amostra vazia"
                    );
                }
                lines.push("Média circular: transformação matemática em radianos, sem interpretação preditiva.".into());
                self.details = lines;
                self.scroll = 0;
                self.error.clear();
                self.screen = Screen::Series;
            }
            Err(e) => self.error = format!("Série indisponível: {e:?}"),
        }
    }
    fn key(&mut self, key: KeyCode) -> bool {
        match self.screen {
            Screen::Menu => match key {
                KeyCode::Char('q') | KeyCode::Esc => return true,
                KeyCode::Down | KeyCode::Char('j') => {
                    self.menu = (self.menu + 1) % STRATEGIES.len();
                    self.menu_scroll = 0
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.menu = (self.menu + STRATEGIES.len() - 1) % STRATEGIES.len();
                    self.menu_scroll = 0
                }
                KeyCode::PageDown | KeyCode::Char('l') => {
                    let width = (self.terminal_width as usize * 70 / 100).saturating_sub(3) as u16;
                    let lines = Paragraph::new(menu_explanation(self))
                        .wrap(Wrap { trim: false })
                        .line_count(width.max(1));
                    self.menu_scroll = self
                        .menu_scroll
                        .saturating_add(1)
                        .min(lines.saturating_sub(self.terminal_height.saturating_sub(8) as usize));
                }
                KeyCode::PageUp | KeyCode::Char('h') => {
                    self.menu_scroll = self.menu_scroll.saturating_sub(1)
                }
                KeyCode::Enter => self.load(),
                _ => {}
            },
            Screen::Loading | Screen::Working => {
                if matches!(key, KeyCode::Esc | KeyCode::Char('c')) {
                    self.stop();
                }
            }
            Screen::Confirm => match key {
                KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Enter => self.calculate(),
                KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => self.stop(),
                _ => {}
            },
            Screen::Error => {
                if matches!(key, KeyCode::Esc | KeyCode::Enter) {
                    self.stop();
                }
            }
            Screen::Series => match key {
                KeyCode::Esc | KeyCode::Backspace => {
                    self.scroll = 0;
                    self.screen = Screen::Result;
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let content = self.details.join("\n");
                    let width = self.terminal_width.saturating_sub(2);
                    let height = self.terminal_height.saturating_sub(8) as usize;
                    let lines = Paragraph::new(content)
                        .wrap(Wrap { trim: false })
                        .line_count(width);
                    let maximum = lines.saturating_sub(height);
                    self.scroll = self
                        .scroll
                        .saturating_add(1)
                        .min(maximum)
                        .min(u16::MAX as usize)
                }
                KeyCode::Up | KeyCode::Char('k') => self.scroll = self.scroll.saturating_sub(1),
                _ => {}
            },
            Screen::Result => match key {
                KeyCode::Esc | KeyCode::Backspace => self.stop(),
                KeyCode::Char('q') => return true,
                KeyCode::Tab | KeyCode::Char(']') => {
                    if let Some(v) = &self.view {
                        self.section = (self.section + 1) % v.sections.len();
                        self.category = 0;
                        self.scroll = 0;
                        self.error.clear();
                    }
                }
                KeyCode::BackTab | KeyCode::Char('[') => {
                    if let Some(v) = &self.view {
                        self.section = (self.section + v.sections.len() - 1) % v.sections.len();
                        self.category = 0;
                        self.scroll = 0;
                        self.error.clear();
                    }
                }
                KeyCode::Right => {
                    self.category = (self.category + 1).min(self.category_count().saturating_sub(1))
                }
                KeyCode::Left => self.category = self.category.saturating_sub(1),
                KeyCode::Down | KeyCode::Char('j') => {
                    if let Some(sec) = self.active_section() {
                        let content = result_lines(self, sec).join("\n");
                        let width = self.terminal_width.saturating_sub(2).max(1);
                        let height = self.terminal_height.saturating_sub(17) as usize;
                        let total = Paragraph::new(content)
                            .wrap(Wrap { trim: false })
                            .line_count(width);
                        self.scroll = self
                            .scroll
                            .saturating_add(1)
                            .min(total.saturating_sub(height));
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => self.scroll = self.scroll.saturating_sub(1),
                KeyCode::Char('s') | KeyCode::Enter => self.series(),
                _ => {}
            },
        }
        false
    }
}
/// Select the population used by a category series. Returns the population and
/// how many leading positions were excluded because their statistic is
/// undefined, not because an observation is missing.
fn series_population(
    menu: usize,
    section: usize,
    values: &[Option<i64>],
) -> (&[Option<i64>], usize) {
    if matches!(menu, 1 | 2) || (menu == 13 && section == 1) {
        (values.get(1..).unwrap_or_default(), values.len().min(1))
    } else if matches!(menu, 16 | 19) {
        let first = values
            .iter()
            .position(Option::is_some)
            .unwrap_or(values.len());
        (&values[first..], first)
    } else {
        (values, 0)
    }
}
fn category_label(section: &Section, value: i64, menu: usize) -> String {
    if menu == 20 && section.title.starts_with("Premiação: ") {
        return ["Seg", "Ter", "Qua", "Qui", "Sex", "Sáb", "Dom"]
            .get(value as usize)
            .map_or_else(|| value.to_string(), |s| (*s).to_string());
    }
    if menu == 15 && section.title == "Ocorrências e atrasos atuais" && (1..=12).contains(&value) {
        return presentation::MONTHS[(value - 1) as usize].to_string();
    }
    presentation::format_value(value, section.scale)
}
fn snapshot_line(s: &Snapshot) -> String {
    let first = s.draws.first().unwrap();
    let last = s.draws.last().unwrap();
    format!(
        "Concursos {}–{} | {} a {} | {} sorteios",
        first.contest,
        last.contest,
        first.date.format("%d/%m/%Y"),
        last.date.format("%d/%m/%Y"),
        s.draws.len()
    )
}
const SECTION_EXPLANATIONS: [&str; 22] = [
    "A seção apresenta a quantidade de concursos para cada número de pares e seu complemento de ímpares.",
    "A seção distribui as quantidades repetidas; o primeiro concurso não tem antecessor e fica indisponível.",
    "A seção distribui a maior repetição observada em qualquer concurso anterior, sem usar resultados futuros.",
    "A seção distribui quantos pares vizinhos consecutivos existem em cada concurso.",
    "A seção distribui o tamanho, em ligações, da maior sequência de dezenas consecutivas.",
    "A seção agrupa os concursos pelo valor da soma das sete dezenas.",
    "A seção agrupa os concursos pela raiz digital calculada a partir da soma.",
    "As seções apresentam amplitude, média, média das raízes, RMS, desvio absoluto médio, desvio padrão populacional e coeficiente de variação das dezenas.",
    "As seções apresentam as mesmas sete medidas aplicadas às diferenças entre dezenas vizinhas e uma distribuição adicional dos espaçamentos agrupados.",
    "As seções distinguem os totais por grupo, cada faixa de dezenas e a ocupação máxima de uma faixa em cada concurso.",
    "As seções distinguem os totais por grupo, cada final de 0 a 9 e a ocupação máxima de uma coluna em cada concurso.",
    "As seções mostram separadamente o máximo por linha, o máximo por coluna e a soma desses dois máximos.",
    "As duas seções mostram separadamente as durações dos ciclos de 27 e 31 dezenas, incluindo a informação de ciclos ainda abertos.",
    "A primeira seção agrupa a posição ordinal em dez faixas; a segunda agrupa as diferenças entre posições de concursos sucessivos.",
    "A primeira seção resume ocorrências e atrasos atuais; as demais apresentam uma série de presença ou ausência para cada dezena.",
    "A primeira seção resume ocorrências e atrasos dos meses; as demais apresentam a série de presença ou ausência de cada mês.",
    "As primeiras seções mostram posições no topo e limites de periodicidade e atraso. Outras comparam medidas de todas as dezenas e das sorteadas; a última detalha os concursos.",
    "A seção distribui a distância entre a menor e a maior dezena de cada concurso.",
    "A seção distribui a média arredondada das raízes quadradas das sete dezenas em cada concurso.",
    "A seção distribui a medida combinada dos desvios de frequência e atraso; concursos sem histórico suficiente ficam indisponíveis.",
    "Para cada faixa de premiação há uma seção por dia da semana com ganhadores e rateios e outra com a série de concursos premiados ou não.",
    "Para cada mês há seções de ocorrência e de intervalos concluídos; duas seções equivalentes analisam a premiação principal e seus dados ausentes.",
];
fn explanation(index: usize) -> String {
    format!(
        "{}\n\n{}",
        presentation::DEFINITIONS[index],
        SECTION_EXPLANATIONS[index]
    )
}
fn menu_explanation(app: &App) -> String {
    let mut text = format!(
        "{}\n\n{}\n\nFonte: {}",
        STRATEGIES[app.menu],
        explanation(app.menu),
        app.path.display()
    );
    if let Some(Err(error)) = &app.verify {
        text.push_str(&format!(
            "\n\nA verificação desta fonte falhou: {error}\nO arquivo precisa existir e ser uma planilha XLSX válida do Dia de Sorte. Pressione Enter para tentar novamente."
        ));
    }
    text
}
fn freshness_status(app: &App) -> String {
    if let Some(Err(_)) = app.verify {
        return "falha na verificação".into();
    }
    match app.snapshot.as_ref().map(|s| &s.freshness) {
        Some(Freshness::Current { checked }) if *checked == chrono::Local::now().date_naive() => {
            format!("Verificada hoje ({})", checked.format("%d/%m/%Y"))
        }
        Some(Freshness::Current { checked }) => format!(
            "possivelmente desatualizada ({})",
            checked.format("%d/%m/%Y")
        ),
        Some(Freshness::Stale { checked }) => format!(
            "possivelmente desatualizada ({})",
            checked.format("%d/%m/%Y")
        ),
        Some(Freshness::Unknown) => "atualização desconhecida".into(),
        None => "ainda não verificada".into(),
    }
}
fn menu_header(app: &App) -> String {
    format!(
        "ChuteGPT — Análise Histórica Descritiva | Fonte: {} | {}",
        app.path.display(),
        freshness_status(app)
    )
}
fn result_lines(app: &App, sec: &Section) -> Vec<String> {
    let mut lines = vec![presentation::DEFINITIONS[app.menu].to_string()];
    if let Some(snapshot) = &app.snapshot {
        lines.push(snapshot_line(snapshot));
        let missing: usize = snapshot.missing_prize_fields.iter().sum();
        if missing > 0 {
            lines.push(format!("Atenção: {missing} campos de premiação indisponíveis no histórico (detalhes por faixa nas análises)."));
        }
    }
    let eligible = sec.values.as_ref().map(|v| v.iter().flatten().count());
    if let Some(n) = eligible {
        lines.push(format!(
            "Observações válidas: {n}; indisponíveis: {}",
            sec.values.as_ref().unwrap().len() - n
        ));
    } else if sec.title == "Espaçamentos agrupados" {
        lines.push(format!(
            "Espaçamentos válidos: {}",
            sec.bins.values().sum::<usize>()
        ));
    }
    lines.push(String::new());
    let denominator: usize = sec.bins.values().sum();
    for (i, (&v, &count)) in sec.bins.iter().enumerate() {
        let label = if app.menu == 0 && sec.scale == 1 {
            format!("{} pares / {} ímpares", v, 7 - v)
        } else {
            category_label(sec, v, app.menu)
        };
        lines.push(format!(
            "{} {}: {} ({})",
            if i == app.category { "▶" } else { " " },
            label,
            count,
            if denominator == 0 {
                "percentual indisponível".into()
            } else {
                format!("{:.2}%", 100.0 * count as f64 / denominator as f64)
            }
        ));
    }
    lines.extend(sec.lines.iter().cloned());
    if !app.error.is_empty() {
        lines.push(app.error.clone());
    }
    lines
}
fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    if area.width < 55 || area.height < 19 {
        frame.render_widget(Paragraph::new("Terminal pequeno demais. Aumente para pelo menos 55 × 19; o estado será preservado.").wrap(Wrap{trim:true}).block(Block::default().borders(Borders::ALL).title("ChuteGPT")),area);
        return;
    }
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(8),
            Constraint::Length(2),
        ])
        .split(area);
    frame.render_widget(
        Paragraph::new(if app.screen == Screen::Menu {
            let header = menu_header(app);
            if header.chars().count() > area.width.saturating_sub(2) as usize {
                format!("{}\n{}", header, freshness_status(app))
            } else {
                header
            }
        } else {
            "ChuteGPT — análise histórica descritiva".into()
        })
        .block(Block::default().borders(Borders::ALL)),
        areas[0],
    );
    match app.screen {
        Screen::Menu => {
            let items: Vec<_> = STRATEGIES
                .iter()
                .enumerate()
                .map(|(i, name)| ListItem::new(format!("{:02}. {name}", i + 1)))
                .collect();
            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Escolha uma estratégia"),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("▶ ");
            let mut state = ListState::default();
            state.select(Some(app.menu));
            let panes = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(areas[1]);
            frame.render_stateful_widget(list, panes[0], &mut state);
            frame.render_widget(
                Paragraph::new(menu_explanation(app))
                    .wrap(Wrap { trim: false })
                    .scroll((app.menu_scroll.min(u16::MAX as usize) as u16, 0))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Como funciona"),
                    ),
                panes[1],
            );
        }
        Screen::Loading | Screen::Working => {
            let (done, total) = app.progress;
            let label = if app.screen == Screen::Loading {
                format!(
                    "{} Lendo e validando a planilha…",
                    ["|", "/", "−", "\\"][app.tick % 4]
                )
            } else if total == 0 {
                format!(
                    "{} Calculando… progresso indeterminado",
                    ["|", "/", "−", "\\"][app.tick % 4]
                )
            } else {
                format!(
                    "Calculando: {done}/{total} — {:.0}%",
                    100.0 * done as f64 / total as f64
                )
            };
            let ratio = if total == 0 {
                0.0
            } else {
                (done as f64 / total as f64).clamp(0.0, 1.0)
            };
            let gauge = Gauge::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Processamento"),
                )
                .gauge_style(Style::default().fg(Color::Cyan))
                .ratio(ratio)
                .label(label);
            frame.render_widget(gauge, areas[1]);
        }
        Screen::Confirm => {
            let s = app.snapshot.as_ref().unwrap();
            let freshness = match s.freshness {
                Freshness::Stale { checked } => format!(
                    "Cópia possivelmente desatualizada: última atualização registrada em {}.",
                    checked.format("%d/%m/%Y")
                ),
                _ => "Não foi possível confirmar quando esta cópia foi atualizada.".into(),
            };
            frame.render_widget(
                Paragraph::new(vec![
                    Line::from(freshness),
                    Line::from(snapshot_line(s)),
                    Line::from("Continuar com esta cópia? Enter/S: sim; Esc/N: voltar."),
                ])
                .wrap(Wrap { trim: true })
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Atenção aos dados"),
                ),
                areas[1],
            );
        }
        Screen::Error => frame.render_widget(
            Paragraph::new(app.error.clone())
                .wrap(Wrap { trim: true })
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Não foi possível concluir"),
                ),
            areas[1],
        ),
        Screen::Series => frame.render_widget(
            Paragraph::new(app.details.join("\n"))
                .scroll((app.scroll.min(u16::MAX as usize) as u16, 0))
                .wrap(Wrap { trim: false })
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Detalhes estatísticos da série"),
                ),
            areas[1],
        ),
        Screen::Result => {
            let view = app.view.as_ref().unwrap();
            let sec = &view.sections[app.section];
            let pieces = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(2),
                    Constraint::Length(7),
                    Constraint::Min(1),
                ])
                .split(areas[1]);
            let subtitle = format!(
                "{} | seção {}/{}: {}",
                view.title,
                app.section + 1,
                view.sections.len(),
                sec.title
            );
            frame.render_widget(Paragraph::new(subtitle), pieces[0]);
            if sec.bins.is_empty() {
                frame.render_widget(
                    Paragraph::new("Sem distribuição para este painel")
                        .block(Block::default().borders(Borders::ALL)),
                    pieces[1],
                );
            } else {
                let start = app.category.saturating_sub(4);
                let labels: Vec<String> = sec
                    .bins
                    .keys()
                    .skip(start)
                    .take(9)
                    .map(|&v| category_label(sec, v, app.menu))
                    .collect();
                let bars: Vec<(&str, u64)> = labels
                    .iter()
                    .zip(sec.bins.values().skip(start).take(9))
                    .map(|(label, &value)| (label.as_str(), value as u64))
                    .collect();
                frame.render_widget(
                    BarChart::default()
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title("Distribuição — valor / quantidade"),
                        )
                        .data(bars.as_slice())
                        .bar_width(5)
                        .bar_gap(1)
                        .bar_style(Style::default().fg(Color::Cyan)),
                    pieces[1],
                );
            }
            let lines = result_lines(app, sec);
            frame.render_widget(
                Paragraph::new(lines.join("\n"))
                    .scroll((app.scroll.min(u16::MAX as usize) as u16, 0))
                    .wrap(Wrap { trim: false })
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Valores e detalhes"),
                    ),
                pieces[2],
            );
        }
    }
    let help = match app.screen {
        Screen::Menu => "↑/↓ selecionar • PgUp/PgDn explicação • Enter executar • Q sair",
        Screen::Loading | Screen::Working => "Esc ou C cancelar e voltar",
        Screen::Confirm => "Enter/S continuar  •  Esc/N voltar",
        Screen::Error => "Enter ou Esc voltar ao menu",
        Screen::Result => {
            "Tab/Shift+Tab seção  •  ←/→ categoria  •  S série  •  ↑/↓ rolar  •  Esc menu  •  Q sair"
        }
        Screen::Series => "↑/↓ rolar  •  Esc voltar ao resultado",
    };
    frame.render_widget(
        Paragraph::new(Line::from(vec![Span::raw(help)]))
            .block(Block::default().borders(Borders::TOP)),
        areas[2],
    );
}
struct TerminalSession {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}
impl TerminalSession {
    fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut out = io::stdout();
        if let Err(e) = execute!(out, EnterAlternateScreen, Hide) {
            let _ = disable_raw_mode();
            return Err(e);
        }
        match Terminal::new(CrosstermBackend::new(out)) {
            Ok(terminal) => Ok(Self { terminal }),
            Err(e) => {
                let _ = disable_raw_mode();
                let _ = execute!(io::stdout(), Show, LeaveAlternateScreen);
                Err(e)
            }
        }
    }
}
impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), Show, LeaveAlternateScreen);
        let _ = self.terminal.show_cursor();
    }
}

pub fn run(path: &Path) -> Result<(), String> {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err("O ChuteGPT precisa de um terminal interativo.".into());
    }
    let mut terminal =
        TerminalSession::new().map_err(|e| format!("Falha ao preparar o terminal: {e}"))?;
    let mut app = App::new(path.to_owned());
    app.preflight();
    loop {
        app.poll();
        let size = terminal
            .terminal
            .size()
            .map_err(|e| format!("Falha ao consultar o terminal: {e}"))?;
        app.terminal_width = size.width;
        app.terminal_height = size.height;
        app.tick = app.tick.wrapping_add(1);
        terminal
            .terminal
            .draw(|frame| draw(frame, &app))
            .map_err(|e| format!("Falha ao desenhar a tela: {e}"))?;
        if event::poll(Duration::from_millis(100)).map_err(|e| format!("Falha no teclado: {e}"))?
            && let Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) = event::read().map_err(|e| format!("Falha no teclado: {e}"))?
            && app.key(code)
        {
            app.cancel.store(true, Ordering::Relaxed);
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};
    #[test]
    fn menu_explanations_and_scrolling_follow_selection() {
        let mut app = App::new(PathBuf::from("fonte.xlsx"));
        for (index, section) in SECTION_EXPLANATIONS.iter().enumerate() {
            let desc = explanation(index);
            assert_eq!(desc.split("\n\n").count(), 2);
            assert!(desc.contains(presentation::DEFINITIONS[index]));
            assert!(desc.contains(SECTION_EXPLANATIONS[index]));
            assert!(section.len() > 40);
        }
        assert!(menu_header(&app).contains(
            "ChuteGPT — Análise Histórica Descritiva | Fonte: fonte.xlsx | ainda não verificada"
        ));
        app.terminal_width = 55;
        app.terminal_height = 19;
        app.menu = 16;
        let width = (55usize * 70 / 100).saturating_sub(3) as u16;
        let lines = Paragraph::new(menu_explanation(&app))
            .wrap(Wrap { trim: false })
            .line_count(width);
        let visible = 19usize - 8;
        assert!(
            lines > visible,
            "a explicação precisa rolar no menor tamanho suportado"
        );
        for _ in 0..100 {
            app.key(KeyCode::PageDown);
        }
        assert_eq!(app.menu_scroll, lines - visible);
        let mut terminal = Terminal::new(TestBackend::new(55, 19)).unwrap();
        terminal.draw(|f| draw(f, &app)).unwrap();
        let flat: String = format!("{:?}", terminal.backend().buffer())
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        assert!(
            flat.contains("detalhaosconcursos"),
            "a última linha da explicação deve ser alcançável"
        );
        app.key(KeyCode::Down);
        assert_eq!(app.menu_scroll, 0);
        assert_eq!(app.menu, 17);
    }
    #[test]
    fn result_omits_source_and_digest_and_menu_uses_snapshot_status() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/Dia de Sorte-atual.xlsx");
        if !path.exists() {
            return;
        }
        let snapshot = Arc::new(data::load(&path).unwrap());
        let view = presentation::build(0, &snapshot.draws, |_, _| true).unwrap();
        let mut app = App::new(path.clone());
        app.snapshot = Some(snapshot.clone());
        assert!(menu_header(&app).contains(&path.display().to_string()));
        match snapshot.freshness {
            Freshness::Current { .. } => assert!(menu_header(&app).contains("Verificada hoje")),
            Freshness::Stale { .. } => assert!(menu_header(&app).contains("desatualizada")),
            Freshness::Unknown => assert!(menu_header(&app).contains("desconhecida")),
        }
        let mut stale = (*snapshot).clone();
        stale.freshness = Freshness::Stale {
            checked: chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
        };
        app.snapshot = Some(Arc::new(stale));
        assert!(menu_header(&app).contains("possivelmente desatualizada"));
        let mut unknown = (*snapshot).clone();
        unknown.freshness = Freshness::Unknown;
        app.snapshot = Some(Arc::new(unknown));
        assert!(menu_header(&app).contains("atualização desconhecida"));
        app.snapshot = Some(snapshot);
        app.view = Some(view);
        app.screen = Screen::Result;
        let mut terminal = Terminal::new(TestBackend::new(140, 40)).unwrap();
        terminal.draw(|f| draw(f, &app)).unwrap();
        let shown = format!("{:?}", terminal.backend().buffer());
        assert!(!shown.contains("Fonte:"));
        assert!(!shown.contains("SHA-256"));
        assert!(shown.contains("Observações válidas"));
        app.key(KeyCode::Esc);
        assert!(matches!(app.screen, Screen::Menu));
        assert!(menu_header(&app).contains(&path.display().to_string()));
    }
    #[test]
    fn failed_reload_clears_previously_verified_snapshot() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/Dia de Sorte-atual.xlsx");
        if !path.exists() {
            return;
        }
        let mut app = App::new(path.clone());
        app.snapshot = Some(Arc::new(data::load(&path).unwrap()));
        let (tx, rx) = mpsc::sync_channel(1);
        tx.send(Message::Loaded(Err("arquivo inválido".into())))
            .unwrap();
        app.receiver = Some(rx);
        app.poll();
        assert!(app.snapshot.is_none());
        app.key(KeyCode::Esc);
        assert!(menu_header(&app).contains("falha na verificação"));
        assert!(!menu_header(&app).contains("ainda não verificada"));
        assert!(menu_explanation(&app).contains("arquivo inválido"));
    }
    #[test]
    fn preflight_failure_is_reported_truthfully() {
        let mut app = App::new(PathBuf::from("/caminho/inexistente/planilha.xlsx"));
        app.preflight();
        for _ in 0..500 {
            app.poll();
            if app.receiver.is_none() {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(app.snapshot.is_none());
        assert!(matches!(app.verify, Some(Err(_))));
        assert!(menu_header(&app).contains("falha na verificação"));
        assert!(!menu_header(&app).contains("ainda não verificada"));
        assert!(menu_explanation(&app).contains("A verificação desta fonte falhou"));
        assert!(matches!(app.screen, Screen::Menu));
    }
    #[test]
    fn explanation_includes_full_strategy_and_path() {
        let mut app = App::new(PathBuf::from("/caminho/muito/longo/planilha.xlsx"));
        app.menu = 21;
        let description = menu_explanation(&app);
        assert!(description.contains(STRATEGIES[21]));
        assert!(description.contains("/caminho/muito/longo/planilha.xlsx"));
    }
    #[test]
    fn menu_header_is_one_line_in_the_requested_order() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/Dia de Sorte-atual.xlsx");
        if !path.exists() {
            return;
        }
        let mut snapshot = data::load(&path).unwrap();
        let today = chrono::Local::now().date_naive();
        snapshot.freshness = Freshness::Current { checked: today };
        let mut app = App::new(PathBuf::from("docs/Dia de Sorte-atual.xlsx"));
        app.snapshot = Some(Arc::new(snapshot));
        let expected = format!(
            "ChuteGPT — Análise Histórica Descritiva | Fonte: docs/Dia de Sorte-atual.xlsx | Verificada hoje ({})",
            today.format("%d/%m/%Y")
        );
        assert_eq!(menu_header(&app), expected);
        let mut terminal = Terminal::new(TestBackend::new(160, 24)).unwrap();
        terminal.draw(|f| draw(f, &app)).unwrap();
        assert!(format!("{:?}", terminal.backend().buffer()).contains(&expected));
    }
    #[test]
    fn preflight_updates_menu_status_without_starting_analysis() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/Dia de Sorte-atual.xlsx");
        if !path.exists() {
            return;
        }
        let mut app = App::new(path.clone());
        app.preflight();
        for _ in 0..500 {
            app.poll();
            if app.receiver.is_none() {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(app.snapshot.is_some());
        assert!(matches!(app.screen, Screen::Menu));
        assert!(!menu_header(&app).contains("ainda não verificada"));
    }

    #[test]
    fn undefined_leading_statistics_are_not_counted_as_unknown() {
        let mut app = App::new(PathBuf::from("x"));
        app.menu = 16;
        app.view = Some(View {
            title: "Frequência e ausência".into(),
            sections: vec![Section {
                title: "Atraso ≥ 5 entre sorteadas".into(),
                bins: [(0, 2)].into_iter().collect(),
                scale: 1,
                values: Some(vec![None, None, Some(0), Some(0)]),
                lines: vec![],
            }],
        });
        app.screen = Screen::Result;
        app.key(KeyCode::Char('s'));
        assert!(
            app.details
                .iter()
                .any(|line| line.contains("fora da população da série: 2"))
        );
        assert!(app.details.iter().any(|line| line.contains("Eventos: 2")));
        assert!(
            app.details
                .iter()
                .any(|line| line.contains("desconhecidos: 0"))
        );
    }

    #[test]
    fn series_population_matches_the_result_population_notice() {
        for menu in [16usize, 19] {
            let mut app = App::new(PathBuf::from("x"));
            app.menu = menu;
            let values = vec![None, None, Some(0), Some(0)];
            app.view = Some(View {
                title: "Série".into(),
                sections: vec![Section {
                    title: "Série".into(),
                    bins: [(0, 2)].into_iter().collect(),
                    scale: 1,
                    values: Some(values),
                    lines: vec![],
                }],
            });
            app.screen = Screen::Result;
            let result = result_lines(&app, app.active_section().unwrap());
            assert!(
                result
                    .iter()
                    .any(|line| line.contains("Observações válidas: 2; indisponíveis: 2")),
                "menu {menu}"
            );
            app.key(KeyCode::Char('s'));
            assert!(matches!(app.screen, Screen::Series), "menu {menu}");
            assert!(
                app.details
                    .iter()
                    .any(|line| line.contains("fora da população da série: 2")),
                "menu {menu}"
            );
            assert!(app.details.iter().any(|line| line.contains("Eventos: 2")));
            assert!(
                app.details
                    .iter()
                    .any(|line| line.contains("desconhecidos: 0"))
            );
        }
    }

    #[test]
    fn negative_hundredths_keep_their_sign() {
        assert_eq!(presentation::format_value(-50, 100), "-0.50");
        assert_eq!(presentation::format_value(125, 100), "1.25");
        assert_eq!(presentation::format_value(-125, 100), "-1.25");
    }

    #[test]
    fn menu_and_small_terminal_render() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let app = App::new(PathBuf::from("dados.xlsx"));
        terminal.draw(|f| draw(f, &app)).unwrap();
        let shown = format!("{:?}", terminal.backend().buffer());
        assert!(shown.contains("Paridade"));
        let mut small = Terminal::new(TestBackend::new(30, 10)).unwrap();
        small.draw(|f| draw(f, &app)).unwrap();
    }
    #[test]
    fn results_survive_resize_and_series_navigation() {
        let mut app = App::new(PathBuf::from("x"));
        let section = Section {
            title: "Paridade".into(),
            bins: [(0, 1), (4, 2)].into_iter().collect(),
            scale: 1,
            values: Some(vec![Some(0), Some(4), Some(4)]),
            lines: vec![],
        };
        app.view = Some(View {
            title: "Paridade".into(),
            sections: vec![section],
        });
        app.screen = Screen::Result;
        app.category = 1;
        let mut terminal = Terminal::new(TestBackend::new(100, 24)).unwrap();
        terminal.draw(|f| draw(f, &app)).unwrap();
        let shown = format!("{:?}", terminal.backend().buffer());
        assert!(shown.contains("4 pares / 3"));
        terminal.backend_mut().resize(30, 10);
        terminal.draw(|f| draw(f, &app)).unwrap();
        assert_eq!(app.category, 1);
        assert!(matches!(app.screen, Screen::Result));
        terminal.backend_mut().resize(100, 24);
        terminal.draw(|f| draw(f, &app)).unwrap();
        app.key(KeyCode::Char('s'));
        assert!(matches!(app.screen, Screen::Series));
        assert!(app.details.iter().any(|line| line.contains("Eventos: 2")));
        app.details.push("x".repeat(1000));
        app.details.push("MARCADOR FINAL".into());
        for _ in 0..500 {
            app.key(KeyCode::Down);
        }
        assert!(app.scroll > 0);
        assert!(app.scroll < 500);
        terminal.draw(|f| draw(f, &app)).unwrap();
        assert!(format!("{:?}", terminal.backend().buffer()).contains("MARCADOR FINAL"));
        app.key(KeyCode::Esc);
        assert!(matches!(app.screen, Screen::Result));
        assert_eq!(app.category, 1);
    }

    #[test]
    fn prior_only_series_excludes_first_ineligible_draw() {
        let mut app = App::new(PathBuf::from("x"));
        app.menu = 1;
        app.view = Some(View {
            title: "Repetência".into(),
            sections: vec![Section {
                title: "Repetição".into(),
                bins: [(0, 2)].into_iter().collect(),
                scale: 1,
                values: Some(vec![None, Some(0), Some(0)]),
                lines: vec![],
            }],
        });
        app.screen = Screen::Result;
        app.key(KeyCode::Char('s'));
        assert!(
            app.details
                .iter()
                .any(|line| line.contains("fora da população da série: 1"))
        );
        assert!(
            app.details
                .iter()
                .any(|line| line.contains("Periodicidades: [1, 1]"))
        );
        assert!(
            app.details
                .iter()
                .any(|line| line.contains("desconhecidos: 0"))
        );
    }

    #[test]
    fn indeterminate_spinner_changes_and_cancel_returns_to_menu() {
        let mut app = App::new(PathBuf::from("x"));
        app.screen = Screen::Loading;
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        terminal.draw(|f| draw(f, &app)).unwrap();
        let first = terminal.backend().buffer().clone();
        app.tick = 1;
        terminal.draw(|f| draw(f, &app)).unwrap();
        assert_ne!(first, *terminal.backend().buffer());
        app.screen = Screen::Working;
        assert!(!app.key(KeyCode::Esc));
        assert!(app.cancel.load(Ordering::Relaxed));
        assert!(matches!(app.screen, Screen::Menu));
    }

    #[test]
    fn progress_updates_are_bounded_per_frame() {
        let mut app = App::new(PathBuf::from("x"));
        let (tx, rx) = mpsc::sync_channel(32);
        for done in 1..=20 {
            tx.send(Message::Progress(done, 20)).unwrap();
        }
        app.receiver = Some(rx);
        app.screen = Screen::Working;
        app.poll();
        assert_eq!(app.progress, (8, 20));
        app.poll();
        assert_eq!(app.progress, (16, 20));
    }

    #[test]
    fn official_snapshot_renders_each_section_and_available_series() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/Dia de Sorte-atual.xlsx");
        if !path.exists() {
            eprintln!("Planilha oficial local ausente; teste de telas não executado");
            return;
        }
        let snapshot = Arc::new(data::load(&path).unwrap());
        let mut terminal = Terminal::new(TestBackend::new(100, 30)).unwrap();
        for (index, strategy) in STRATEGIES.iter().enumerate() {
            let view = presentation::build(index, &snapshot.draws, |_, _| true).unwrap();
            let count = view.sections.len();
            let mut app = App::new(path.clone());
            app.menu = index;
            app.snapshot = Some(snapshot.clone());
            app.view = Some(view);
            app.screen = Screen::Result;
            for section in 0..count {
                app.section = section;
                app.category = 0;
                app.scroll = 0;
                terminal.draw(|f| draw(f, &app)).unwrap();
                let selected = app.active_section().and_then(|s| {
                    if s.values.is_none() {
                        None
                    } else {
                        s.bins.values().position(|count| *count > 0)
                    }
                });
                if let Some(category) = selected {
                    app.category = category;
                    app.key(KeyCode::Char('s'));
                    assert!(
                        matches!(app.screen, Screen::Series),
                        "{} seção {section}",
                        strategy
                    );
                    terminal.draw(|f| draw(f, &app)).unwrap();
                    app.key(KeyCode::Esc);
                    assert!(matches!(app.screen, Screen::Result));
                }
            }
        }
    }

    #[test]
    fn menu_keyboard_wraps() {
        let mut app = App::new(PathBuf::from("x"));
        assert!(!app.key(KeyCode::Up));
        assert_eq!(app.menu, 21);
        assert!(!app.key(KeyCode::Down));
        assert_eq!(app.menu, 0);
        assert!(app.key(KeyCode::Char('q')));
    }
}
