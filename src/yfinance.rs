use yahoo_finance_api as yahoo;
use tokio_test;

pub fn get_quote() -> std::string::String {
    let provider = yahoo::YahooConnector::new();
    // get the latest quotes in 1 minute interval

    let response = tokio_test::block_on(provider.get_latest_quotes("AAPL", "1m")).unwrap();
    // extract just the latest valid quote summery
    let quote = response.last_quote().unwrap();

    let quote_close = quote.close;

    let quote_close_rounded = format!("{:.2}", quote_close);

    return quote_close_rounded;
}
