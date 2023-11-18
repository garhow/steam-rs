use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::errors::ErrorHandle;
use crate::{
    errors::EconServiceError,
    macros::{do_http, gen_args},
    Steam,
};

const END_POINT: &str = "https://api.steampowered.com/IEconService/GetTradeOffersSummary/v1/?";

//NOT FINISHED BRUH, DONT RUN OR COMPUTER BUSTS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOffersSummary {
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
struct Wrapper {
    response: TradeOffersSummary,
}

impl Steam {
    pub async fn get_trade_offers_summary(
        &self,
        time_last_visit: u32,
    ) -> Result<TradeOffersSummary, EconServiceError> {
        let key = &self.api_key.clone();
        let args = gen_args!(key, time_last_visit);
        let url = format!("{END_POINT}{args}");
        let data = do_http!(
            url,
            Value,
            ErrorHandle,
            EconServiceError::GetTradeOffersSummary
        );
        let trade_offers_summary: Wrapper = ErrorHandle!(
            serde_json::from_value(data.to_owned()),
            EconServiceError::GetTradeOffersSummary
        );
        Ok(trade_offers_summary.response)
    }
}
