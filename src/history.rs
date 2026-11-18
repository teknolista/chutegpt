use crate::data::Draw;

const TOTAL: u64 = 2_629_575;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Cancelled,
    InvalidDraw,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cycle {
    pub first: u32,
    pub last: u32,
    pub duration: usize,
    pub distinct: u32,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleSeries {
    pub threshold: u32,
    pub completed: Vec<Cycle>,
    pub tail: Option<Cycle>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cycles {
    pub partial: CycleSeries,
    pub full: CycleSeries,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ordinals {
    pub ranks: Vec<u64>,
    pub bands: [usize; 10],
    pub changes: Vec<Option<u64>>,
    pub change_bands: [usize; 10],
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Occurrence {
    pub category: u8,
    pub count: usize,
    pub delay: Option<usize>,
}

fn choose(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut value = 1;
    for i in 1..=k {
        value = value * (n - k + i) / i;
    }
    value
}

/// Exact 1-based lexicographic rank for seven distinct sorted numbers in 1..31.
pub fn rank(numbers: &[u8; 7]) -> Option<u64> {
    if numbers[0] < 1 || numbers[6] > 31 || numbers.windows(2).any(|w| w[0] >= w[1]) {
        return None;
    }
    let mut result = 1;
    let mut previous = 0;
    for (i, &n) in numbers.iter().enumerate() {
        for smaller in previous + 1..n {
            result += choose(u64::from(31 - smaller), (6 - i) as u64);
        }
        previous = n;
    }
    Some(result)
}

struct Tracker {
    threshold: u32,
    seen: u32,
    first: Option<u32>,
    duration: usize,
    completed: Vec<Cycle>,
}
impl Tracker {
    fn new(threshold: u32) -> Self {
        Self {
            threshold,
            seen: 0,
            first: None,
            duration: 0,
            completed: Vec::new(),
        }
    }
    fn add(&mut self, draw: &Draw) {
        self.first.get_or_insert(draw.contest);
        self.duration += 1;
        for n in draw.numbers {
            debug_assert!((1..=31).contains(&n), "draw number outside 1..=31");
            self.seen |= 1_u32 << (n - 1);
        }
        if self.seen.count_ones() >= self.threshold {
            self.completed.push(Cycle {
                first: self.first.unwrap_or(draw.contest),
                last: draw.contest,
                duration: self.duration,
                distinct: self.seen.count_ones(),
            });
            self.seen = 0;
            self.first = None;
            self.duration = 0;
        }
    }
    fn finish(self, last: Option<u32>) -> CycleSeries {
        let tail = self.first.zip(last).map(|(first, last)| Cycle {
            first,
            last,
            duration: self.duration,
            distinct: self.seen.count_ones(),
        });
        CycleSeries {
            threshold: self.threshold,
            completed: self.completed,
            tail,
        }
    }
}

pub fn cycles<F>(draws: &[Draw], mut progress: F) -> Result<Cycles, Error>
where
    F: FnMut(usize, usize) -> bool,
{
    if !progress(0, draws.len()) {
        return Err(Error::Cancelled);
    }
    let (mut p, mut f) = (Tracker::new(27), Tracker::new(31));
    for (i, draw) in draws.iter().enumerate() {
        if !draw.is_valid() {
            return Err(Error::InvalidDraw);
        }
        p.add(draw);
        f.add(draw);
        if !progress(i + 1, draws.len()) {
            return Err(Error::Cancelled);
        }
    }
    let last = draws.last().map(|d| d.contest);
    Ok(Cycles {
        partial: p.finish(last),
        full: f.finish(last),
    })
}

pub fn ordinals<F>(draws: &[Draw], mut progress: F) -> Result<Ordinals, Error>
where
    F: FnMut(usize, usize) -> bool,
{
    if !progress(0, draws.len()) {
        return Err(Error::Cancelled);
    }
    let mut out = Ordinals {
        ranks: Vec::new(),
        bands: [0; 10],
        changes: Vec::new(),
        change_bands: [0; 10],
    };
    for (i, draw) in draws.iter().enumerate() {
        if !draw.is_valid() {
            return Err(Error::InvalidDraw);
        }
        let Some(value) = rank(&draw.numbers) else {
            return Err(Error::InvalidDraw);
        };
        let band = (10 * (value - 1) / TOTAL) as usize;
        out.bands[band] += 1;
        let change = out.ranks.last().map(|&old| value.abs_diff(old));
        if let Some(difference) = change {
            out.change_bands[(10 * difference / TOTAL).min(9) as usize] += 1;
        }
        out.ranks.push(value);
        out.changes.push(change);
        if !progress(i + 1, draws.len()) {
            return Err(Error::Cancelled);
        }
    }
    Ok(out)
}

pub fn occurrences<F>(
    draws: &[Draw],
    months: bool,
    mut progress: F,
) -> Result<Vec<Occurrence>, Error>
where
    F: FnMut(usize, usize) -> bool,
{
    if !progress(0, draws.len()) {
        return Err(Error::Cancelled);
    }
    let count = if months { 12 } else { 31 };
    let mut totals = vec![0_usize; count];
    let mut last = vec![None; count];
    for (i, draw) in draws.iter().enumerate() {
        if !draw.is_valid() {
            return Err(Error::InvalidDraw);
        }
        if months {
            let idx = usize::from(draw.month - 1);
            totals[idx] += 1;
            last[idx] = Some(i);
        } else {
            for &number in &draw.numbers {
                let idx = usize::from(number - 1);
                totals[idx] += 1;
                last[idx] = Some(i);
            }
        }
        if !progress(i + 1, draws.len()) {
            return Err(Error::Cancelled);
        }
    }
    Ok((0..count)
        .map(|i| Occurrence {
            category: (i + 1) as u8,
            count: totals[i],
            delay: last[i].map(|n| draws.len() - 1 - n),
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Prize;
    use chrono::NaiveDate;
    fn draw(id: u32, numbers: [u8; 7], month: u8) -> Draw {
        Draw {
            contest: id,
            date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            numbers,
            month,
            prizes: std::array::from_fn(|_| Prize {
                winners: None,
                cents: None,
            }),
        }
    }
    #[test]
    fn exact_rank_endpoints_duplicates_and_changes() {
        assert_eq!(rank(&[1, 2, 3, 4, 5, 6, 7]), Some(1));
        assert_eq!(rank(&[1, 2, 3, 4, 5, 6, 8]), Some(2));
        assert_eq!(rank(&[1, 4, 8, 17, 21, 22, 30]), Some(262_958));
        assert_eq!(rank(&[1, 4, 8, 17, 21, 22, 31]), Some(262_959));
        let boundary = [
            draw(1, [1, 4, 8, 17, 21, 22, 30], 1),
            draw(2, [1, 4, 8, 17, 21, 22, 31], 1),
        ];
        let boundary = ordinals(&boundary, |_, _| true).unwrap();
        assert_eq!((boundary.bands[0], boundary.bands[1]), (1, 1));
        assert_eq!(rank(&[25, 26, 27, 28, 29, 30, 31]), Some(TOTAL));
        assert_eq!(rank(&[1, 1, 3, 4, 5, 6, 7]), None);
        let data = [
            draw(1, [1, 2, 3, 4, 5, 6, 7], 1),
            draw(2, [25, 26, 27, 28, 29, 30, 31], 2),
            draw(3, [25, 26, 27, 28, 29, 30, 31], 1),
        ];
        let a = ordinals(&data, |_, _| true).unwrap();
        assert_eq!(a.ranks, vec![1, TOTAL, TOTAL]);
        assert_eq!(a.changes, vec![None, Some(TOTAL - 1), Some(0)]);
        assert_eq!(a.bands[0], 1);
        assert_eq!(a.bands[9], 2);
        assert_eq!(a.change_bands[0], 1);
        assert_eq!(a.change_bands[9], 1);
        let m = occurrences(&data, true, |_, _| true).unwrap();
        assert_eq!((m[0].count, m[0].delay), (2, Some(0)));
        assert_eq!((m[1].count, m[1].delay), (1, Some(1)));
        assert_eq!((m[2].count, m[2].delay), (0, None));
        let n = occurrences(&data, false, |_, _| true).unwrap();
        assert_eq!((n[7].count, n[7].delay), (0, None));
    }
    #[test]
    fn separate_cycles_and_incomplete_tail() {
        let data = [
            draw(1, [1, 2, 3, 4, 5, 6, 7], 1),
            draw(2, [8, 9, 10, 11, 12, 13, 14], 1),
            draw(3, [15, 16, 17, 18, 19, 20, 21], 1),
            draw(4, [22, 23, 24, 25, 26, 27, 28], 1),
            draw(5, [25, 26, 27, 28, 29, 30, 31], 1),
        ];
        let c = cycles(&data, |_, _| true).unwrap();
        assert_eq!(
            (
                c.partial.completed[0].duration,
                c.partial.completed[0].distinct
            ),
            (4, 28)
        );
        assert_eq!(c.partial.tail.unwrap().duration, 1);
        assert_eq!(
            (c.full.completed[0].duration, c.full.completed[0].distinct),
            (5, 31)
        );
        assert!(c.full.tail.is_none());
        assert_eq!(cycles(&data, |done, _| done < 2), Err(Error::Cancelled));
        assert!(cycles(&[], |_, _| true).unwrap().full.tail.is_none());
    }
    #[test]
    fn invalid_draws_are_rejected_instead_of_corrupting_cycles_or_counts() {
        let bad_number = draw(1, [0, 2, 3, 4, 5, 6, 7], 1);
        let unsorted = draw(2, [2, 1, 3, 4, 5, 6, 7], 1);
        let bad_month = draw(3, [1, 2, 3, 4, 5, 6, 7], 0);
        assert_eq!(
            cycles(std::slice::from_ref(&bad_number), |_, _| true),
            Err(Error::InvalidDraw)
        );
        assert_eq!(
            cycles(std::slice::from_ref(&unsorted), |_, _| true),
            Err(Error::InvalidDraw)
        );
        assert_eq!(
            occurrences(&[bad_month], false, |_, _| true),
            Err(Error::InvalidDraw)
        );
        assert_eq!(
            occurrences(&[bad_number], true, |_, _| true),
            Err(Error::InvalidDraw)
        );
    }
}
