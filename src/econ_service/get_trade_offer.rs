
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;


use crate::errors::ErrorHandle;
use crate::{macros::{gen_args, do_http}, errors::EconServiceError, Steam};

const END_POINT: &str = "https://api.steampowered.com/IEconService/GetTradeOffer/v1/?";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOffer{
    pub offer : Vec<Offer>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Offer{
    pub tradeofferid: String,
    pub accountid_other: u32,
    pub message: String,
    pub expiration_time: u32,
    pub trade_offer_state: u32,
    pub items_to_give: Option<Vec<ItemsGive>>,
    pub items_to_recieve: Option<Vec<ItemsRecieve>>,
    pub is_our_offer: bool,
    pub time_created: u32,
    pub time_updated: u32,
    pub from_real_time_trade: bool,
    pub escrow_end_date: u32,
    pub confirmation_method: u32,
    pub eresult: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct  ItemsGive{
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
pub struct  ItemsRecieve{
    pub appid: u32,
    pub contextid: String,
    pub assetid: String,
    pub classid: String,
    pub instanceid: String,
    pub amount: String,
    pub missing: bool,
    pub est_usd: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response { response: TradeOffer }

impl Steam {
    /// WARNING - UNTESTED - NOT FULLY IMPLEMENTED - DO NOT USE
    /// I cannot get this to work with my steam API key as I have never traded on steam before
    /// Please don't actually try to use this - no support will be provided for this
    pub async fn get_trade_offer(&self,
        
        tradeofferid: u64,
        language: &str,

        ) -> Result<TradeOffer, EconServiceError> {

        let key = &self.api_key.clone();
        let args = gen_args!(key, tradeofferid, language);
        let url = format!("{END_POINT}{args}");

        println!("{url}");

        let data = do_http!(url, Value, ErrorHandle, EconServiceError::GetTradeOffer);

        //println!("{:?}",data);
        let trade_offer: Response = ErrorHandle!(serde_json::from_value(data.to_owned()), EconServiceError::GetTradeOffer);

        Ok(trade_offer.response)
    }
}