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
    layout::{Constraint, Direction, Layout, Rect},
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
    provenance_view: bool,
    provenance_scroll: usize,
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
            provenance_view: false,
            provenance_scroll: 0,
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
        if self.provenance_view {
            let maximum = provenance_max_scroll(self);
            let page = provenance_viewport(self).1.max(1);
            match key {
                KeyCode::Esc | KeyCode::Backspace | KeyCode::Char('p' | 'P') => {
                    self.provenance_view = false;
                    self.provenance_scroll = 0;
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    self.provenance_scroll = self.provenance_scroll.saturating_add(1).min(maximum)
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.provenance_scroll = self.provenance_scroll.saturating_sub(1)
                }
                KeyCode::PageDown => {
                    self.provenance_scroll = self
                        .provenance_scroll
                        .saturating_add(page.saturating_sub(1).max(1))
                        .min(maximum)
                }
                KeyCode::PageUp => {
                    self.provenance_scroll = self.provenance_scroll.saturating_sub(page.max(1))
                }
                KeyCode::Home => self.provenance_scroll = 0,
                KeyCode::End => self.provenance_scroll = maximum,
                _ => {}
            }
            return false;
        }
        if matches!(key, KeyCode::Char('p' | 'P')) {
            self.provenance_view = true;
            self.provenance_scroll = 0;
            return false;
        }
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
                    let lines = Paragraph::new(menu_explanation(self))
                        .wrap(Wrap { trim: false })
                        .line_count(explanation_width(self.terminal_width).max(1));
                    let visible = menu_explanation_viewport_height(self);
                    self.menu_scroll = self
                        .menu_scroll
                        .saturating_add(1)
                        .min(lines.saturating_sub(visible));
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
                    let visible = series_viewport_height(self);
                    let lines = Paragraph::new(content)
                        .wrap(Wrap { trim: false })
                        .line_count(width);
                    let maximum = lines.saturating_sub(visible);
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
                        let visible = result_details_viewport_height(self);
                        let total = Paragraph::new(content)
                            .wrap(Wrap { trim: false })
                            .line_count(width);
                        self.scroll = self
                            .scroll
                            .saturating_add(1)
                            .min(total.saturating_sub(visible));
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
    "Em cada concurso válido, contam-se as dezenas pares entre as sete dezenas sorteadas; o total de ímpares é sete menos o total de pares. O cálculo usa o resultado do próprio concurso e não compara concursos entre si.\n\nA distribuição inclui as oito categorias possíveis, de zero a sete pares, mesmo quando alguma não ocorreu. O denominador é o número de concursos analisados; o painel mostra as contagens de pares e seu complemento de ímpares. É uma descrição histórica, não uma regra para escolher dezenas.",
    "Para cada concurso, compara-se seu conjunto de sete dezenas com o do concurso imediatamente anterior na ordem histórica validada. A contagem é o tamanho da interseção, de zero a sete; o primeiro concurso não tem antecessor e fica fora da população elegível.\n\nA distribuição apresenta quantos concursos elegíveis tiveram cada quantidade de repetidas, incluindo categorias sem ocorrências. O número elegível e a exclusão inicial são informados separadamente. A coincidência observada não implica que a mesma quantidade se repetirá.",
    "Para cada concurso após o primeiro, compara-se suas sete dezenas somente com concursos estritamente anteriores, nunca com resultados posteriores. O valor é a maior interseção encontrada entre essas comparações; se não houver números coincidentes, o resultado é zero.\n\nA primeira posição fica indisponível por não haver histórico anterior. O painel distribui os máximos entre zero e sete e informa a população elegível. Isso descreve semelhança retrospectiva e não estima a chance do próximo sorteio.",
    "As sete dezenas de cada concurso são ordenadas de forma crescente e suas seis vizinhanças são examinadas. Cada par adjacente com diferença exatamente igual a um acrescenta uma ocorrência; um bloco de três dezenas consecutivas contém dois pares adjacentes.\n\nO painel distribui a contagem inteira de zero a seis por concurso. A população é formada por todos os concursos válidos, sem dependência de concursos anteriores; os valores são contagens de pares, não de blocos ou de dezenas consecutivas.",
    "Em cada conjunto ordenado de sete dezenas, percorrem-se as seis diferenças entre vizinhas. O resultado é a maior sequência de ligações consecutivas com diferença um; se não houver ligação desse tipo, o valor é zero.\n\nA distribuição reúne concursos pelo tamanho da maior sequência, medido em ligações e limitado de zero a seis. Assim, três dezenas consecutivas correspondem a duas ligações. O indicador resume apenas a composição de cada concurso.",
    "A entrada de cada observação são as sete dezenas do concurso. Somam-se seus valores inteiros sem arredondamento intermediário, produzindo uma soma entre os limites permitidos pelas dezenas válidas.\n\nO resultado é uma distribuição por soma exata, com a quantidade de concursos em cada valor. Todas as observações válidas participam; o painel também apresenta o tamanho da população e o intervalo histórico. A soma passada não altera a probabilidade teórica de uma combinação.",
    "Primeiro somam-se as sete dezenas de cada concurso; em seguida, somam-se repetidamente os algarismos desse total até restar um único algarismo, de um a nove. A categoria é calculada a partir da soma, não das dezenas isoladamente.\n\nA distribuição apresenta a quantidade de concursos em cada raiz digital possível. Todos os concursos válidos são considerados, e as categorias sem ocorrências continuam identificadas. Essa transformação é uma classificação descritiva, sem significado causal ou vantagem preditiva.",
    "Para cada concurso, calculam-se sete medidas sobre suas dezenas: amplitude, média, média das raízes quadradas, RMS, desvio absoluto médio, desvio padrão populacional e coeficiente de variação. As fórmulas usam os valores não arredondados; medidas reais são arredondadas a duas casas decimais para exibição e agrupamento.\n\nO painel traz uma distribuição por medida e valores associados a cada concurso. A população é de concursos, não de dezenas individuais; nenhuma medida usa sorteios futuros ou presume tendência. O coeficiente de variação fica indisponível se sua média não puder servir de denominador.",
    "Em cada concurso, ordenam-se as sete dezenas e calculam-se as seis diferenças positivas entre valores vizinhos. As sete medidas de dispersão são aplicadas a esse vetor de espaçamentos, sem arredondamento intermediário e com exibição a duas casas decimais.\n\nAlém das distribuições das medidas por concurso, há uma seção agregada com as frequências dos espaçamentos individuais; seu denominador é seis vezes o número de concursos. Os valores agregados não são associados artificialmente a um concurso específico, e as medidas por concurso mantêm uma observação por vetor de seis diferenças.",
    "Cada dezena é colocada em um dos grupos fixos: 01–10, 11–20, 21–30 ou 31. Para cada concurso, contam-se as dezenas em cada grupo e identifica-se a maior ocupação; a fronteira não usa o agrupamento alternativo de outras aplicações legadas.\n\nAs seções mostram totais por grupo, a distribuição da ocupação de cada faixa e o histograma da ocupação máxima por concurso. Os totais usam sete dezenas por concurso como população e a máxima é uma contagem entre zero e sete.",
    "Para cada concurso, classifica-se cada dezena pelo algarismo final, de zero a nove, e contam-se as ocorrências nas dez colunas. A ordenação das dezenas não altera essas contagens.\n\nAs seções mostram os totais de cada final, as frequências de ocupação por coluna e a distribuição da maior ocupação de uma coluna em cada concurso. A população das contagens totais é de sete dezenas por concurso; o máximo é apresentado separadamente, sem inferência sobre resultados futuros.",
    "Cada concurso é dividido nas quatro linhas de dezenas 01–10, 11–20, 21–30 e 31 e nas dez colunas por algarismo final. Calcula-se o maior número de dezenas em qualquer linha, o maior número em qualquer coluna e a soma desses dois máximos.\n\nO resultado apresenta distribuições separadas para máximo por linha, máximo por coluna e soma dos máximos, além dos valores por concurso. As entradas são as sete dezenas válidas de cada observação; a soma dos máximos é um resumo, não o total de dezenas do sorteio.",
    "Dois rastreadores independentes percorrem a sequência histórica: um aguarda a cobertura de pelo menos 27 dezenas distintas, o outro a cobertura das 31. Cada concurso entra inteiro; ao atingir o limite, esse concurso fecha o ciclo e o próximo ciclo começa no concurso seguinte.\n\nPara cada limite, as seções mostram a duração dos ciclos concluídos e identificam seus concursos inicial e final e a cobertura alcançada. A cauda ainda aberta aparece separadamente, com duração e dezenas cobertas, e nunca entra no histograma como ciclo completo.",
    "Cada conjunto de sete dezenas é convertido em sua posição ordinal, começando em 1, na ordem lexicográfica de todas as combinações possíveis de sete entre 31; o cálculo usa contagem combinatória exata. As posições são agrupadas em dez faixas com limites inteiros explícitos.\n\nUma segunda distribuição agrupa as diferenças absolutas entre ordinais de concursos consecutivos; a primeira não tem diferença anterior e fica indisponível. Repetições permanecem como observações próprias e geram diferença zero. As seções também listam ordinais e diferenças, sem transformar posição em probabilidade.",
    "Para cada dezena de 01 a 31, percorre-se o histórico validado para contar aparições e localizar a última ocorrência. O atraso atual é o número de concursos posteriores a essa última aparição; uma dezena nunca sorteada tem atraso indisponível, distinto de zero.\n\nO resumo apresenta ocorrências e atraso para cada dezena, e as seções individuais mostram sua série de ausência ou presença ao longo dos concursos. O atraso zero indica presença no concurso mais recente. São medidas retrospectivas do conjunto selecionado, sem ranking de apostas.",
    "A unidade é o mês da sorte registrado em cada concurso, separado das sete dezenas numéricas. Para cada um dos 12 meses contam-se ocorrências e calcula-se o número de concursos decorridos desde sua aparição mais recente; mês nunca observado tem atraso indisponível.\n\nO painel resume os 12 totais e atrasos e oferece uma série binária de presença ou ausência para cada mês. A sequência respeita a ordem dos concursos da planilha, e atraso zero significa que o mês apareceu no registro mais recente. A frequência passada não prevê o mês seguinte.",
    "Para cada concurso depois do primeiro, frequência e atraso são calculados usando somente concursos estritamente anteriores. A frequência conta aparições de cada dezena; o atraso conta concursos posteriores à última aparição. Os rankings incluem empates no décimo lugar, enquanto periodicidade é o número de concursos anteriores dividido pela frequência positiva; valores nunca observados ficam indisponíveis.\n\nAs seções mostram quantas dezenas sorteadas pertenciam aos topos de frequência e atraso, incluindo empates no décimo lugar; periodicidade ≥4 entre dezenas sorteadas; e atraso ≥5 somente quando todas as 31 dezenas têm histórico observado. Também apresentam histogramas e medidas de dispersão para todas as dezenas e para as sorteadas. A primeira observação não tem histórico, e valores ausentes são informados sem substituir ausências por zero.",
    "Em cada concurso válido, toma-se a maior dezena e subtrai-se a menor. O resultado inteiro é a amplitude do conjunto e depende apenas das sete dezenas daquela observação; não exige histórico anterior.\n\nA seção distribui concursos pela distância entre extremos, mantendo valores exatos e identificando a população analisada. Essa medida coincide matematicamente com a amplitude apresentada em dispersão, mas permanece como análise individual para preservar o catálogo legado.",
    "Para cada concurso, calcula-se a raiz quadrada de cada uma das sete dezenas, tira-se a média aritmética desses sete valores e arredonda-se o resultado ao inteiro mais próximo, com empates para o inteiro par. O arredondamento ocorre somente ao final da média.\n\nA seção distribui os concursos pelo valor inteiro resultante e apresenta as categorias observadas. Trata-se de uma média transformada das raízes, não da mediana estatística; todos os concursos são tratados individualmente e não há comparação com sorteios futuros.",
    "Para cada concurso após o primeiro, toma-se cada uma de suas sete dezenas e consulta-se sua frequência e seu atraso calculados apenas com o histórico anterior. A medida só é definida quando todas as 31 dezenas já apareceram nesse histórico; antes disso, a observação fica indisponível, sem completar ausências com valor artificial.\n\nNos casos elegíveis, calculam-se os desvios-padrão populacionais das sete frequências e dos sete atrasos; a raiz quadrada do produto é arredondada ao inteiro mais próximo, com empate para o par. A distribuição mostra os resultados elegíveis e a contagem de observações excluídas por cobertura insuficiente.",
    "Cada concurso é agrupado pelo dia da semana de sua data e pelas cinco faixas registradas na planilha: 7, 6, 5 e 4 acertos e mês da sorte. Os campos de ganhadores e rateio são lidos separadamente; dados ausentes ou inválidos permanecem desconhecidos, não equivalem a zero.\n\nPara cada faixa e dia, as tabelas mostram concursos, ganhadores conhecidos, concursos premiados, soma de ganhadores, média entre concursos com ganhadores, parcela dos concursos premiados entre os dias e média monetária por linha de rateio conhecida. Cada medida usa seu próprio denominador; a média de rateio não é por ganhador. São estatísticas históricas, não chances condicionais de premiação.",
    "A estratégia produz séries separadas para cada um dos 12 meses e para a ocorrência de prêmio principal, definida por pelo menos um ganhador conhecido na faixa de 7 acertos. Para o prêmio, um campo ausente é estado desconhecido; não é classificado como concurso sem ganhador. As observações seguem a ordem histórica validada.\n\nCada série mostra quantidade de ocorrências, intervalos fechados entre eventos e sua distribuição, além do atraso aberto atual como cauda censurada. O intervalo fechado conta concursos intervenientes, admitindo zero quando os eventos são consecutivos; valores desconhecidos impedem medir o trecho afetado e não são tratados como ausência. O painel também informa eventos e lacunas de informação.",
];
fn explanation(index: usize) -> String {
    format!(
        "{}\n\n{}",
        presentation::DEFINITIONS[index],
        SECTION_EXPLANATIONS[index]
    )
}
fn menu_explanation(app: &App) -> String {
    let mut text = format!("{}\n\n{}", STRATEGIES[app.menu], explanation(app.menu));
    if let Some(Err(error)) = &app.verify {
        text.push_str(&format!(
            "\n\nA verificação desta fonte falhou: {error}\nO arquivo precisa existir e ser uma planilha XLSX válida do Dia de Sorte. Pressione Enter para tentar novamente."
        ));
    }
    text
}
fn explanation_width(area_width: u16) -> u16 {
    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(Rect::new(0, 0, area_width, 1));
    panes[1].width.saturating_sub(2)
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
fn screen_header(app: &App) -> String {
    format!(
        "ChuteGPT — Análise Histórica Descritiva | Fonte: {} | {}",
        app.path.display(),
        freshness_status(app)
    )
}
fn display_header(app: &App, area: Rect) -> String {
    let header = screen_header(app);
    if header_overflows(&header, area) {
        format!(
            "ChuteGPT — Análise Histórica Descritiva | Fonte: caminho longo (P: caminho completo) | {}",
            freshness_status(app)
        )
    } else {
        header
    }
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
/// Height needed for the rendered header, bounded to preserve the body and
/// footer space; `display_header` replaces paths that exceed this budget.
fn header_lines(header: &str, area: Rect) -> u16 {
    let inner = area.width.saturating_sub(2).max(1);
    Paragraph::new(header)
        .wrap(Wrap { trim: true })
        .line_count(inner) as u16
}
fn header_overflows(header: &str, area: Rect) -> bool {
    let available = area.height.saturating_sub(10).max(3);
    header_lines(header, area).saturating_add(2) > available
}
fn header_height(header: &str, area: Rect) -> u16 {
    let available = area.height.saturating_sub(10).max(3);
    header_lines(header, area)
        .saturating_add(2)
        .clamp(3, available)
}
fn body_area_height(app: &App) -> u16 {
    let area = Rect::new(0, 0, app.terminal_width, app.terminal_height);
    let header = display_header(app, area);
    area.height
        .saturating_sub(header_height(&header, area))
        .saturating_sub(2)
}
fn menu_explanation_viewport_height(app: &App) -> usize {
    usize::from(body_area_height(app).saturating_sub(2))
}
fn series_viewport_height(app: &App) -> usize {
    usize::from(body_area_height(app).saturating_sub(2))
}
fn result_details_viewport_height(app: &App) -> usize {
    let body = Rect::new(0, 0, app.terminal_width, body_area_height(app));
    usize::from(result_areas(body)[2].height.saturating_sub(2))
}
fn result_areas(body: Rect) -> std::rc::Rc<[Rect]> {
    // Keep a bordered detail panel with at least one readable row, even when
    // the header uses its full budget at the minimum terminal size.
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(body.height.saturating_sub(5).min(7)),
            Constraint::Min(3),
        ])
        .split(body)
}
fn provenance_viewport(app: &App) -> (usize, usize) {
    let area = Rect::new(0, 0, app.terminal_width, app.terminal_height);
    let visible = usize::from(body_area_height(app).saturating_sub(2));
    let width = area.width.saturating_sub(2).max(1);
    let lines = Paragraph::new(app.path.display().to_string())
        .wrap(Wrap { trim: false })
        .line_count(width);
    (
        lines.saturating_sub(visible).min(u16::MAX as usize),
        visible,
    )
}
fn provenance_max_scroll(app: &App) -> usize {
    provenance_viewport(app).0
}
fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    if area.width < 55 || area.height < 19 {
        frame.render_widget(Paragraph::new("Terminal pequeno demais. Aumente para pelo menos 55 × 19; o estado será preservado.").wrap(Wrap{trim:true}).block(Block::default().borders(Borders::ALL).title("ChuteGPT")),area);
        return;
    }
    let header = display_header(app, area);
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(header_height(&header, area)),
            Constraint::Min(8),
            Constraint::Length(2),
        ])
        .split(area);
    frame.render_widget(
        Paragraph::new(header)
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL)),
        areas[0],
    );
    if app.provenance_view {
        frame.render_widget(
            Paragraph::new(app.path.display().to_string())
                .wrap(Wrap { trim: false })
                .scroll((
                    app.provenance_scroll
                        .min(provenance_max_scroll(app))
                        .min(u16::MAX as usize) as u16,
                    0,
                ))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Caminho completo da planilha"),
                ),
            areas[1],
        );
    } else {
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
                let pieces = result_areas(areas[1]);
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
    }
    let help = if app.provenance_view {
        "P/Esc fechar • ↑/↓ rolar • PgUp/PgDn página • Home/End início/fim"
    } else {
        match app.screen {
            Screen::Menu => "↑/↓ selecionar • PgUp/PgDn explicação • Enter executar • Q sair",
            Screen::Loading | Screen::Working => "Esc ou C cancelar e voltar",
            Screen::Confirm => "Enter/S continuar  •  Esc/N voltar",
            Screen::Error => "Enter ou Esc voltar ao menu",
            Screen::Result => {
                "Tab/Shift+Tab seção  •  ←/→ categoria  •  S série  •  ↑/↓ rolar  •  Esc menu  •  Q sair"
            }
            Screen::Series => "↑/↓ rolar  •  Esc voltar ao resultado",
        }
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
        assert!(screen_header(&app).contains(
            "ChuteGPT — Análise Histórica Descritiva | Fonte: fonte.xlsx | ainda não verificada"
        ));
        for index in 0..STRATEGIES.len() {
            app.menu = index;
            app.menu_scroll = 0;
            let desc = explanation(index);
            let shown = menu_explanation(&app);
            assert_eq!(desc.split("\n\n").count(), 3, "estratégia {index}");
            assert!(desc.contains(presentation::DEFINITIONS[index]));
            assert!(SECTION_EXPLANATIONS[index].len() > 300);
            assert!(shown.contains(STRATEGIES[index]));
            assert!(shown.contains(&desc));
            assert!(!shown.contains("Fonte:"));

            for (width, height) in [(55, 19), (100, 24)] {
                app.terminal_width = width;
                app.terminal_height = height;
                app.menu_scroll = 0;
                let pane_width = explanation_width(width);
                let lines = Paragraph::new(shown.clone())
                    .wrap(Wrap { trim: false })
                    .line_count(pane_width);
                let visible = menu_explanation_viewport_height(&app);
                for _ in 0..lines {
                    app.key(KeyCode::PageDown);
                }
                assert_eq!(
                    app.menu_scroll,
                    lines.saturating_sub(visible),
                    "rolagem incompleta para estratégia {index} em {width}×{height}"
                );

                let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
                terminal.draw(|f| draw(f, &app)).unwrap();
                let buffer = terminal.backend().buffer();
                let rendered: String = (0..height)
                    .flat_map(|y| (0..width).map(move |x| buffer[(x, y)].symbol()))
                    .collect::<String>()
                    .chars()
                    .filter(|character| !character.is_whitespace())
                    .collect();
                let last_word: String = SECTION_EXPLANATIONS[index]
                    .split_whitespace()
                    .rev()
                    .take(1)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<String>();
                let last_word: String = last_word
                    .chars()
                    .filter(|character| !character.is_whitespace())
                    .collect();
                assert!(
                    rendered.contains(&last_word),
                    "fim da explicação não ficou visível para estratégia {index} em {width}×{height}: esperado {last_word}, rolagem {}/{lines}, visível {visible}",
                    app.menu_scroll
                );
            }
        }
        app.menu = 0;
        app.menu_scroll = 5;
        app.key(KeyCode::Down);
        assert_eq!(app.menu_scroll, 0);
        assert_eq!(app.menu, 1);
    }

    #[test]
    fn provenance_header_is_rendered_for_every_screen_state() {
        use chrono::NaiveDate;

        let mut app = App::new(PathBuf::from("fonte.xlsx"));
        app.snapshot = Some(Arc::new(Snapshot {
            path: PathBuf::from("fonte.xlsx"),
            sha256: "snapshot-hash".into(),
            draws: vec![data::Draw {
                contest: 1,
                date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
                numbers: [1, 2, 3, 4, 5, 6, 7],
                month: 1,
                prizes: std::array::from_fn(|_| data::Prize {
                    winners: None,
                    cents: None,
                }),
            }],
            freshness: Freshness::Unknown,
            missing_prize_fields: [0; 10],
        }));
        app.view = Some(View {
            title: STRATEGIES[0].into(),
            sections: vec![Section {
                title: "Pares".into(),
                bins: [(4, 1)].into_iter().collect(),
                scale: 1,
                values: Some(vec![Some(4)]),
                lines: Vec::new(),
            }],
        });
        app.details = vec!["Detalhe da série".into()];
        app.error = "Falha de teste".into();

        let expected = "ChuteGPT — Análise Histórica Descritiva | Fonte: fonte.xlsx | atualização desconhecida";
        for screen in [
            Screen::Menu,
            Screen::Loading,
            Screen::Confirm,
            Screen::Working,
            Screen::Result,
            Screen::Series,
            Screen::Error,
        ] {
            app.screen = screen;
            let mut terminal = Terminal::new(TestBackend::new(100, 24)).unwrap();
            terminal.draw(|frame| draw(frame, &app)).unwrap();
            let rendered = format!("{:?}", terminal.backend().buffer());
            assert!(rendered.contains(expected));
            assert!(!rendered.contains("ChuteGPT — análise histórica descritiva"));
        }
    }
    #[test]
    fn provenance_header_keeps_the_full_path_visible_on_narrow_terminals() {
        let path = "/opt/fixture/program/example/example-example-example-example-workbook.xlsx";
        let app = App::new(PathBuf::from(path));
        let header = screen_header(&app);
        assert!(header.chars().count() > 55);
        for (width, height) in [(55, 19), (100, 24)] {
            let area = Rect::new(0, 0, width, height);
            let header_height = header_height(&header, area);
            assert!(
                height - header_height >= 10,
                "cabeçalho consumiu a área útil em {width}×{height}"
            );
            let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
            terminal.draw(|frame| draw(frame, &app)).unwrap();
            let buffer = terminal.backend().buffer();
            let shown: String = (1..header_height - 1)
                .flat_map(|y| (1..width - 1).map(move |x| buffer[(x, y)].symbol().to_string()))
                .collect::<String>()
                .chars()
                .filter(|character| !character.is_whitespace())
                .collect();
            let expected: String = path
                .chars()
                .filter(|character| !character.is_whitespace())
                .collect();
            assert!(
                shown.contains(&expected),
                "caminho completo ausente em {width}×{height}: {shown}"
            );
            assert!(shown.contains("ChuteGPT—AnáliseHistóricaDescritiva"));
            let body: String = (header_height..height)
                .flat_map(|y| (0..width).map(move |x| buffer[(x, y)].symbol().to_string()))
                .collect();
            assert!(body.contains("Como funciona"));
        }
    }
    #[test]
    fn oversized_provenance_view_scrolls_and_preserves_processing_state() {
        let component = "c".repeat(240);
        let path = format!(
            "/START-OF-PATH/{}/{}/{}/{}/{}/END-OF-PATH.xlsx",
            component, component, component, component, component
        );
        let mut app = App::new(PathBuf::from(&path));
        app.terminal_width = 55;
        app.terminal_height = 19;
        app.screen = Screen::Working;
        app.menu = 7;
        let (sender, receiver) = mpsc::channel();
        app.receiver = Some(receiver);

        assert!(!app.key(KeyCode::Char('p')));
        assert!(app.provenance_view);
        let area = Rect::new(0, 0, 55, 19);
        let header = display_header(&app, area);
        assert!(header.contains("caminho longo"));
        assert!(header.contains("P: caminho completo"));

        let mut terminal = Terminal::new(TestBackend::new(55, 19)).unwrap();
        terminal.draw(|frame| draw(frame, &app)).unwrap();
        let buffer = terminal.backend().buffer();
        let first_page: String = (0..19)
            .flat_map(|y| (0..55).map(move |x| buffer[(x, y)].symbol()))
            .collect::<String>()
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect();
        assert!(first_page.contains("Caminhocompletodaplanilha"));
        assert!(first_page.contains("START-OF-PATH"));

        app.key(KeyCode::End);
        assert_eq!(app.provenance_scroll, provenance_max_scroll(&app));
        terminal.draw(|frame| draw(frame, &app)).unwrap();
        let buffer = terminal.backend().buffer();
        let last_page: String = (0..19)
            .flat_map(|y| (0..55).map(move |x| buffer[(x, y)].symbol()))
            .collect::<String>()
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect();
        assert!(
            last_page.contains("END-OF-PATH.xlsx"),
            "o fim do caminho não ficou acessível pela rolagem: {last_page}; offset {}",
            app.provenance_scroll
        );

        sender.send(Message::Progress(4, 9)).unwrap();
        app.poll();
        assert_eq!(app.progress, (4, 9));
        assert!(matches!(app.screen, Screen::Working));
        assert!(app.provenance_view);

        sender
            .send(Message::Done(Ok(View {
                title: "Resultado concluído".into(),
                sections: Vec::new(),
            })))
            .unwrap();
        app.poll();
        assert!(matches!(app.screen, Screen::Result));
        assert!(app.provenance_view);
        app.key(KeyCode::Esc);
        assert!(!app.provenance_view);
        assert!(matches!(app.screen, Screen::Result));
        assert_eq!(app.menu, 7);
        assert_eq!(app.view.as_ref().unwrap().title, "Resultado concluído");
        app.key(KeyCode::Char('p'));
        assert!(app.provenance_view);
        app.key(KeyCode::Char('P'));
        assert!(!app.provenance_view);
        assert!(matches!(app.screen, Screen::Result));
    }
    #[test]
    fn explanation_scrolling_reaches_end_with_multi_line_header() {
        let path = "/opt/fixture/program/example/example-example-example-example-workbook.xlsx";
        let mut app = App::new(PathBuf::from(path));
        app.terminal_width = 55;
        app.terminal_height = 19;
        let area = Rect::new(0, 0, 55, 19);
        let header = display_header(&app, area);
        assert!(
            header_height(&header, area) >= 5,
            "o cabeçalho deve ocupar pelo menos 5 linhas de altura"
        );
        for index in [0, 7, 16, 21] {
            app.menu = index;
            app.menu_scroll = 0;
            let pane_width = explanation_width(55);
            let shown = menu_explanation(&app);
            let lines = Paragraph::new(shown)
                .wrap(Wrap { trim: false })
                .line_count(pane_width);
            let visible = menu_explanation_viewport_height(&app);
            assert_eq!(
                visible, 10,
                "área visível esperada para cabeçalho de altura 5 em 19 linhas"
            );
            for _ in 0..lines {
                app.key(KeyCode::PageDown);
            }
            assert_eq!(
                app.menu_scroll,
                lines.saturating_sub(visible),
                "rolagem da estratégia {index} não atingiu o limite esperado"
            );
            let mut terminal = Terminal::new(TestBackend::new(55, 19)).unwrap();
            terminal.draw(|f| draw(f, &app)).unwrap();
            let buffer = terminal.backend().buffer();
            let rendered: String = (0..19)
                .flat_map(|y| (0..55).map(move |x| buffer[(x, y)].symbol()))
                .collect::<String>()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();
            let last_word: String = SECTION_EXPLANATIONS[index]
                .split_whitespace()
                .rev()
                .take(1)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<String>()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();
            assert!(
                rendered.contains(&last_word),
                "última palavra '{last_word}' não ficou visível para estratégia {index} com cabeçalho quebrado: offset {}",
                app.menu_scroll
            );
        }
    }
    #[test]
    fn scroll_viewports_adjust_to_dynamic_header_height() {
        let short_path = "dados.xlsx";
        let long_path =
            "/opt/fixture/program/example/example-example-example-example-workbook.xlsx";
        let mut app_short = App::new(PathBuf::from(short_path));
        app_short.terminal_width = 55;
        app_short.terminal_height = 19;

        let mut app_long = App::new(PathBuf::from(long_path));
        app_long.terminal_width = 55;
        app_long.terminal_height = 19;

        assert_eq!(
            header_height(
                &display_header(&app_short, Rect::new(0, 0, 55, 19)),
                Rect::new(0, 0, 55, 19)
            ),
            4
        );
        assert_eq!(
            header_height(
                &display_header(&app_long, Rect::new(0, 0, 55, 19)),
                Rect::new(0, 0, 55, 19)
            ),
            5
        );

        assert_eq!(menu_explanation_viewport_height(&app_short), 11);
        assert_eq!(menu_explanation_viewport_height(&app_long), 10);

        assert_eq!(series_viewport_height(&app_short), 11);
        assert_eq!(series_viewport_height(&app_long), 10);

        assert_eq!(result_details_viewport_height(&app_short), 2);
        assert_eq!(result_details_viewport_height(&app_long), 1);
    }
    #[test]
    fn result_details_remain_readable_under_tall_headers() {
        for length in [120, 180, 240, 300, 400] {
            let mut app = App::new(PathBuf::from(format!("/{}.xlsx", "a".repeat(length))));
            app.terminal_width = 55;
            app.terminal_height = 19;
            app.screen = Screen::Result;
            app.view = Some(View {
                title: "Paridade".into(),
                sections: vec![Section {
                    title: "Pares".into(),
                    bins: [(4, 1)].into_iter().collect(),
                    scale: 1,
                    values: Some(vec![Some(4)]),
                    lines: vec!["FINAL-DO-RESULTADO".into()],
                }],
            });
            for _ in 0..100 {
                app.key(KeyCode::Down);
            }
            let mut terminal = Terminal::new(TestBackend::new(55, 19)).unwrap();
            terminal.draw(|f| draw(f, &app)).unwrap();
            let buffer = terminal.backend().buffer();
            let shown: String = (0..19)
                .flat_map(|y| (0..55).map(move |x| buffer[(x, y)].symbol()))
                .collect();
            assert!(
                shown.contains("FINAL-DO-RESULTADO"),
                "result hidden for path length {length}, viewport {}",
                result_details_viewport_height(&app)
            );
        }
    }

    #[test]
    fn result_omits_source_and_digest_and_menu_uses_snapshot_status() {
        let path = crate::test_fixture::write_synthetic_workbook(40);
        let snapshot = Arc::new(data::load(&path).expect("synthetic workbook must load"));
        let view = presentation::build(0, &snapshot.draws, |_, _| true).unwrap();
        let mut app = App::new(path.clone());
        app.snapshot = Some(snapshot.clone());
        assert!(screen_header(&app).contains(&path.display().to_string()));
        match snapshot.freshness {
            Freshness::Current { .. } => assert!(screen_header(&app).contains("Verificada hoje")),
            Freshness::Stale { .. } => assert!(screen_header(&app).contains("desatualizada")),
            Freshness::Unknown => assert!(screen_header(&app).contains("desconhecida")),
        }
        let mut stale = (*snapshot).clone();
        stale.freshness = Freshness::Stale {
            checked: chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
        };
        app.snapshot = Some(Arc::new(stale));
        assert!(screen_header(&app).contains("possivelmente desatualizada"));
        let mut unknown = (*snapshot).clone();
        unknown.freshness = Freshness::Unknown;
        app.snapshot = Some(Arc::new(unknown));
        assert!(screen_header(&app).contains("atualização desconhecida"));
        app.snapshot = Some(snapshot);
        app.view = Some(view);
        app.screen = Screen::Result;
        let mut terminal = Terminal::new(TestBackend::new(140, 40)).unwrap();
        terminal.draw(|f| draw(f, &app)).unwrap();
        let shown = format!("{:?}", terminal.backend().buffer());
        assert!(shown.contains("ChuteGPT — Análise Histórica Descritiva | Fonte:"));
        assert!(!shown.contains("SHA-256"));
        assert!(shown.contains("Observações válidas"));
        let rlines = result_lines(&app, app.active_section().unwrap());
        assert!(!rlines.iter().any(|l| l.starts_with("Fonte:")));
        assert!(!rlines.iter().any(|l| l.starts_with("SHA-256:")));
        app.key(KeyCode::Esc);
        assert!(matches!(app.screen, Screen::Menu));
        assert!(screen_header(&app).contains(&path.display().to_string()));
        let _ = std::fs::remove_file(&path);
    }
    #[test]
    fn failed_reload_clears_previously_verified_snapshot() {
        let fixture = crate::test_fixture::write_synthetic_workbook(40);
        let path = fixture.clone();
        let mut app = App::new(path);
        app.snapshot = Some(Arc::new(
            data::load(&fixture).expect("synthetic workbook must load"),
        ));
        let (tx, rx) = mpsc::sync_channel(1);
        tx.send(Message::Loaded(Err("arquivo inválido".into())))
            .unwrap();
        app.receiver = Some(rx);
        app.poll();
        assert!(app.snapshot.is_none());
        app.key(KeyCode::Esc);
        assert!(screen_header(&app).contains("falha na verificação"));
        assert!(!screen_header(&app).contains("ainda não verificada"));
        assert!(menu_explanation(&app).contains("arquivo inválido"));
        let _ = std::fs::remove_file(&fixture);
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
        assert!(screen_header(&app).contains("falha na verificação"));
        assert!(!screen_header(&app).contains("ainda não verificada"));
        assert!(menu_explanation(&app).contains("A verificação desta fonte falhou"));
        assert!(matches!(app.screen, Screen::Menu));
    }
    #[test]
    fn explanation_omits_redundant_source_path_and_header_includes_it() {
        let mut app = App::new(PathBuf::from("/caminho/muito/longo/planilha.xlsx"));
        app.menu = 21;
        let description = menu_explanation(&app);
        assert!(description.contains(STRATEGIES[21]));
        assert!(!description.contains("Fonte:"));
        assert!(!description.contains("/caminho/muito/longo/planilha.xlsx"));
        assert!(screen_header(&app).contains("/caminho/muito/longo/planilha.xlsx"));
    }
    #[test]
    fn screen_header_is_one_line_in_the_requested_order() {
        let fixture = crate::test_fixture::write_synthetic_workbook(24);
        let mut snapshot = data::load(&fixture).expect("synthetic workbook must load");
        let today = chrono::Local::now().date_naive();
        snapshot.freshness = Freshness::Current { checked: today };
        let app_path = fixture.to_string_lossy().into_owned();
        let mut app = App::new(PathBuf::from(&app_path));
        app.snapshot = Some(Arc::new(snapshot));
        let expected = format!(
            "ChuteGPT — Análise Histórica Descritiva | Fonte: {app_path} | Verificada hoje ({})",
            today.format("%d/%m/%Y")
        );
        assert_eq!(screen_header(&app), expected);
        let mut terminal = Terminal::new(TestBackend::new(200, 24)).unwrap();
        terminal.draw(|f| draw(f, &app)).unwrap();
        assert!(format!("{:?}", terminal.backend().buffer()).contains(&expected));
        let _ = std::fs::remove_file(&fixture);
    }
    #[test]
    fn preflight_updates_menu_status_without_starting_analysis() {
        let fixture = crate::test_fixture::write_synthetic_workbook(24);
        let mut app = App::new(fixture.clone());
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
        assert!(!screen_header(&app).contains("ainda não verificada"));
        let _ = std::fs::remove_file(&fixture);
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
    fn synthetic_snapshot_renders_each_section_and_available_series() {
        let fixture = crate::test_fixture::write_synthetic_workbook(100);
        let path = fixture.clone();
        let snapshot = Arc::new(data::load(&path).expect("synthetic workbook must load"));
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
        let _ = std::fs::remove_file(&fixture);
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
