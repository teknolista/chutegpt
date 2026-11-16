use crate::data::Draw;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    Dispersion,
    Gaps,
    Decades,
    Columns,
    Matrix,
    Distance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metric {
    pub name: String,
    /// Integer units: 1 for counts/distances; 100 for displayed hundredths.
    pub scale: u32,
    pub values: Vec<Option<i64>>,
    pub bins: BTreeMap<i64, usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultSet {
    pub metrics: Vec<Metric>,
    pub totals: Vec<(String, u64)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Cancelled,
    NonFiniteOrOverflow,
}

fn key(value: f64) -> Result<i64, Error> {
    let scaled = (value * 100.0).round_ties_even();
    if !scaled.is_finite() || scaled < i64::MIN as f64 || scaled >= i64::MAX as f64 {
        Err(Error::NonFiniteOrOverflow)
    } else {
        Ok(scaled as i64)
    }
}

fn measures(input: &[u8]) -> Result<[Option<i64>; 7], Error> {
    let n = input.len() as f64;
    let mean = input.iter().map(|&v| f64::from(v)).sum::<f64>() / n;
    let min = input
        .iter()
        .min()
        .copied()
        .ok_or(Error::NonFiniteOrOverflow)?;
    let max = input
        .iter()
        .max()
        .copied()
        .ok_or(Error::NonFiniteOrOverflow)?;
    let roots = input.iter().map(|&v| f64::from(v).sqrt()).sum::<f64>() / n;
    let rms = (input.iter().map(|&v| f64::from(v).powi(2)).sum::<f64>() / n).sqrt();
    let mad = input
        .iter()
        .map(|&v| (f64::from(v) - mean).abs())
        .sum::<f64>()
        / n;
    let sd = (input
        .iter()
        .map(|&v| (f64::from(v) - mean).powi(2))
        .sum::<f64>()
        / n)
        .sqrt();
    Ok([
        Some(key(f64::from(max - min))?),
        Some(key(mean)?),
        Some(key(roots)?),
        Some(key(rms)?),
        Some(key(mad)?),
        Some(key(sd)?),
        if mean == 0.0 {
            None
        } else {
            Some(key(100.0 * sd / mean)?)
        },
    ])
}

fn metric(name: impl Into<String>, scale: u32, values: Vec<Option<i64>>) -> Metric {
    let mut bins = BTreeMap::new();
    for &v in values.iter().flatten() {
        *bins.entry(v).or_default() += 1;
    }
    Metric {
        name: name.into(),
        scale,
        values,
        bins,
    }
}

fn group_counts(draw: &Draw) -> ([u8; 4], [u8; 10]) {
    let mut rows = [0_u8; 4];
    let mut columns = [0_u8; 10];
    for &n in &draw.numbers {
        rows[usize::from((n - 1) / 10)] += 1;
        columns[usize::from(n % 10)] += 1;
    }
    (rows, columns)
}

/// Returns independently selectable multi-metric historical analyses.
pub fn calculate<F>(draws: &[Draw], strategy: Strategy, mut progress: F) -> Result<ResultSet, Error>
where
    F: FnMut(usize, usize) -> bool,
{
    if !progress(0, draws.len()) {
        return Err(Error::Cancelled);
    }
    let mut rows: Vec<Vec<Option<i64>>> = Vec::with_capacity(draws.len());
    let mut pooled = Vec::new();
    let mut totals = Vec::new();
    for (idx, draw) in draws.iter().enumerate() {
        let (decades, columns) = group_counts(draw);
        let values = match strategy {
            Strategy::Dispersion => measures(&draw.numbers)?.to_vec(),
            Strategy::Gaps => {
                let gaps: Vec<u8> = draw.numbers.windows(2).map(|x| x[1] - x[0]).collect();
                pooled.extend(gaps.iter().map(|&x| Some(i64::from(x))));
                measures(&gaps)?.to_vec()
            }
            Strategy::Decades => {
                let mut v: Vec<_> = decades.into_iter().map(|x| Some(i64::from(x))).collect();
                v.push(Some(i64::from(*decades.iter().max().unwrap_or(&0))));
                v
            }
            Strategy::Columns => {
                let mut v: Vec<_> = columns.into_iter().map(|x| Some(i64::from(x))).collect();
                v.push(Some(i64::from(*columns.iter().max().unwrap_or(&0))));
                v
            }
            Strategy::Matrix => {
                let row = *decades.iter().max().unwrap_or(&0);
                let col = *columns.iter().max().unwrap_or(&0);
                vec![
                    Some(i64::from(row)),
                    Some(i64::from(col)),
                    Some(i64::from(row + col)),
                ]
            }
            Strategy::Distance => vec![Some(i64::from(draw.numbers[6] - draw.numbers[0]))],
        };
        rows.push(values);
        if !progress(idx + 1, draws.len()) {
            return Err(Error::Cancelled);
        }
    }
    let (names, scale): (Vec<String>, u32) = match strategy {
        Strategy::Dispersion | Strategy::Gaps => (
            vec![
                "Amplitude",
                "Média",
                "Média das raízes",
                "RMS",
                "Desvio absoluto médio",
                "Desvio padrão populacional",
                "Coeficiente de variação (%)",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            100,
        ),
        Strategy::Decades => (
            vec!["01–10", "11–20", "21–30", "31", "Ocupação máxima"]
                .into_iter()
                .map(String::from)
                .collect(),
            1,
        ),
        Strategy::Columns => (
            (0..10)
                .map(|n| format!("Final {n}"))
                .chain(std::iter::once("Ocupação máxima".into()))
                .collect(),
            1,
        ),
        Strategy::Matrix => (
            vec![
                "Máximo por linha".into(),
                "Máximo por coluna".into(),
                "Soma dos máximos".into(),
            ],
            1,
        ),
        Strategy::Distance => (vec!["Distância entre extremos".into()], 1),
    };
    let mut metrics = Vec::with_capacity(names.len() + 1);
    for (i, name) in names.into_iter().enumerate() {
        let values: Vec<_> = rows.iter().map(|r| r[i]).collect();
        if matches!(strategy, Strategy::Decades | Strategy::Columns)
            && i < if strategy == Strategy::Decades { 4 } else { 10 }
        {
            totals.push((
                name.clone(),
                values.iter().flatten().map(|v| *v as u64).sum(),
            ));
        }
        metrics.push(metric(name, scale, values));
    }
    if strategy == Strategy::Gaps {
        metrics.push(metric("Espaçamentos agrupados", 1, pooled));
    }
    Ok(ResultSet { metrics, totals })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Prize;
    use chrono::NaiveDate;
    fn sample() -> Draw {
        Draw {
            contest: 1,
            date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            numbers: [1, 2, 3, 10, 20, 30, 31],
            month: 1,
            prizes: std::array::from_fn(|_| Prize {
                winners: None,
                cents: None,
            }),
        }
    }
    fn run(s: Strategy) -> ResultSet {
        calculate(&[sample()], s, |_, _| true).unwrap()
    }
    #[test]
    fn independently_derived_composite_measures() {
        let d = run(Strategy::Dispersion);
        assert_eq!(
            d.metrics.iter().map(|m| m.values[0]).collect::<Vec<_>>(),
            vec![
                Some(3000),
                Some(1386),
                Some(326),
                Some(1842),
                Some(1127),
                Some(1214),
                Some(8757)
            ]
        );
        let g = run(Strategy::Gaps);
        assert_eq!(
            g.metrics
                .iter()
                .take(7)
                .map(|m| m.values[0])
                .collect::<Vec<_>>(),
            vec![
                Some(900),
                Some(500),
                Some(200),
                Some(648),
                Some(400),
                Some(412),
                Some(8246)
            ]
        );
        assert_eq!(
            g.metrics[7].values,
            vec![Some(1), Some(1), Some(7), Some(10), Some(10), Some(1)]
        );
        assert_eq!(g.metrics[7].bins.values().sum::<usize>(), 6);
    }
    #[test]
    fn grouping_boundaries_and_progress() {
        let d = run(Strategy::Decades);
        assert_eq!(
            d.totals.iter().map(|x| x.1).collect::<Vec<_>>(),
            vec![4, 1, 1, 1]
        );
        let c = run(Strategy::Columns);
        assert_eq!(
            c.totals.iter().map(|x| x.1).collect::<Vec<_>>(),
            vec![3, 2, 1, 1, 0, 0, 0, 0, 0, 0]
        );
        let m = run(Strategy::Matrix);
        assert_eq!(
            m.metrics.iter().map(|m| m.values[0]).collect::<Vec<_>>(),
            vec![Some(4), Some(3), Some(7)]
        );
        assert_eq!(run(Strategy::Distance).metrics[0].values, vec![Some(30)]);
        assert_eq!(key(1.125).unwrap(), 112);
        assert_eq!(
            calculate(&[sample()], Strategy::Distance, |done, _| done == 0),
            Err(Error::Cancelled)
        );
        assert!(
            calculate(&[], Strategy::Gaps, |_, _| true)
                .unwrap()
                .metrics
                .iter()
                .all(|m| m.values.is_empty())
        );
    }
}
