use std::collections::BTreeMap;

use crate::data::Draw;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    Parity,
    Repeat,
    Recurrence,
    Adjacent,
    LongestRun,
    Sum,
    DigitalRoot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Histogram {
    pub title: &'static str,
    /// One value per source draw; `None` is excluded from the eligible population.
    pub observations: Vec<Option<u32>>,
    pub bins: BTreeMap<u32, usize>,
    pub eligible: usize,
    pub excluded: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Cancelled,
    InvalidDraw,
}

fn common(a: &[u8; 7], b: &[u8; 7]) -> u32 {
    let (mut i, mut j, mut count) = (0, 0, 0);
    while i < 7 && j < 7 {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                count += 1;
                i += 1;
                j += 1;
            }
        }
    }
    count
}

/// Calculate one historical strategy; `progress` returns false to cancel.
/// Reject draw values that violate the sorted snapshot invariants.
pub fn calculate<F>(draws: &[Draw], strategy: Strategy, mut progress: F) -> Result<Histogram, Error>
where
    F: FnMut(usize, usize) -> bool,
{
    if !draws.iter().all(Draw::is_valid) {
        return Err(Error::InvalidDraw);
    }
    let n = draws.len();
    let total = if strategy == Strategy::Recurrence {
        n.saturating_mul(n.saturating_sub(1)) / 2
    } else {
        n
    };
    let mut completed = 0;
    if !progress(0, total) {
        return Err(Error::Cancelled);
    }
    let mut observations = Vec::with_capacity(n);
    for (idx, draw) in draws.iter().enumerate() {
        let value = match strategy {
            Strategy::Parity => Some(draw.numbers.iter().filter(|v| *v % 2 == 0).count() as u32),
            Strategy::Repeat => idx
                .checked_sub(1)
                .map(|prev| common(&draw.numbers, &draws[prev].numbers)),
            Strategy::Recurrence => {
                if idx == 0 {
                    None
                } else {
                    let mut max_overlap = 0;
                    for earlier in &draws[..idx] {
                        max_overlap = max_overlap.max(common(&draw.numbers, &earlier.numbers));
                        completed += 1;
                        if !progress(completed, total) {
                            return Err(Error::Cancelled);
                        }
                    }
                    Some(max_overlap)
                }
            }
            Strategy::Adjacent => {
                Some(draw.numbers.windows(2).filter(|v| v[1] - v[0] == 1).count() as u32)
            }
            Strategy::LongestRun => {
                let (mut current, mut longest) = (0, 0);
                for pair in draw.numbers.windows(2) {
                    if pair[1] - pair[0] == 1 {
                        current += 1;
                        longest = longest.max(current);
                    } else {
                        current = 0;
                    }
                }
                Some(longest)
            }
            Strategy::Sum => Some(draw.numbers.iter().map(|n| u32::from(*n)).sum()),
            Strategy::DigitalRoot => {
                let sum: u32 = draw.numbers.iter().map(|n| u32::from(*n)).sum();
                Some(1 + (sum - 1) % 9)
            }
        };
        observations.push(value);
        if strategy != Strategy::Recurrence {
            completed += 1;
            if !progress(completed, total) {
                return Err(Error::Cancelled);
            }
        }
    }
    let (title, known_range) = match strategy {
        Strategy::Parity => ("Dezenas pares e ímpares", Some(0..=7)),
        Strategy::Repeat => ("Repetição do concurso anterior", Some(0..=7)),
        Strategy::Recurrence => ("Maior interseção com concursos anteriores", Some(0..=7)),
        Strategy::Adjacent => ("Pares de dezenas consecutivas", Some(0..=6)),
        Strategy::LongestRun => ("Maior sequência de ligações", Some(0..=6)),
        Strategy::Sum => ("Soma das dezenas", None),
        Strategy::DigitalRoot => ("Raiz digital da soma", Some(1..=9)),
    };
    let mut bins = BTreeMap::new();
    if let Some(range) = known_range {
        for value in range {
            bins.insert(value, 0);
        }
    }
    let mut eligible = 0;
    for value in observations.iter().flatten() {
        *bins.entry(*value).or_default() += 1;
        eligible += 1;
    }
    Ok(Histogram {
        title,
        observations,
        bins,
        eligible,
        excluded: n - eligible,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Prize;
    use chrono::NaiveDate;

    fn draw(contest: u32, numbers: [u8; 7]) -> Draw {
        Draw {
            contest,
            date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            numbers,
            month: 1,
            prizes: std::array::from_fn(|_| Prize {
                winners: None,
                cents: None,
            }),
        }
    }
    fn run(draws: &[Draw], s: Strategy) -> Histogram {
        calculate(draws, s, |_, _| true).unwrap()
    }

    #[test]
    fn independent_reference_vector_and_boundaries() {
        let draws = [
            draw(1, [1, 2, 3, 10, 20, 30, 31]),
            draw(2, [1, 4, 5, 10, 12, 20, 31]),
        ];
        let expected = [
            (Strategy::Parity, vec![Some(4), Some(4)]),
            (Strategy::Repeat, vec![None, Some(4)]),
            (Strategy::Recurrence, vec![None, Some(4)]),
            (Strategy::Adjacent, vec![Some(3), Some(1)]),
            (Strategy::LongestRun, vec![Some(2), Some(1)]),
            (Strategy::Sum, vec![Some(97), Some(83)]),
            (Strategy::DigitalRoot, vec![Some(7), Some(2)]),
        ];
        for (strategy, values) in expected {
            let result = run(&draws, strategy);
            assert_eq!(result.observations, values);
            assert_eq!(result.bins.values().sum::<usize>(), result.eligible);
            assert_eq!(result.eligible + result.excluded, draws.len());
        }
        let parity = run(&draws, Strategy::Parity);
        assert_eq!(parity.bins.len(), 8);
        assert_eq!(parity.bins[&4], 2);
        assert_eq!(run(&draws[..1], Strategy::Repeat).eligible, 0);
        assert_eq!(run(&[], Strategy::Recurrence).eligible, 0);
        assert_eq!(
            run(
                &[
                    draw(1, [1, 3, 5, 7, 9, 11, 13]),
                    draw(2, [2, 4, 6, 8, 10, 12, 14])
                ],
                Strategy::Parity
            )
            .observations,
            vec![Some(0), Some(7)]
        );
    }

    #[test]
    fn recurrence_uses_only_prior_draws_and_progress_can_cancel() {
        let a = draw(1, [1, 2, 3, 4, 5, 6, 7]);
        let b = draw(2, [8, 9, 10, 11, 12, 13, 14]);
        let c = draw(3, [8, 9, 10, 11, 12, 13, 14]);
        let result = run(&[a, b, c], Strategy::Recurrence);
        assert_eq!(result.observations, vec![None, Some(0), Some(7)]);
        let mut progress = Vec::new();
        let result = calculate(
            &[
                draw(1, [1, 2, 3, 4, 5, 6, 7]),
                draw(2, [2, 3, 4, 5, 6, 7, 8]),
            ],
            Strategy::Recurrence,
            |done, total| {
                progress.push((done, total));
                done == 0
            },
        );
        assert_eq!(result, Err(Error::Cancelled));
        assert_eq!(progress, vec![(0, 1), (1, 1)]);
    }
}
