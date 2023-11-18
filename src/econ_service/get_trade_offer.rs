use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::errors::ErrorHandle;
use crate::{
    errors::EconServiceError,
    macros::{do_http, gen_args},
    Steam,
};

const END_POINT: &str = "https://api.steampowered.com/IEconService/GetTradeOffer/v1/?";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeOffer {
    #[serde(rename = "tradeofferid")]
    pub id: String,
    pub accountid_other: u32,
    pub message: String,
    pub expiration_time: u32,
    pub trade_offer_state: u32,
    pub items_to_give: Option<Vec<Item>>,
    pub items_to_recieve: Option<Vec<Item>>,
    pub is_our_offer: bool,
    pub time_created: u32,
    pub time_updated: u32,
    pub from_real_time_trade: bool,
    pub escrow_end_date: u32,
    pub confirmation_method: u32,
    pub eresult: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item {
    pub appid: u32,
    pub contextid: String,
    pub assetid: String,
    pub classid: String,
    pub instanceid: String,
    pub amount: String,
    pub missing: bool,
    pub est_usd: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Offer {
    pub offer: TradeOffer,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wrapper {
    response: Offer,
}

impl Steam {
    pub async fn get_trade_offer(
        &self,
        tradeofferid: u64,
        language: &str,
    ) -> Result<TradeOffer, EconServiceError> {
        let key = &self.api_key.clone();
        let args = gen_args!(key, tradeofferid, language);
        let url = format!("{END_POINT}{args}");
        let data = do_http!(url, Value, ErrorHandle, EconServiceError::GetTradeOffer);
        let trade_offer: Wrapper = ErrorHandle!(
            serde_json::from_value(data.to_owned()),
            EconServiceError::GetTradeOffer
        );
        Ok(trade_offer.response.offer)
    }
}
