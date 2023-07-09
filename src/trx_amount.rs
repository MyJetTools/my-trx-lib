pub const TRX_AMOUNT_MULTIPLIER: u64 = 1_000_000;
#[derive(Clone, Copy)]
pub struct TrxAmount(u64);

impl TrxAmount {
    pub fn from_u64(value: u64) -> Self {
        Self(value)
    }

    pub fn from_f64(value: f64) -> Self {
        let value = value * TRX_AMOUNT_MULTIPLIER as f64;
        Self(value as u64)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn as_f64(&self) -> f64 {
        self.0 as f64 / TRX_AMOUNT_MULTIPLIER as f64
    }
}

impl std::fmt::Display for TrxAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.as_u64();
        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for TrxAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.as_u64();
        write!(f, "{}", value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u64() {
        let value = TrxAmount::from_u64(14200000);

        assert_eq!(value.as_f64(), 14.2);
        assert_eq!(value.as_u64(), 14200000);
    }

    #[test]
    fn test_from_f64() {
        let value = TrxAmount::from_f64(14.2);

        assert_eq!(value.as_f64(), 14.2);
        assert_eq!(value.as_u64(), 14200000);
    }
}
