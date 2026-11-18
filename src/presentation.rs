use crate::{advanced, analysis, data::Draw, historical, history};
use std::collections::BTreeMap;

pub const STRATEGIES: [&str; 22] = [
    "Paridade",
    "Repetência",
    "Recorrência histórica",
    "Sequências adjacentes",
    "Maior sequência consecutiva",
    "Soma das dezenas",
    "Raiz digital",
    "Dispersão das dezenas",
    "Espaçamento",
    "Decenário",
    "Colunas",
    "Matriz",
    "Ciclos de cobertura",
    "Ordinal",
    "Dezenas individuais",
    "Mês da sorte",
    "Frequência e ausência",
    "Distância",
    "Média das raízes",
    "Dispersão frequência e atraso",
    "Premiação por dia da semana",
    "Séries de mês e premiação",
];

pub const MONTHS: [&str; 12] = [
    "Janeiro",
    "Fevereiro",
    "Março",
    "Abril",
    "Maio",
    "Junho",
    "Julho",
    "Agosto",
    "Setembro",
    "Outubro",
    "Novembro",
    "Dezembro",
];

pub const DEFINITIONS: [&str; 22] = [
    "Distribuição dos concursos por número de dezenas pares e ímpares.",
    "Interseção com o concurso imediatamente anterior; o primeiro é excluído.",
    "Maior interseção com qualquer concurso anterior; não usa concursos futuros.",
    "Quantidade de pares adjacentes de dezenas consecutivas em cada concurso.",
    "Maior quantidade de ligações consecutivas em cada concurso.",
    "Soma das sete dezenas em cada concurso.",
    "Raiz digital da soma; categorização descritiva.",
    "Medidas de dispersão das sete dezenas em cada concurso.",
    "Diferenças entre dezenas adjacentes e suas medidas de dispersão.",
    "Ocupação das linhas 01–10, 11–20, 21–30 e 31.",
    "Ocupação das dez colunas identificadas pelo último algarismo.",
    "Máximos de ocupação por linha e coluna e sua soma.",
    "Ciclos independentes até a cobertura de 27 ou 31 dezenas.",
    "Posição lexicográfica entre as combinações e diferenças entre concursos.",
    "Ocorrências e atraso atual das dezenas 01 a 31.",
    "Ocorrências e atraso atual de cada mês da sorte.",
    "Frequência e atraso medidos somente com os concursos anteriores.",
    "Diferença entre a maior e a menor dezena de cada concurso.",
    "Média das raízes quadradas das dezenas, arredondada; não é mediana.",
    "Raiz do produto dos desvios populacionais de frequência e atraso anteriores.",
    "Premiações e ganhadores por dia da semana e faixa, com dados ausentes.",
    "Ocorrências e intervalos fechados de meses e premiação principal.",
];

#[derive(Debug, Clone)]
pub struct Section {
    pub title: String,
    pub bins: BTreeMap<i64, usize>,
    pub scale: i64,
    pub values: Option<Vec<Option<i64>>>,
    pub lines: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct View {
    pub title: String,
    pub sections: Vec<Section>,
}
fn discrete(title: impl Into<String>, values: Vec<Option<i64>>, scale: i64) -> Section {
    let mut bins = BTreeMap::new();
    for v in values.iter().flatten() {
        *bins.entry(*v).or_default() += 1;
    }
    Section {
        title: title.into(),
        bins,
        scale,
        values: Some(values),
        lines: Vec::new(),
    }
}
fn pad(section: &mut Section, range: std::ops::RangeInclusive<i64>) {
    for value in range {
        section.bins.entry(value).or_insert(0);
    }
}
fn text(title: impl Into<String>, lines: Vec<String>) -> Section {
    Section {
        title: title.into(),
        bins: BTreeMap::new(),
        scale: 1,
        values: None,
        lines,
    }
}
fn cast(v: u64) -> Result<i64, String> {
    i64::try_from(v).map_err(|_| "Valor numérico excede o limite".into())
}
/// Render a scaled integer. Scale 1 means counts and scale 100 means stored
/// hundredths; no other scale exists in the analysis model. An unsupported
/// scale is marked instead of silently printing a raw scaled integer.
pub fn format_value(v: i64, scale: i64) -> String {
    match scale {
        1 => v.to_string(),
        100 => {
            let sign = if v < 0 { "-" } else { "" };
            let magnitude = v.unsigned_abs();
            format!("{sign}{}.{:02}", magnitude / 100, magnitude % 100)
        }
        _ => format!("{v} [escala não suportada: {scale}]"),
    }
}
fn month_name(category: u8) -> &'static str {
    usize::from(category)
        .checked_sub(1)
        .and_then(|index| MONTHS.get(index))
        .copied()
        .unwrap_or("Mês desconhecido")
}
fn ordinal(name: &str, values: Vec<Option<u64>>) -> Result<Section, String> {
    Ok(discrete(
        name,
        values
            .into_iter()
            .map(|v| v.map(cast).transpose())
            .collect::<Result<Vec<_>, _>>()?,
        1,
    ))
}
fn add_advanced(out: &mut Vec<Section>, result: advanced::ResultSet, index: usize, draws: &[Draw]) {
    if !result.totals.is_empty() {
        out.push(text(
            "Totais por grupo",
            result
                .totals
                .into_iter()
                .map(|(name, total)| format!("{name}: {total}"))
                .collect(),
        ));
    }
    for m in result.metrics {
        let pooled = m.pooled;
        let mut section = discrete(m.name, m.values, i64::from(m.scale));
        if !pooled && let Some(values) = &section.values {
            section.lines = draws
                .iter()
                .zip(values)
                .map(|(draw, value)| {
                    let text = value
                        .map_or_else(|| "indisponível".into(), |v| format_value(v, section.scale));
                    format!("Concurso {}: {}", draw.contest, text)
                })
                .collect();
        }
        if pooled {
            section.values = None;
            pad(&mut section, 1..=25);
        } else if index == 9 || index == 10 {
            pad(&mut section, 0..=7);
        } else if index == 11 {
            let end = if section.title == "Soma dos máximos" {
                14
            } else {
                7
            };
            pad(&mut section, 0..=end);
        } else if index == 17 {
            pad(&mut section, 6..=30);
        }
        out.push(section);
    }
}
fn history_error(error: history::Error) -> String {
    match error {
        history::Error::Cancelled => "Cancelado".into(),
        history::Error::InvalidDraw => "Concurso inválido na planilha".into(),
    }
}
fn map_simple(index: usize) -> analysis::Strategy {
    use analysis::Strategy::*;
    [
        Parity,
        Repeat,
        Recurrence,
        Adjacent,
        LongestRun,
        Sum,
        DigitalRoot,
    ][index]
}
fn map_advanced(index: usize) -> advanced::Strategy {
    use advanced::Strategy::*;
    match index {
        7 => Dispersion,
        8 => Gaps,
        9 => Decades,
        10 => Columns,
        11 => Matrix,
        17 => Distance,
        _ => unreachable!(),
    }
}

pub fn build<F>(index: usize, draws: &[Draw], mut progress: F) -> Result<View, String>
where
    F: FnMut(usize, usize) -> bool,
{
    if index >= STRATEGIES.len() {
        return Err("Estratégia desconhecida".into());
    }
    let mut sections = Vec::new();
    match index {
        0..=6 => {
            let h =
                analysis::calculate(draws, map_simple(index), &mut progress).map_err(|error| {
                    match error {
                        analysis::Error::Cancelled => "Cancelado".to_string(),
                        analysis::Error::InvalidDraw => "Concurso inválido na planilha".to_string(),
                    }
                })?;
            let mut sec = discrete(
                h.title,
                h.observations
                    .into_iter()
                    .map(|v| v.map(i64::from))
                    .collect(),
                1,
            );
            sec.bins = h.bins.into_iter().map(|(k, v)| (i64::from(k), v)).collect();
            sec.lines.push(format!(
                "Concursos elegíveis: {}; indisponíveis: {}",
                h.eligible, h.excluded
            ));
            if index == 0 {
                sec.lines
                    .push("Para cada categoria: ímpares = 7 − pares".into());
            }
            sections.push(sec);
        }
        7..=11 | 17 => {
            let result = advanced::calculate(draws, map_advanced(index), &mut progress)
                .map_err(|e| format!("Falha no cálculo: {e:?}"))?;
            add_advanced(&mut sections, result, index, draws);
        }
        12 => {
            let result = history::cycles(draws, &mut progress).map_err(history_error)?;
            for cycle in [result.partial, result.full] {
                let mut sec = discrete(
                    format!(
                        "Ciclos de {} dezenas — duração em concursos",
                        cycle.threshold
                    ),
                    cycle
                        .completed
                        .iter()
                        .map(|c| Some(c.duration as i64))
                        .collect(),
                    1,
                );
                sec.lines = cycle
                    .completed
                    .iter()
                    .map(|c| {
                        format!(
                            "Concursos {} a {}: {} sorteios, {} dezenas",
                            c.first, c.last, c.duration, c.distinct
                        )
                    })
                    .collect();
                sec.lines.push(match cycle.tail {
                    Some(c) => format!(
                        "Ciclo aberto (não concluído): {} a {}, {} sorteios, {} dezenas",
                        c.first, c.last, c.duration, c.distinct
                    ),
                    None => "Sem ciclo aberto".into(),
                });
                sections.push(sec);
            }
        }
        13 => {
            let x = history::ordinals(draws, &mut progress).map_err(history_error)?;
            let ranks = x
                .ranks
                .iter()
                .map(|&v| Some((10 * (v - 1) / 2_629_575) as i64))
                .collect();
            let mut sec = discrete("Faixa ordinal — 0 a 9", ranks, 1);
            pad(&mut sec, 0..=9);
            sec.lines = (0..10)
                .map(|band| {
                    let lower = (band * 2_629_575_u64).div_ceil(10) + 1;
                    let upper = ((band + 1) * 2_629_575_u64).div_ceil(10);
                    format!("Faixa {band}: ordinais {lower}–{upper}")
                })
                .collect();
            sec.lines.extend(
                x.ranks
                    .iter()
                    .zip(draws)
                    .map(|(rank, d)| format!("Concurso {}: ordinal {}", d.contest, rank)),
            );
            sections.push(sec);
            let bands = x
                .changes
                .iter()
                .map(|v| v.map(|v| (10 * v / 2_629_575).min(9) as i64))
                .collect();
            let mut sec = discrete("Faixa da diferença ordinal — 0 a 9", bands, 1);
            pad(&mut sec, 0..=9);
            sec.lines = (0..10)
                .map(|band| {
                    let lower = (band * 2_629_575_u64).div_ceil(10);
                    let upper = ((band + 1) * 2_629_575_u64).div_ceil(10) - 1;
                    format!("Faixa {band}: diferenças {lower}–{upper}")
                })
                .collect();
            sec.lines
                .extend(x.changes.iter().zip(draws).map(|(change, d)| {
                    format!(
                        "Concurso {}: diferença {}",
                        d.contest,
                        change.map_or_else(|| "indisponível".into(), |v| v.to_string())
                    )
                }));
            sections.push(sec);
        }
        14 | 15 => {
            let months = index == 15;
            let x = history::occurrences(draws, months, &mut progress).map_err(history_error)?;
            let mut summary = text("Ocorrências e atrasos atuais", Vec::new());
            summary.bins = x.iter().map(|v| (i64::from(v.category), v.count)).collect();
            summary.lines = x
                .iter()
                .map(|v| {
                    format!(
                        "{}: {} ocorrências; atraso {}",
                        if months {
                            month_name(v.category).to_string()
                        } else {
                            format!("{:02}", v.category)
                        },
                        v.count,
                        v.delay
                            .map_or_else(|| "nunca observado".into(), |d| d.to_string())
                    )
                })
                .collect();
            sections.push(summary);
            for v in x {
                let values = draws
                    .iter()
                    .map(|d| {
                        Some(i64::from(if months {
                            d.month == v.category
                        } else {
                            d.numbers.contains(&v.category)
                        }))
                    })
                    .collect();
                let mut sec = discrete(
                    format!(
                        "Série {} — 0 ausente / 1 presente",
                        if months {
                            month_name(v.category).to_string()
                        } else {
                            format!("{:02}", v.category)
                        }
                    ),
                    values,
                    1,
                );
                sec.lines.push(format!(
                    "Ocorrências: {}; atraso atual: {}",
                    v.count,
                    v.delay
                        .map_or_else(|| "nunca observado".into(), |d| d.to_string())
                ));
                sections.push(sec);
            }
        }
        16 | 19 => {
            let rows = historical::frequency_and_delay(draws, &mut progress)
                .map_err(|e| format!("Falha no cálculo histórico: {e:?}"))?;
            if index == 19 {
                sections.push(ordinal(
                    "Dispersão entre frequência e atraso",
                    rows.iter().map(|r| r.frequency_delay_dispersion).collect(),
                )?);
            } else {
                for (name, values) in [
                    (
                        "Dezenas no topo de frequência",
                        rows.iter()
                            .map(|r| r.frequency_hits.map(|v| v as i64))
                            .collect(),
                    ),
                    (
                        "Dezenas no topo de atraso",
                        rows.iter()
                            .map(|r| r.delay_hits.map(|v| v as i64))
                            .collect(),
                    ),
                    (
                        "Periodicidade ≥ 4 entre sorteadas",
                        rows.iter()
                            .map(|r| r.periodicity_ge_4.map(|v| v as i64))
                            .collect(),
                    ),
                    (
                        "Atraso ≥ 5 entre sorteadas",
                        rows.iter()
                            .map(|r| r.delay_ge_5.map(|v| v as i64))
                            .collect(),
                    ),
                ] {
                    let mut sec = discrete(name, values, 1);
                    pad(&mut sec, 0..=7);
                    sections.push(sec);
                }
                let measure_names = [
                    "Amplitude",
                    "Média",
                    "Média das raízes",
                    "RMS",
                    "Desvio absoluto médio",
                    "Desvio padrão populacional",
                    "Coeficiente de variação (%)",
                ];
                for group in 0..4 {
                    for (i, name) in measure_names.iter().enumerate() {
                        let title = match group {
                            0 => "Periodicidade — todas",
                            1 => "Periodicidade — sorteadas",
                            2 => "Atraso — todas",
                            _ => "Atraso — sorteadas",
                        };
                        let vals = rows
                            .iter()
                            .map(|r| {
                                match group {
                                    0 => r.periodicity_measures_all,
                                    1 => r.periodicity_measures_drawn,
                                    2 => r.delay_measures_all,
                                    _ => r.delay_measures_drawn,
                                }
                                .and_then(|v| v[i])
                            })
                            .collect();
                        sections.push(discrete(format!("{title}: {name}"), vals, 100));
                    }
                }
                let mut lines = Vec::new();
                for (draw, row) in draws.iter().zip(&rows) {
                    lines.push(format!("Concurso {}: dezenas nunca vistas {:?}; distribuição de periodicidades (valor: quantidade), todas {:?}, sorteadas {:?}; não vistas entre sorteadas {}; atrasos todas {:?}; sorteadas {:?}", draw.contest, row.never_seen, historical::histogram(row.periodicity_all.iter().copied().map(Some)), historical::histogram(row.periodicity_drawn.iter().copied().map(Some)), row.periodicity_missing_drawn, row.delays_all, row.delays_drawn));
                }
                sections.push(text("Valores históricos por concurso", lines));
            }
        }
        18 => {
            let vals = historical::mean_of_roots(draws, &mut progress)
                .map_err(|e| format!("Falha no cálculo: {e:?}"))?;
            let mut sec = ordinal(
                "Média das raízes arredondada",
                vals.into_iter().map(Some).collect(),
            )?;
            pad(&mut sec, 1..=6);
            sections.push(sec);
        }
        20 => {
            let w = historical::weekly(draws, &mut progress)
                .map_err(|e| format!("Falha na premiação: {e:?}"))?;
            let names = [
                "7 acertos",
                "6 acertos",
                "5 acertos",
                "4 acertos",
                "Mês da sorte",
            ];
            let days = ["Seg", "Ter", "Qua", "Qui", "Sex", "Sáb", "Dom"];
            for (tier, name) in names.iter().enumerate() {
                let mut lines = Vec::new();
                let winning_total: usize = w.tiers[tier].iter().map(|v| v.winning_draws).sum();
                for (day, label) in days.iter().enumerate() {
                    let c = &w.tiers[tier][day];
                    let share = if winning_total == 0 {
                        "indisponível".into()
                    } else {
                        format!(
                            "{:.2}%",
                            100.0 * c.winning_draws as f64 / winning_total as f64
                        )
                    };
                    let avg_w = if c.winning_draws == 0 {
                        "indisponível".into()
                    } else {
                        format!("{:.2}", c.winner_sum as f64 / c.winning_draws as f64)
                    };
                    let avg_c = if c.known_rates == 0 {
                        "indisponível".into()
                    } else {
                        // The mean prize is rounded half-to-even to whole cents,
                        // the same convention used by the other rounded measures.
                        format!(
                            "R$ {:.2}",
                            (c.rate_sum_cents as f64 / c.known_rates as f64).round_ties_even()
                                / 100.0
                        )
                    };
                    lines.push(format!("{label}: concursos {}; ganhadores conhecidos {}, ausentes {}; premiados {}; parcela {share}; ganhadores {}; média/premiado {avg_w}; rateios conhecidos {}, ausentes {}; rateio médio {avg_c}", w.draws[day], c.known_winners, w.draws[day]-c.known_winners, c.winning_draws, c.winner_sum, c.known_rates, w.draws[day]-c.known_rates));
                }
                let mut sec = text(format!("Premiação: {name}"), lines);
                sec.bins = w.tiers[tier]
                    .iter()
                    .enumerate()
                    .map(|(d, v)| (d as i64, v.winning_draws))
                    .collect();
                sections.push(sec);
                let values = draws
                    .iter()
                    .map(|d| d.prizes[tier].winners.map(|v| if v > 0 { 1 } else { 0 }))
                    .collect();
                sections.push(discrete(
                    format!("Série premiada: {name} — 0 não / 1 sim"),
                    values,
                    1,
                ));
            }
        }
        21 => {
            let s = historical::prize_months(draws, &mut progress)
                .map_err(|e| format!("Falha na série: {e:?}"))?;
            for (idx, value) in s.months.iter().enumerate() {
                let states = draws
                    .iter()
                    .map(|d| Some(i64::from(usize::from(d.month) == idx + 1)))
                    .collect();
                let mut sec = discrete(format!("{} — ocorrência", MONTHS[idx]), states, 1);
                sec.lines.push(format!(
                    "Ocorrências: {}; intervalos concluídos: {:?}; atraso aberto: {}",
                    value.occurrences,
                    value.closed_waits,
                    value
                        .open_delay
                        .map_or_else(|| "indisponível".into(), |v| v.to_string())
                ));
                pad(&mut sec, 0..=1);
                sections.push(sec);
                let waits = value.closed_waits.iter().map(|&v| Some(v as i64)).collect();
                sections.push(discrete(
                    format!("{} — intervalos concluídos", MONTHS[idx]),
                    waits,
                    1,
                ));
            }
            let values = draws
                .iter()
                .map(|d| d.prizes[0].winners.map(|v| if v > 0 { 1 } else { 0 }))
                .collect();
            let mut sec = discrete("Premiação principal — ocorrência", values, 1);
            sec.lines.push(format!("Ocorrências: {}; intervalos concluídos: {:?}; intervalos desconhecidos: {}; registros ausentes: {}; atraso aberto: {}",s.principal.occurrences,s.principal.closed_waits,s.principal.missing_waits,s.principal.unknown,s.principal.open_delay.map_or_else(||"indisponível".into(),|v|v.to_string())));
            pad(&mut sec, 0..=1);
            sections.push(sec);
            let waits = s
                .principal
                .closed_waits
                .iter()
                .map(|&v| Some(v as i64))
                .collect();
            sections.push(discrete(
                "Premiação principal — intervalos concluídos",
                waits,
                1,
            ));
        }
        _ => return Err("Estratégia desconhecida".into()),
    }
    Ok(View {
        title: STRATEGIES[index].into(),
        sections,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_strategy_rejects_invalid_public_draws() {
        let valid = Draw {
            contest: 1,
            date: chrono::NaiveDate::from_ymd_opt(2018, 5, 19).unwrap(),
            numbers: [1, 2, 3, 4, 5, 6, 7],
            month: 1,
            prizes: std::array::from_fn(|_| crate::data::Prize {
                winners: None,
                cents: None,
            }),
        };
        let mut cases = Vec::new();
        for numbers in [
            [0, 2, 3, 4, 5, 6, 7],
            [1, 2, 3, 4, 5, 6, 32],
            [2, 1, 3, 4, 5, 6, 7],
            [1, 1, 3, 4, 5, 6, 7],
        ] {
            let mut draw = valid.clone();
            draw.numbers = numbers;
            cases.push(draw);
        }
        for month in [0, 13] {
            let mut draw = valid.clone();
            draw.month = month;
            cases.push(draw);
        }
        for draw in cases {
            for index in 0..STRATEGIES.len() {
                let error = build(index, std::slice::from_ref(&draw), |_, _| true).unwrap_err();
                assert!(
                    error.contains("inválido") || error.contains("InvalidDraw"),
                    "strategy {index}: {error}"
                );
            }
        }
        for index in 0..=6 {
            assert_eq!(
                build(index, std::slice::from_ref(&valid), |_, _| false).unwrap_err(),
                "Cancelado"
            );
        }
    }
    #[test]
    fn result_views_include_zero_bins_and_wait_histograms() {
        use crate::data::Prize;
        use chrono::NaiveDate;
        let draws: Vec<_> = [1, 1, 2, 1]
            .into_iter()
            .enumerate()
            .map(|(i, month)| Draw {
                contest: (i + 1) as u32,
                date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                numbers: [1, 2, 3, 10, 20, 30, 31],
                month,
                prizes: std::array::from_fn(|_| Prize {
                    winners: None,
                    cents: None,
                }),
            })
            .collect();
        let parity = build(0, &draws, |_, _| true).unwrap();
        assert_eq!(parity.sections[0].bins.len(), 8);
        assert_eq!(parity.sections[0].bins[&4], 4);
        let dispersion = build(7, &draws, |_, _| true).unwrap();
        assert!(
            dispersion.sections[0]
                .lines
                .iter()
                .any(|line| line.starts_with("Concurso 1:"))
        );
        assert!(
            dispersion.sections[0]
                .lines
                .iter()
                .any(|line| line.starts_with("Concurso 2:"))
        );
        let ordinal = build(13, &draws, |_, _| true).unwrap();
        assert!(
            ordinal.sections[0]
                .lines
                .iter()
                .any(|x| x.contains("1–262958"))
        );
        assert_eq!(ordinal.sections[0].bins.len(), 10);
        let months = build(21, &draws, |_, _| true).unwrap();
        assert!(months.sections[1].title.contains("intervalos concluídos"));
        assert_eq!(months.sections[1].bins.get(&0), Some(&1));
        assert_eq!(months.sections[1].bins.get(&1), Some(&1));
    }

    #[test]
    fn prize_mean_rounds_half_cent_to_even() {
        use crate::data::Prize;
        use chrono::NaiveDate;
        let draws: Vec<_> = [0, 1]
            .into_iter()
            .enumerate()
            .map(|(i, cents)| Draw {
                contest: (i + 1) as u32,
                date: NaiveDate::from_ymd_opt(2020, 1, 6).unwrap(),
                numbers: [1, 2, 3, 4, 5, 6, 7],
                month: 1,
                prizes: std::array::from_fn(|_| Prize {
                    winners: Some(1),
                    cents: Some(cents),
                }),
            })
            .collect();
        let result = build(20, &draws, |_, _| true).unwrap();
        assert!(result.sections[0].lines[0].contains("rateio médio R$ 0.00"));
    }

    #[test]
    fn pooled_gaps_do_not_claim_per_draw_values() {
        crate::test_fixture::with_synthetic_workbook(2, |fixture| {
            let snapshot = crate::data::load(fixture).expect("synthetic workbook must load");
            let view = build(8, &snapshot.draws, |_, _| true).unwrap();
            let section = view
                .sections
                .iter()
                .find(|s| s.title == "Espaçamentos agrupados")
                .unwrap();
            assert!(section.lines.is_empty());
            assert_eq!(section.bins.values().sum::<usize>(), 12);
        });
    }
    #[test]
    fn scaled_values_keep_their_sign() {
        assert_eq!(format_value(0, 100), "0.00");
        assert_eq!(format_value(-50, 100), "-0.50");
        assert_eq!(format_value(-125, 100), "-1.25");
        assert_eq!(format_value(125, 100), "1.25");
        assert_eq!(format_value(-5, 1), "-5");
        assert_eq!(format_value(5, 1), "5");
        assert_eq!(format_value(1234, 10), "1234 [escala não suportada: 10]");
    }

    #[test]
    fn every_strategy_builds_from_a_synthetic_snapshot() {
        crate::test_fixture::with_synthetic_workbook(24, |fixture| {
            let snapshot = crate::data::load(fixture).expect("synthetic workbook must load");
            assert_eq!(snapshot.draws.len(), 24);
            for (i, name) in STRATEGIES.iter().enumerate() {
                let result = build(i, &snapshot.draws, |_, _| true)
                    .unwrap_or_else(|e| panic!("{name}: {e}"));
                assert_eq!(result.title, *name);
                assert!(!result.sections.is_empty(), "{name}");
            }
        });
    }
}
