use yahoo_finance_api as yahoo;
use tokio_test;

pub fn get_quote() -> std::string::String {
    let provider = yahoo::YahooConnector::new();
    // get the latest quotes in 1 minute interval

    let response = tokio_test::block_on(provider.get_latest_quotes("AAPL", "1m")).unwrap();
    // extract just the latest valid quote summery
    let quote = response.last_quote().unwrap();

    let quote = quote.close;

    let quote_close = format!("{:.2}", quote);

    return quote_close;
}
