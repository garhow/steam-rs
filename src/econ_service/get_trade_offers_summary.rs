
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;


use crate::errors::ErrorHandle;
use crate::{macros::{gen_args, do_http}, errors::EconServiceError, Steam};

const END_POINT: &str = "https://api.steampowered.com/IEconService/GetTradeOffersSummary/v1/?";

//NOT FINISHED BRUH, DONT RUN OR COMPUTER BUSTS

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOffersSummary{
    pub pending_recieved_count: Option<u32>,
    pub new_recieved_count: Option<u32>,
    pub updated_recieved_count: Option<u32>,
    pub historical_recieved_count: Option<u32>,
    pub pending_sent_count: Option<u32>,
    pub newly_recieved_count: Option<u32>,
    pub updated_sent_count: Option<u32>,
    pub historical_sent_count: Option<u32>,
    pub escrow_recieved_count: Option<u32>,
    pub escrow_sent_count: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response { response: TradeOffersSummary }

impl Steam {
    /// WARNING - UNTESTED - NOT FULLY IMPLEMENTED - DO NOT USE
    /// I cannot get this to work with my steam API key as I have never traded on steam before
    /// Please don't actually try to use this - no support will be provided for this
    pub async fn get_trade_offers_summary(&self,
        
        time_last_visit: u32,

        ) -> Result<TradeOffersSummary, EconServiceError> {

        let key = &self.api_key.clone();
        let args = gen_args!(key, time_last_visit);
        let url = format!("{END_POINT}{args}");

        println!("{url}");

        let data = do_http!(url, Value, ErrorHandle, EconServiceError::GetTradeOffersSummary);

        //println!("{:?}",data);
        let trade_offers_summary: Response = ErrorHandle!(serde_json::from_value(data.to_owned()), EconServiceError::GetTradeOffersSummary);

        Ok(trade_offers_summary.response)
    }
}