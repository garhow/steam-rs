use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::errors::ErrorHandle;
use crate::{macros::{gen_args, do_http}, errors::EconServiceError, Steam};

const END_POINT: &str = "https://api.steampowered.com/IEconService/GetTradeOffers/v1/?";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOffer {
    pub next_cursor: u32,
    pub trade_offers_received: Option<Vec<OfferReceived>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfferReceived {
    pub tradeofferid: String,
    pub accountid_other: u32,
    pub message: String,
    pub expiration_time: u32,
    pub trade_offer_state: u32,
    pub items_to_give: Vec<Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub appid: u32,
    pub contextid: String,
    pub assetid: String,
    pub classid: String,
    pub instanceid: String,
    pub amount: String,
    pub missing: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wrapper { response: TradeOffer }

impl Steam {
    pub async fn get_trade_offers(&self,
        get_sent_offers: bool,
        get_received_offers: bool,
        get_descriptions: bool,
        language: &str,
        active_only: bool,
        historical_only: bool,
        time_historical_cutoff: u32
    ) -> Result<TradeOffer, EconServiceError> {
        let key = &self.api_key.clone();
        let args = gen_args!(key, get_sent_offers, get_received_offers, get_descriptions, language, active_only, historical_only, time_historical_cutoff);
        let url = format!("{END_POINT}{args}");
        let data: Value = do_http!(url, Value, ErrorHandle, EconServiceError::GetTradeOffers);
        let trade_offer: Wrapper = ErrorHandle!(serde_json::from_value(data.to_owned()), EconServiceError::GetTradeOffers);
        Ok(trade_offer.response)
    }
}