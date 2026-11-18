//! Numeric helpers shared by the analysis modules.

/// Round a value to integer hundredths (ties to even), rejecting values that
/// cannot be represented as `i64` after scaling.
///
/// Both the composed dispersion measures and the historical interval measures
/// must use this single rule so their displayed values stay comparable.
pub fn scaled100(value: f64) -> Option<i64> {
    let scaled = (value * 100.0).round_ties_even();
    if !scaled.is_finite() || scaled < i64::MIN as f64 || scaled >= i64::MAX as f64 {
        None
    } else {
        Some(scaled as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_half_to_even_and_rejects_overflow() {
        assert_eq!(scaled100(1.125), Some(112));
        assert_eq!(scaled100(1.135), Some(114));
        assert_eq!(scaled100(-1.125), Some(-112));
        assert_eq!(scaled100(0.0), Some(0));
        assert_eq!(scaled100(f64::NAN), None);
        assert_eq!(scaled100(f64::INFINITY), None);
        assert_eq!(scaled100(f64::MAX), None);
        assert_eq!(scaled100(i64::MIN as f64 / 100.0), Some(i64::MIN));
        assert_eq!(scaled100(i64::MAX as f64 / 100.0), None);
    }
}
