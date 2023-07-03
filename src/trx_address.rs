use rust_extensions::StrOrString;

pub enum TrxAddress<'s> {
    AsHex(&'s str),
    AsBase58(&'s str),
}

impl<'s> TrxAddress<'s> {
    pub fn from_hex(src: &'s str) -> Self {
        if src.len() != 42 {
            panic!("Invalid address length");
        }
        Self::AsHex(src)
    }

    pub fn from_base58(src: &'s str) -> Self {
        Self::AsBase58(src)
    }

    pub fn as_hex(&self) -> StrOrString<'s> {
        match self {
            TrxAddress::AsHex(value) => (*value).into(),
            TrxAddress::AsBase58(value) => {
                hex::encode(decode_base58_check_address(value).unwrap()).into()
            }
        }
    }

    pub fn as_base58(&'s self) -> StrOrString<'s> {
        match self {
            TrxAddress::AsHex(value) => {
                let mut buffer = [0u8; 21];
                hex::decode_to_slice(value, &mut buffer).unwrap();
                let encoded = get_base58_check_address(&buffer);
                StrOrString::create_as_string(encoded)
            }
            TrxAddress::AsBase58(value) => (*value).into(),
        }
    }
}

pub fn get_base58_check_address(address_bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};

    let mut sha256 = Sha256::new();

    sha256.update(address_bytes);
    let hash0 = sha256.finalize().to_vec();

    let mut sha256 = Sha256::new();
    sha256.update(&hash0);
    let hash1 = sha256.finalize().to_vec();

    let check_sum = &hash1[0..4];
    let mut check_sum_vec = address_bytes.to_vec();
    check_sum_vec.extend_from_slice(check_sum);

    bs58::encode(&check_sum_vec).into_string()
    //encode58(&check_sum_vec)
}

pub fn decode_base58_check_address(address: &str) -> Result<Vec<u8>, &'static str> {
    use sha2::{Digest, Sha256};
    // Decode the address from Base58
    let decoded = bs58::decode(address).into_vec().unwrap();

    if decoded.len() < 4 {
        return Err("Invalid address length");
    }

    // Split the address and checksum
    let (address_bytes, checksum) = decoded.split_at(decoded.len() - 4);
    let address_bytes = address_bytes.to_vec();

    // Compute checksum
    let mut sha256 = Sha256::new();

    sha256.update(&address_bytes);
    let hash0 = sha256.finalize().to_vec();

    let mut sha256 = Sha256::new();
    sha256.update(hash0);
    let hash1 = sha256.finalize().to_vec();

    // Validate checksum
    if &hash1[0..4] != checksum {
        return Err("Invalid checksum");
    }

    Ok(address_bytes)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_from_hex_to_base58() {
        let src = "41b9e2ead763496f431205354cd14ed18e74eb3774";

        let address = super::TrxAddress::from_hex(src);

        assert_eq!(
            address.as_base58().as_str(),
            "TSv5r3Mg2hdPvot6QgtSDXZuxVmKtLzSuS"
        );
    }

    #[test]
    fn test_from_base58_to_hex() {
        let src = "TSv5r3Mg2hdPvot6QgtSDXZuxVmKtLzSuS";

        let dest = super::decode_base58_check_address(src).unwrap();

        assert_eq!(
            hex::encode(dest),
            "41b9e2ead763496f431205354cd14ed18e74eb3774"
        );
    }
}
