use serde_json::Value;

use crate::{macros::{gen_args, do_http}, errors::EconServiceError, Steam};

const END_POINT: &str = "https://api.steampowered.com/IEconService/GetTradeHistory/v1/";

impl Steam {
    /// WARNING - UNTESTED - NOT FULLY IMPLEMENTED - DO NOT USE
    /// I cannot get this to work with my steam API key as I have never traded on steam before
    /// Please don't actually try to use this - no support will be provided for this
    pub async fn get_trade_history(&self,
        max_trades: u32,
        start_after_time: u32,
        start_after_trade_id: u64,
        navigating_back: bool,
        get_descriptions: bool,
        language: &str,
        include_failed: bool,
        include_total: bool) -> Result<(), EconServiceError> {

        let key = &self.api_key.clone();
        let args = gen_args!(key, max_trades, start_after_time, start_after_trade_id, navigating_back, get_descriptions, language, include_failed, include_total);
        let url = format!("{END_POINT}{args}");

        println!("{url}");

        let data = do_http!(url);

        println!("{:?}",data);
    
    
    
        Ok(())
    }
}