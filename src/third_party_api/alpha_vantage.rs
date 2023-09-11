use std::fmt::{self, Debug};
use anyhow::{Result, Context};
use reqwest::Client;

/// Specifies the time range function to be called in the Alpha Vantage API
#[derive(Debug)]
pub enum AlphaVantageRangeFunction {
    /// An intraday time series of the equity specified, covering extended trading hours where applicable.
    Intraday,
    /// A daily time series of the equity specified, covering 20+ years of historical data.
    Daily,
    /// A daily time series of the equity specified, covering up to 20 years of historical data with adjusted close values.
    DailyAdjusted,
    /// A weekly time series of the equity specified, covering 20+ years of historical data.
    Weekly,
    /// A weekly time series of the equity specified, covering up to 20 years of historical data with adjusted close values.
    WeeklyAdjusted,
    /// A monthly time series of the equity specified, covering 20+ years of historical data.
    Monthly,
    /// A monthly time series of the equity specified, covering up to 20 years of historical data with adjusted close values.
    MonthlyAdjusted,
}

impl fmt::Display for AlphaVantageRangeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

/// Specifies the parameters for a query to the Alpha Vantage API
#[derive(Debug)]
pub struct AlphaVantageRangeQuery {
    /// The function to be called in the Alpha Vantage API
    function: AlphaVantageRangeFunction,
    /// The symbol of the equity to be queried
    symbol: String,
    /// The API key to be used in the query
    api_key: String,
    /// The interval between two consecutive data points in the time series
    interval: String,
    /// The size of the output time series of the query
    output_size: String,
}

/// Sends a query to the AlphaVantage API and retrieves the response as a string.
///
/// This function constructs a URL based on the provided `AlphaVantageRangeQuery` and sends an HTTP GET request to that URL.
/// It expects the query to include the necessary function, symbol, API key, interval, and output size parameters.
///
/// # Arguments
///
/// * `query` - An `AlphaVantageRangeQuery` struct containing the query parameters.
///
/// # Returns
///
/// * If successful, it returns a `Result<String, Error>` where `String` contains the response from the API.
/// * If an error occurs during the request or response handling, it returns an `Err` variant with an error message.
///
/// # Example
///
/// ```rust
/// use alphavantage::AlphaVantageRangeQuery;
///
/// let query = AlphaVantageRangeQuery {
///     function: "TIME_SERIES_INTRADAY",
///     symbol: "AAPL",
///     api_key: "your_api_key",
///     interval: "15min",
///     output_size: "compact",
/// };
///
/// match query(query) {
///     Ok(response) => println!("Response: {}", response),
///     Err(err) => eprintln!("Error: {}", err),
/// }
/// ```
/// 
/// # Errors
///
/// This function may return an error if:
/// - The HTTP request fails to be sent.
/// - The response status code is not successful
/// - The response body cannot be converted to a string.
pub async fn query(query: AlphaVantageRangeQuery) -> Result<String> {
    let url = format!(
        "https://www.alphavantage.co/query?function={}&symbol={}&apikey={}&interval={}&outputsize={}",
        query.function,
        query.symbol,
        query.api_key,
        query.interval,
        query.output_size,
    );

    Client::new()
        .get(url).send()
        .await.context(format!("Failed to send request for query {:?}", query))?
        .text()
        .await.context(format!("Failed to get response text for query {:?}", query))
}