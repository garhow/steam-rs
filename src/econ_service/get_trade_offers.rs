
use crate::{macros::{gen_args, do_http}, errors::EconServiceError, Steam};

const END_POINT: &str = "https://api.steampowered.com/IEconService/GetTradeOffers/v1/";

impl Steam {
    /// WARNING - UNTESTED - NOT FULLY IMPLEMENTED - DO NOT USE
    /// I cannot get this to work with my steam API key as I have never traded on steam before
    pub async fn get_trade_offers(&self,
        get_sent_offers: bool,
        get_received_offers: bool,
        get_descriptions: bool,
        language: &str,
        active_only: bool,
        historical_only: bool,
        time_historical_cutoff: u32 ) -> Result<(), EconServiceError> {

        let key = &self.api_key.clone();
        let args = gen_args!(key, get_sent_offers, get_received_offers, get_descriptions, language, active_only, historical_only, time_historical_cutoff);
        let url = format!("{END_POINT}{args}");

        println!("{url}");

        let data = do_http!(url);

        println!("{:?}",data);
    
    
    
        Ok(())
    }
}