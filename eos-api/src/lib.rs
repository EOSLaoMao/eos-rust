extern crate reqwest;
#[macro_use]
extern crate serde_json;

use reqwest::Error;
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct API {
    base_url: String,
}


impl API {
    pub fn new(base_url: String) -> API {
        API { base_url }
    }

    // net api
    pub fn get_connections(&self) -> Result<Value, Error> {
        self.http_get("/net/connections")
    }

    //chain api
    pub fn get_chain_info(&self) -> Result<Value, Error> {
        self.http_get("/chain/get_info")
    }

    pub fn get_account(&self, account_name: &str) -> Result<Value, Error> {
        let body = &json!({ "account_name": account_name });
        self.http_post("/chain/get_account", body)
    }

    pub fn get_abi(&self, account_name: &str) -> Result<Value, Error> {
        let body = &json!({ "account_name": account_name });
        self.http_post("/chain/get_abi", body)
    }

    pub fn get_block(&self, block_id: u64) -> Result<Value, Error> {
        let body = &json!({ "block_num_or_id": block_id });
        self.http_post("/chain/get_block", body)
    }

    pub fn get_code(&self, account: &str) -> Result<Value, Error> {
        let body = &json!({"account_name": account, "code_as_wasm": false});
        self.http_post("/chain/get_code", body)
    }

    pub fn get_currency_balance(
        &self,
        account_name: &str,
        code: &str,
        symbol: &str,
    ) -> Result<Value, Error> {
        let body = &json!({
                "account": account_name,
                "code": code,
                "symbol": symbol
            });

        self.http_post("/chain/get_currency_balance", body)
    }

    fn http_get(&self, _end_point: &str) -> Result<Value, Error> {
        let url = self.base_url.to_owned() + "/v1" + _end_point;
        let res = reqwest::get(&url)?.json()?;
        Ok(res)
    }

    fn http_post(&self, _end_point: &str, body: &serde_json::Value) -> Result<Value, Error> {
        let url = self.base_url.to_owned() + "/v1" + _end_point;
        let res = reqwest::Client::new()
            .post(&url)
            .json(body)
            .send()?
            .json()?;
        Ok(res)
    }
}



