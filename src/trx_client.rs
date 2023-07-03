use flurl::IntoFlUrl;
use rust_extensions::StrOrString;
use serde::{Deserialize, Serialize};

pub enum TrxClient {
    Shasta,
    Nile,
    Production,
}

impl TrxClient {
    pub fn get_url(&self) -> &str {
        match self {
            TrxClient::Shasta => "https://api.shasta.trongrid.io",
            TrxClient::Nile => "https://nile.trongrid.io",
            TrxClient::Production => "https://api.trongrid.io",
        }
    }

    pub async fn get_block(&self, data: GetBlockInfoModel) -> String {
        /* cSpell:disable */
        let result = self
            .get_url()
            .append_path_segment("walletsolidity")
            .append_path_segment("getblock")
            .post(Some(data.get_content()))
            .await
            .unwrap()
            .get_body()
            .await
            .unwrap()
            .to_vec();

        /* cSpell:enable */
        String::from_utf8(result).unwrap()
    }

    pub async fn get_transaction_info<'s>(&self, tx: impl Into<StrOrString<'s>>) -> String {
        let tx: StrOrString = tx.into();
        let model = GetTransactionIdModel {
            value: tx.to_string(),
            detail: true,
        };
        /* cSpell:disable */
        let result = self
            .get_url()
            .append_path_segment("walletsolidity")
            .append_path_segment("gettransactioninfobyid")
            .post(Some(serde_json::to_vec(&model).unwrap()))
            .await
            .unwrap()
            .get_body()
            .await
            .unwrap()
            .to_vec();
        /* cSpell:enable */
        String::from_utf8(result).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTransactionIdModel {
    value: String,
    detail: bool,
}

pub enum GetBlockInfoModel {
    Latest(GetLastBlockModel),
    ByNumber(GetBlockByNumber),
}

impl GetBlockInfoModel {
    pub fn get_content(&self) -> Vec<u8> {
        match self {
            GetBlockInfoModel::Latest(model) => serde_json::to_vec(model).unwrap(),
            GetBlockInfoModel::ByNumber(model) => serde_json::to_vec(model).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLastBlockModel {
    pub detail: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlockByNumber {
    pub id_or_num: String,
    pub detail: bool,
}
