use super::TrxAddress;

pub struct UsdtParams<'s> {
    src: &'s str,
}

impl<'s> UsdtParams<'s> {
    pub fn from_str(src: &'s str) -> Option<Self> {
        if !src.starts_with("a9059cbb") {
            return None;
        }

        Self { src }.into()
    }

    pub fn get_to_address(&self) -> TrxAddress {
        TrxAddress::from_hex(&self.src[30..30 + 42])
    }

    pub fn get_amount(&self) -> u64 {
        let mut result = [0u8; 8];

        let src = &self.src[self.src.len() - 16..];

        hex::decode_to_slice(src, &mut result).unwrap();

        u64::from_be_bytes(result)
    }
}

#[cfg(test)]
mod test {
    use super::UsdtParams;

    #[test]
    fn test_real_case_1() {
        let params = UsdtParams::from_str("a9059cbb000000000000000000000041b9e2ead763496f431205354cd14ed18e74eb377400000000000000000000000000000000000000000000000000000000b6d76750").unwrap();

        assert_eq!(
            params.get_to_address().as_hex().as_str(),
            "41b9e2ead763496f431205354cd14ed18e74eb3774"
        );

        assert_eq!(
            params.get_to_address().as_base58().as_str(),
            "TSv5r3Mg2hdPvot6QgtSDXZuxVmKtLzSuS"
        );
        assert_eq!(params.get_amount(), 3067570000);
    }

    #[test]
    fn test_real_case_2() {
        let params = UsdtParams::from_str("a9059cbb000000000000000000000041b9e2ead763496f431205354cd14ed18e74eb37740000000000000000000000000000000000000000000000000000000165a0bc00").unwrap();
        assert_eq!(
            params.get_to_address().as_hex().as_str(),
            "41b9e2ead763496f431205354cd14ed18e74eb3774"
        );

        assert_eq!(
            params.get_to_address().as_base58().as_str(),
            "TSv5r3Mg2hdPvot6QgtSDXZuxVmKtLzSuS"
        );
        assert_eq!(params.get_amount(), 6000000000);
    }
}
