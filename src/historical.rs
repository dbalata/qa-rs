use std::vec::Vec;
use chrono::DateTime;

pub struct HistoricalMetaData {
    information: String,
    symbol: String,
    last_refreshed: String,
    interval: String,
    output_size: String,
    time_zone: String,
}

pub struct HistoricalPrice {
    time: DateTime<chrono::Utc>,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: i32,
}

pub struct HistoricalData {
    meta_data: HistoricalMetaData,
    time_series: Vec<HistoricalPrice>,
}