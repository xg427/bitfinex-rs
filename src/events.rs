use ticker::*;
use trades::{TradingPair as TradesTradingPair, FundingCurrency as TradesFundingCurrency};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum NotificationEvent {
    Info { event: String, version: u16, platform: Platform },

    #[serde(rename_all = "camelCase")]
    TradingSubsbribed { event: String, channel: String, chan_id: u16, symbol: String, pair: String },

    #[serde(rename_all = "camelCase")]
    FundingSubsbribed { event: String, channel: String, chan_id: u16, symbol: String, currency: String },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DataEvent {
    TickerTradingEvent (i32, TradingPair),
    TickerFundingEvent (i32, FundingCurrency),
    TradesTradingEvent (i32, Vec<TradesTradingPair>),
    TradesTradingUpdateEvent (i32, String, TradesTradingPair),
    TradesFundingEvent (i32, Vec<TradesFundingCurrency>),
    TradesFundingUpdateEvent (i32, String, TradesFundingCurrency),
    HeartbeatEvent (i32, String)
}

#[derive(Debug, Deserialize)]
pub struct Platform {
    pub status: u16,
}
