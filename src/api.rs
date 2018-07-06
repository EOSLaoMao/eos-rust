extern crate reqwest;

use api::reqwest::Error;
use api::reqwest::Response;
use std::collections::HashMap;

pub struct API {
    base_url: String,
    Debug: bool,
}

impl API {
    pub fn new(base_url: String, Debug: bool) -> API {
        API { base_url, Debug }
    }

    // chain api
    pub fn get_chain_info(self) -> Result<Response, Error> {
        self.http_get("/v1/chain/get_info".to_string())
    }

    pub fn get_currency_balance(
        &self,
        account_name: &str,
        code: &str,
        symbol: &str,
    ) -> Result<Response, Error> {

        let mut body = HashMap::new();
        body.insert("account", account_name);
        body.insert("code", code);
        body.insert("symbol", symbol);

        self.http_post("/v1/chain/get_currency_balance".to_string(), body)
    }

    fn http_get(&self, _end_point: String) -> Result<Response, Error> {
        let url = self.base_url.clone() + &_end_point;
        let res = reqwest::get(&url)?;
        Ok(res)
    }

    fn http_post(
        &self,
        _end_point: String,
        body: HashMap<&str, &str>,
    ) -> Result<Response, Error> {
        let url = self.base_url.clone() + &_end_point;
        let client = reqwest::Client::new();
        let res = client.post(&url).json(&body).send()?;
        Ok(res)
    }
}
