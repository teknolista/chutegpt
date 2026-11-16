use crate::data::Draw;
use chrono::Datelike;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Cancelled,
    Numeric,
}
#[derive(Debug, Clone, PartialEq)]
pub struct HistoricalRow {
    pub frequency_hits: Option<usize>,
    pub delay_hits: Option<usize>,
    pub never_seen: Vec<u8>,
    pub periodicity_all: Vec<u64>,
    pub periodicity_drawn: Vec<u64>,
    pub periodicity_measures_all: Option<[Option<i64>; 7]>,
    pub periodicity_measures_drawn: Option<[Option<i64>; 7]>,
    pub periodicity_ge_4: Option<usize>,
    pub periodicity_missing_drawn: usize,
    pub delays_all: Option<Vec<u64>>,
    pub delays_drawn: Option<Vec<u64>>,
    pub delay_measures_all: Option<[Option<i64>; 7]>,
    pub delay_measures_drawn: Option<[Option<i64>; 7]>,
    pub delay_ge_5: Option<usize>,
    pub frequency_delay_dispersion: Option<u64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WeekdayTier {
    pub known_winners: usize,
    pub winning_draws: usize,
    pub winner_sum: u64,
    pub known_rates: usize,
    pub rate_sum_cents: u64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Weekly {
    pub draws: [usize; 7],
    pub tiers: [[WeekdayTier; 7]; 5],
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventSeries {
    pub occurrences: usize,
    pub closed_waits: Vec<usize>,
    pub missing_waits: usize,
    pub open_delay: Option<usize>,
    pub unknown: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrizeMonths {
    pub months: [EventSeries; 12],
    pub principal: EventSeries,
}

fn round100(x: f64) -> Result<i64, Error> {
    let x = (x * 100.0).round_ties_even();
    if !x.is_finite() || x < i64::MIN as f64 || x >= i64::MAX as f64 {
        Err(Error::Numeric)
    } else {
        Ok(x as i64)
    }
}
fn measures(v: &[u64]) -> Result<Option<[Option<i64>; 7]>, Error> {
    if v.is_empty() {
        return Ok(None);
    }
    let n = v.len() as f64;
    let mean = v.iter().map(|&x| x as f64).sum::<f64>() / n;
    let min = *v.iter().min().ok_or(Error::Numeric)?;
    let max = *v.iter().max().ok_or(Error::Numeric)?;
    let roots = v.iter().map(|&x| (x as f64).sqrt()).sum::<f64>() / n;
    let rms = (v.iter().map(|&x| (x as f64).powi(2)).sum::<f64>() / n).sqrt();
    let mad = v.iter().map(|&x| (x as f64 - mean).abs()).sum::<f64>() / n;
    let sd = (v.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / n).sqrt();
    Ok(Some([
        Some(round100((max - min) as f64)?),
        Some(round100(mean)?),
        Some(round100(roots)?),
        Some(round100(rms)?),
        Some(round100(mad)?),
        Some(round100(sd)?),
        if mean == 0.0 {
            None
        } else {
            Some(round100(100.0 * sd / mean)?)
        },
    ]))
}
fn population_sd(v: &[u64]) -> f64 {
    let n = v.len() as f64;
    let mean = v.iter().map(|&x| x as f64).sum::<f64>() / n;
    (v.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / n).sqrt()
}
fn top(values: &[Option<u64>; 31]) -> [bool; 31] {
    let mut sorted: Vec<u64> = values.iter().flatten().copied().collect();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    let Some(&limit) = sorted.get(sorted.len().min(10).saturating_sub(1)) else {
        return [false; 31];
    };
    std::array::from_fn(|i| values[i].is_some_and(|v| v >= limit))
}

pub fn frequency_and_delay<F>(draws: &[Draw], mut progress: F) -> Result<Vec<HistoricalRow>, Error>
where
    F: FnMut(usize, usize) -> bool,
{
    if !progress(0, draws.len()) {
        return Err(Error::Cancelled);
    }
    let mut counts = [0_u64; 31];
    let mut last = [None; 31];
    let mut rows = Vec::with_capacity(draws.len());
    for (i, draw) in draws.iter().enumerate() {
        let mut row = HistoricalRow {
            frequency_hits: None,
            delay_hits: None,
            never_seen: counts
                .iter()
                .enumerate()
                .filter_map(|(j, &count)| (count == 0).then_some((j + 1) as u8))
                .collect(),
            periodicity_all: Vec::new(),
            periodicity_drawn: Vec::new(),
            periodicity_measures_all: None,
            periodicity_measures_drawn: None,
            periodicity_ge_4: None,
            periodicity_missing_drawn: 7,
            delays_all: None,
            delays_drawn: None,
            delay_measures_all: None,
            delay_measures_drawn: None,
            delay_ge_5: None,
            frequency_delay_dispersion: None,
        };
        if i > 0 {
            let frequencies = std::array::from_fn(|j| Some(counts[j]));
            let delays: [Option<u64>; 31] =
                std::array::from_fn(|j| last[j].map(|p| (i - 1 - p) as u64));
            let freq_top = top(&frequencies);
            let delay_top = top(&delays);
            row.frequency_hits = Some(
                draw.numbers
                    .iter()
                    .filter(|&&n| freq_top[usize::from(n - 1)])
                    .count(),
            );
            row.delay_hits = Some(
                draw.numbers
                    .iter()
                    .filter(|&&n| delay_top[usize::from(n - 1)])
                    .count(),
            );
            let periodicity: [Option<u64>; 31] =
                std::array::from_fn(|j| (i as u64).checked_div(counts[j]));
            row.periodicity_all = periodicity.iter().flatten().copied().collect();
            row.periodicity_drawn = draw
                .numbers
                .iter()
                .filter_map(|&n| periodicity[usize::from(n - 1)])
                .collect();
            row.periodicity_missing_drawn = 7 - row.periodicity_drawn.len();
            row.periodicity_ge_4 = Some(row.periodicity_drawn.iter().filter(|&&v| v >= 4).count());
            row.periodicity_measures_all = measures(&row.periodicity_all)?;
            row.periodicity_measures_drawn = measures(&row.periodicity_drawn)?;
            let all: Vec<u64> = delays.iter().flatten().copied().collect();
            let drawn: Vec<u64> = draw
                .numbers
                .iter()
                .filter_map(|&n| delays[usize::from(n - 1)])
                .collect();
            row.delay_measures_all = measures(&all)?;
            row.delay_measures_drawn = measures(&drawn)?;
            row.delays_all = Some(all);
            row.delays_drawn = Some(drawn.clone());
            if delays.iter().all(Option::is_some) {
                row.delay_ge_5 = Some(drawn.iter().filter(|&&v| v >= 5).count());
                let freq: Vec<u64> = draw
                    .numbers
                    .iter()
                    .map(|&n| counts[usize::from(n - 1)])
                    .collect();
                let dispersion = (population_sd(&freq) * population_sd(&drawn))
                    .sqrt()
                    .round_ties_even();
                if !dispersion.is_finite() || dispersion < 0.0 || dispersion >= u64::MAX as f64 {
                    return Err(Error::Numeric);
                }
                row.frequency_delay_dispersion = Some(dispersion as u64);
            }
        }
        rows.push(row);
        for &number in &draw.numbers {
            let j = usize::from(number - 1);
            counts[j] = counts[j].checked_add(1).ok_or(Error::Numeric)?;
            last[j] = Some(i);
        }
        if !progress(i + 1, draws.len()) {
            return Err(Error::Cancelled);
        }
    }
    Ok(rows)
}

pub fn mean_of_roots<F>(draws: &[Draw], mut progress: F) -> Result<Vec<u64>, Error>
where
    F: FnMut(usize, usize) -> bool,
{
    if !progress(0, draws.len()) {
        return Err(Error::Cancelled);
    }
    let mut values = Vec::with_capacity(draws.len());
    for (i, draw) in draws.iter().enumerate() {
        let v = (draw
            .numbers
            .iter()
            .map(|&n| f64::from(n).sqrt())
            .sum::<f64>()
            / 7.0)
            .round_ties_even();
        if !v.is_finite() || v < 0.0 || v >= u64::MAX as f64 {
            return Err(Error::Numeric);
        }
        values.push(v as u64);
        if !progress(i + 1, draws.len()) {
            return Err(Error::Cancelled);
        }
    }
    Ok(values)
}

pub fn weekly<F>(draws: &[Draw], mut progress: F) -> Result<Weekly, Error>
where
    F: FnMut(usize, usize) -> bool,
{
    if !progress(0, draws.len()) {
        return Err(Error::Cancelled);
    }
    let mut out = Weekly {
        draws: [0; 7],
        tiers: std::array::from_fn(|_| std::array::from_fn(|_| WeekdayTier::default())),
    };
    for (i, draw) in draws.iter().enumerate() {
        let day = draw.date.weekday().num_days_from_monday() as usize;
        out.draws[day] += 1;
        for (tier, p) in draw.prizes.iter().enumerate() {
            let cell = &mut out.tiers[tier][day];
            if let Some(w) = p.winners {
                cell.known_winners += 1;
                if w > 0 {
                    cell.winning_draws += 1;
                    cell.winner_sum = cell.winner_sum.checked_add(w).ok_or(Error::Numeric)?;
                }
            }
            if let Some(c) = p.cents {
                cell.known_rates += 1;
                cell.rate_sum_cents = cell.rate_sum_cents.checked_add(c).ok_or(Error::Numeric)?;
            }
        }
        if !progress(i + 1, draws.len()) {
            return Err(Error::Cancelled);
        }
    }
    Ok(out)
}

fn series(states: &[Option<bool>]) -> EventSeries {
    let mut events = 0;
    let mut closed = Vec::new();
    let mut missing = 0;
    let mut last = None;
    let mut unknown_since = false;
    for (i, state) in states.iter().enumerate() {
        match state {
            None => unknown_since = true,
            Some(true) => {
                events += 1;
                if let Some(prev) = last {
                    if unknown_since {
                        missing += 1
                    } else {
                        closed.push(i - prev - 1);
                    }
                }
                last = Some(i);
                unknown_since = false;
            }
            Some(false) => {}
        }
    }
    EventSeries {
        occurrences: events,
        closed_waits: closed,
        missing_waits: missing,
        open_delay: if unknown_since {
            None
        } else {
            last.map(|p| states.len() - 1 - p)
        },
        unknown: states.iter().filter(|v| v.is_none()).count(),
    }
}
pub fn prize_months<F>(draws: &[Draw], mut progress: F) -> Result<PrizeMonths, Error>
where
    F: FnMut(usize, usize) -> bool,
{
    if !progress(0, draws.len()) {
        return Err(Error::Cancelled);
    }
    let mut months: [Vec<Option<bool>>; 12] =
        std::array::from_fn(|_| Vec::with_capacity(draws.len()));
    let mut prizes = Vec::with_capacity(draws.len());
    for (i, d) in draws.iter().enumerate() {
        for (j, m) in months.iter_mut().enumerate() {
            m.push(Some(usize::from(d.month) == j + 1));
        }
        prizes.push(d.prizes[0].winners.map(|v| v > 0));
        if !progress(i + 1, draws.len()) {
            return Err(Error::Cancelled);
        }
    }
    Ok(PrizeMonths {
        months: std::array::from_fn(|i| series(&months[i])),
        principal: series(&prizes),
    })
}
pub fn histogram(values: impl IntoIterator<Item = Option<u64>>) -> BTreeMap<u64, usize> {
    let mut out = BTreeMap::new();
    for v in values.into_iter().flatten() {
        *out.entry(v).or_default() += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Prize;
    use chrono::NaiveDate;
    fn d(id: u32, nums: [u8; 7], month: u8, day: u32, w: Option<u64>, c: Option<u64>) -> Draw {
        Draw {
            contest: id,
            date: NaiveDate::from_ymd_opt(2026, 11, day).unwrap(),
            numbers: nums,
            month,
            prizes: std::array::from_fn(|i| {
                if i == 0 {
                    Prize {
                        winners: w,
                        cents: c,
                    }
                } else {
                    Prize {
                        winners: None,
                        cents: None,
                    }
                }
            }),
        }
    }
    #[test]
    fn prior_only_rank_ties_periodicity_and_coverage() {
        let data = [
            d(1, [1, 2, 3, 4, 5, 6, 7], 1, 2, Some(1), Some(100)),
            d(2, [8, 9, 10, 11, 12, 13, 14], 1, 3, Some(0), None),
            d(3, [15, 16, 17, 18, 19, 20, 21], 2, 4, Some(1), Some(200)),
            d(4, [22, 23, 24, 25, 26, 27, 28], 1, 5, None, None),
            d(5, [25, 26, 27, 28, 29, 30, 31], 1, 6, Some(0), None),
            d(6, [1, 2, 3, 4, 5, 6, 7], 1, 7, Some(0), None),
        ];
        let rows = frequency_and_delay(&data, |_, _| true).unwrap();
        assert_eq!(rows[0].frequency_hits, None);
        assert_eq!(rows[1].delays_all.as_ref().unwrap(), &vec![0; 7]);
        assert_eq!(rows[1].delay_ge_5, None);
        assert_eq!(rows[0].never_seen.len(), 31);
        assert_eq!(rows[0].periodicity_missing_drawn, 7);
        assert_eq!(rows[1].never_seen, (8..=31).collect::<Vec<_>>());
        assert!(rows[5].never_seen.is_empty());
        assert_eq!(rows[4].frequency_delay_dispersion, None);
        assert_eq!(
            (rows[5].frequency_hits, rows[5].delay_hits),
            (Some(7), Some(7))
        );
        assert_eq!(rows[5].periodicity_drawn, vec![5; 7]);
        assert_eq!(rows[5].periodicity_ge_4, Some(7));
        assert_eq!(rows[5].delay_ge_5, Some(0));
        assert_eq!(rows[5].frequency_delay_dispersion, Some(0));
        assert_eq!(
            frequency_and_delay(&data, |done, _| done < 2),
            Err(Error::Cancelled)
        );
        assert_eq!(
            mean_of_roots(
                &[d(1, [1, 2, 3, 10, 20, 30, 31], 1, 2, None, None)],
                |_, _| true
            )
            .unwrap(),
            vec![3]
        );
    }
    #[test]
    fn missing_prizes_weekdays_and_censored_month_series() {
        let numbers = [1, 2, 3, 4, 5, 6, 7];
        let data = [
            d(1, numbers, 1, 2, Some(2), Some(100)),
            d(2, numbers, 1, 9, Some(0), None),
            d(3, numbers, 2, 3, Some(1), Some(200)),
            d(4, numbers, 1, 16, None, None),
        ];
        let w = weekly(&data, |_, _| true).unwrap();
        let monday = &w.tiers[0][0];
        let tuesday = &w.tiers[0][1];
        assert_eq!(
            (
                w.draws[0],
                monday.known_winners,
                monday.winning_draws,
                monday.winner_sum
            ),
            (3, 2, 1, 2)
        );
        assert_eq!((tuesday.winning_draws, tuesday.rate_sum_cents), (1, 200));
        let s = prize_months(&data, |_, _| true).unwrap();
        assert_eq!(s.months[0].closed_waits, vec![0, 1]);
        assert_eq!(s.months[0].open_delay, Some(0));
        let s = series(&[Some(true), Some(false), Some(true), None, Some(false)]);
        assert_eq!(s.closed_waits, vec![1]);
        assert_eq!(s.open_delay, None);
        assert_eq!(s.unknown, 1);
        let s = series(&[Some(true), None, Some(true)]);
        assert_eq!(s.missing_waits, 1);
        assert!(s.closed_waits.is_empty());
    }
}
