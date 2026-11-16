use std::collections::BTreeMap;
use std::f64::consts::TAU;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Cancelled,
    InvalidObservation,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Statistics {
    pub count: usize,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub range: Option<f64>,
    pub modes: Vec<f64>,
    pub mean: Option<f64>,
    pub geometric_mean: Option<f64>,
    pub harmonic_mean: Option<f64>,
    pub circular_mean_radians: Option<f64>,
    pub median: Option<f64>,
    pub mean_roots: Option<f64>,
    pub rms: Option<f64>,
    pub mad: Option<f64>,
    pub population_variance: Option<f64>,
    pub population_sd: Option<f64>,
    pub sample_variance: Option<f64>,
    pub sample_sd: Option<f64>,
    pub cv_percent: Option<f64>,
    pub skewness: Option<f64>,
    pub excess_kurtosis: Option<f64>,
}
impl Statistics {
    fn empty() -> Self {
        Self {
            count: 0,
            minimum: None,
            maximum: None,
            range: None,
            modes: Vec::new(),
            mean: None,
            geometric_mean: None,
            harmonic_mean: None,
            circular_mean_radians: None,
            median: None,
            mean_roots: None,
            rms: None,
            mad: None,
            population_variance: None,
            population_sd: None,
            sample_variance: None,
            sample_sd: None,
            cv_percent: None,
            skewness: None,
            excess_kurtosis: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Series {
    pub events: usize,
    pub unknown: usize,
    pub periodicities: Vec<u64>,
    pub missing_periodicities: usize,
    pub closed_waits: Vec<u64>,
    pub missing_waits: usize,
    pub open_delay: Option<u64>,
    pub periodicity_stats: Statistics,
    pub wait_stats: Statistics,
}
fn finite(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

/// Descriptive statistics on a nonnegative sample; undefined measures are `None`.
pub fn statistics(values: &[f64]) -> Result<Statistics, Error> {
    if values.iter().any(|v| !v.is_finite() || *v < 0.0) {
        return Err(Error::InvalidObservation);
    }
    if values.is_empty() {
        return Ok(Statistics::empty());
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let n = values.len();
    let nf = n as f64;
    let mean = values.iter().sum::<f64>() / nf;
    if !mean.is_finite() {
        return Err(Error::InvalidObservation);
    }
    let mut modes = Vec::new();
    let (mut max_count, mut i) = (0, 0);
    while i < n {
        let mut j = i + 1;
        while j < n && sorted[j] == sorted[i] {
            j += 1;
        }
        let count = j - i;
        if count > max_count {
            modes.clear();
            max_count = count;
        }
        if count == max_count {
            modes.push(sorted[i]);
        }
        i = j;
    }
    let m2 = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / nf;
    let m3 = values.iter().map(|v| (v - mean).powi(3)).sum::<f64>() / nf;
    let m4 = values.iter().map(|v| (v - mean).powi(4)).sum::<f64>() / nf;
    let sin = values.iter().map(|v| v.sin()).sum::<f64>();
    let cos = values.iter().map(|v| v.cos()).sum::<f64>();
    let sample = if n > 1 {
        finite(m2 * nf / (nf - 1.0))
    } else {
        None
    };
    let root_mean = values.iter().map(|v| v.sqrt()).sum::<f64>() / nf;
    let rms = (values.iter().map(|v| v * v).sum::<f64>() / nf).sqrt();
    let mad = values.iter().map(|v| (v - mean).abs()).sum::<f64>() / nf;
    let geometric = if values.contains(&0.0) {
        Some(0.0)
    } else {
        finite((values.iter().map(|v| v.ln()).sum::<f64>() / nf).exp())
    };
    let harmonic = if values.contains(&0.0) {
        None
    } else {
        finite(nf / values.iter().map(|v| 1.0 / v).sum::<f64>())
    };
    let circular = if sin.hypot(cos) <= 1e-12 * nf {
        None
    } else {
        finite(sin.atan2(cos).rem_euclid(TAU))
    };
    Ok(Statistics {
        count: n,
        minimum: Some(sorted[0]),
        maximum: Some(sorted[n - 1]),
        range: finite(sorted[n - 1] - sorted[0]),
        modes,
        mean: Some(mean),
        geometric_mean: geometric,
        harmonic_mean: harmonic,
        circular_mean_radians: circular,
        median: Some(sorted[(n - 1) / 2]),
        mean_roots: finite(root_mean),
        rms: finite(rms),
        mad: finite(mad),
        population_variance: finite(m2),
        population_sd: finite(m2.sqrt()),
        sample_variance: sample,
        sample_sd: sample.and_then(|v| finite(v.sqrt())),
        cv_percent: if mean == 0.0 {
            None
        } else {
            finite(100.0 * m2.sqrt() / mean)
        },
        skewness: if m2 == 0.0 {
            None
        } else {
            finite(m3 / m2.powf(1.5))
        },
        excess_kurtosis: if m2 == 0.0 {
            None
        } else {
            finite(m4 / (m2 * m2) - 3.0)
        },
    })
}

/// Analyze occurrence states while retaining unknown intervals and the censored tail.
pub fn analyze<F>(states: &[Option<bool>], mut progress: F) -> Result<Series, Error>
where
    F: FnMut(usize, usize) -> bool,
{
    if !progress(0, states.len()) {
        return Err(Error::Cancelled);
    }
    let mut events = 0_usize;
    let mut unknown = 0;
    let mut unknown_before = false;
    let mut unknown_since = false;
    let mut last = None;
    let mut periods = Vec::new();
    let mut missed_periods = 0;
    let mut waits = Vec::new();
    let mut missed_waits = 0;
    for (i, state) in states.iter().enumerate() {
        match state {
            None => {
                unknown += 1;
                unknown_before = true;
                unknown_since = true;
            }
            Some(true) => {
                events += 1;
                if unknown_before {
                    missed_periods += 1;
                } else {
                    periods.push((i as u64 + 1) / (events as u64));
                }
                if let Some(p) = last {
                    if unknown_since {
                        missed_waits += 1;
                    } else {
                        waits.push((i - p - 1) as u64);
                    }
                }
                last = Some(i);
                unknown_since = false;
            }
            Some(false) => {}
        }
        if !progress(i + 1, states.len()) {
            return Err(Error::Cancelled);
        }
    }
    let periodicity_stats = statistics(&periods.iter().map(|&x| x as f64).collect::<Vec<_>>())?;
    let wait_stats = statistics(&waits.iter().map(|&x| x as f64).collect::<Vec<_>>())?;
    Ok(Series {
        events,
        unknown,
        periodicities: periods,
        missing_periodicities: missed_periods,
        closed_waits: waits,
        missing_waits: missed_waits,
        open_delay: if unknown_since {
            None
        } else {
            last.map(|p| (states.len() - 1 - p) as u64)
        },
        periodicity_stats,
        wait_stats,
    })
}

/// Turn aligned category observations into a 0/1/missing event series.
pub fn category<F>(values: &[Option<i64>], target: i64, progress: F) -> Result<Series, Error>
where
    F: FnMut(usize, usize) -> bool,
{
    let states: Vec<_> = values.iter().map(|v| v.map(|v| v == target)).collect();
    analyze(&states, progress)
}
pub fn histogram(values: &[u64]) -> BTreeMap<u64, usize> {
    let mut out = BTreeMap::new();
    for &v in values {
        *out.entry(v).or_default() += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    fn near(actual: Option<f64>, expected: f64) {
        assert!((actual.unwrap() - expected).abs() < 1e-10);
    }
    #[test]
    fn all_reference_statistics_and_edge_cases() {
        let s = statistics(&[1., 2., 3.]).unwrap();
        assert_eq!(s.modes, vec![1., 2., 3.]);
        assert_eq!(s.count, 3);
        near(s.minimum, 1.);
        near(s.maximum, 3.);
        near(s.range, 2.);
        near(s.mean, 2.);
        near(s.geometric_mean, 6_f64.powf(1. / 3.));
        near(s.harmonic_mean, 18. / 11.);
        near(s.circular_mean_radians, 2.);
        near(s.median, 2.);
        near(s.mean_roots, (1. + 2_f64.sqrt() + 3_f64.sqrt()) / 3.);
        near(s.rms, (14_f64 / 3.).sqrt());
        near(s.mad, 2. / 3.);
        near(s.population_variance, 2. / 3.);
        near(s.population_sd, (2_f64 / 3.).sqrt());
        near(s.sample_variance, 1.);
        near(s.sample_sd, 1.);
        near(s.cv_percent, 40.8248290463863);
        near(s.skewness, 0.);
        near(s.excess_kurtosis, -1.5);
        assert_eq!(statistics(&[1., 2., 3., 4.]).unwrap().median, Some(2.));
        let z = statistics(&[0., 0.]).unwrap();
        assert_eq!(z.geometric_mean, Some(0.));
        assert_eq!(z.harmonic_mean, None);
        assert_eq!(z.cv_percent, None);
        assert_eq!(z.skewness, None);
        assert_eq!(z.excess_kurtosis, None);
        assert_eq!(statistics(&[1.]).unwrap().sample_variance, None);
        assert_eq!(statistics(&[]).unwrap().mean, None);
        assert_eq!(
            statistics(&[0., std::f64::consts::PI])
                .unwrap()
                .circular_mean_radians,
            None
        );
        assert_eq!(statistics(&[-1.]), Err(Error::InvalidObservation));
        assert_eq!(statistics(&[f64::NAN]), Err(Error::InvalidObservation));
    }
    #[test]
    fn zero_waits_unknowns_and_censored_tail() {
        let s = analyze(
            &[Some(true), Some(true), Some(false), Some(true)],
            |_, _| true,
        )
        .unwrap();
        assert_eq!(s.periodicities, vec![1, 1, 1]);
        assert_eq!(s.closed_waits, vec![0, 1]);
        assert_eq!(s.open_delay, Some(0));
        let s = analyze(&[Some(true), None, Some(true)], |_, _| true).unwrap();
        assert_eq!(s.periodicities, vec![1]);
        assert_eq!(s.missing_periodicities, 1);
        assert_eq!(s.missing_waits, 1);
        assert!(s.closed_waits.is_empty());
        let s = analyze(&[Some(true), None, Some(false)], |_, _| true).unwrap();
        assert_eq!(s.open_delay, None);
        assert_eq!(
            analyze(&[Some(true)], |done, _| done == 0),
            Err(Error::Cancelled)
        );
        assert_eq!(
            category(&[Some(1), Some(1), None], 1, |_, _| true)
                .unwrap()
                .closed_waits,
            vec![0]
        );
    }
}
