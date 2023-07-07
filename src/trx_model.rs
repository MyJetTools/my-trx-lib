use crate::{TrxAddress, UsdtParams};
use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrxBlockModel {
    #[serde(rename = "blockID")]
    pub block_id: String,
    pub block_header: TrxBlockHeader,
    pub transactions: Vec<TrxTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrxBlockHeader {
    pub raw_data: RawData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawData {
    pub number: u64,
}

/// Transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrxTransaction {
    #[serde(rename = "txID")]
    pub tx_id: String,
    pub raw_data: TrxTransactionRawData,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrxTransactionRawData {
    pub contract: Vec<TrxTransactionContract>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrxTransactionContract {
    pub parameter: TrxTransactionContractParameter,
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
}

impl TrxTransactionContract {
    pub fn read_transaction_data(&self) -> Option<ContractData> {
        let tx_type = self.tx_type.as_ref()?;

        match tx_type.as_str() {
            "TransferContract" => {
                let from = self.parameter.value.owner_address.as_ref()?;
                let from = TrxAddress::from_hex(from);

                let to = self.parameter.value.to_address.as_ref()?;
                let to = TrxAddress::from_hex(to);

                let result = ContractData::Transfer {
                    from: from.as_base58().to_string(),
                    to: to.as_base58().to_string(),
                    amount: self.parameter.value.amount.clone()?,
                };

                Some(result)
            }

            "TriggerSmartContract" => {
                let usdt_params = UsdtParams::from_str(self.parameter.value.data.as_ref()?)?;

                let from = self.parameter.value.owner_address.as_ref()?;
                let from = TrxAddress::from_hex(from);

                let result = ContractData::TriggerUsdtSmartContract {
                    from: from.as_base58().to_string(),
                    to: usdt_params.get_to_address().as_base58().to_string(),
                    amount: usdt_params.get_amount(),
                };

                Some(result)
            }

            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrxTransactionContractParameter {
    pub value: TrxTransactionContractParameterValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrxTransactionContractParameterValue {
    pub owner_address: Option<String>,
    pub to_address: Option<String>,
    pub amount: Option<u64>,
    pub data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractData {
    Transfer {
        from: String,
        to: String,
        amount: u64,
    },
    TriggerUsdtSmartContract {
        from: String,
        to: String,
        amount: u64,
    },
}

#[cfg(test)]
mod test {

    #[test]
    fn test_random_transaction_scan() {
        let tx_data = std::fs::read_to_string("./test_data/block.json").unwrap();

        let value = serde_json::from_str::<super::TrxBlockModel>(&tx_data).unwrap();

        for tx in value.transactions {
            for contract in tx.raw_data.contract {
                let data = contract.read_transaction_data();

                if let Some(data) = data {
                    println!("Tx {} : {:?}", tx.tx_id, data);
                }
            }
        }
    }
}
