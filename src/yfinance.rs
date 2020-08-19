pub fn get_quote() -> &'static str {
//	let provider = yahoo::YahooConnector::new();
	// get the latest quotes in 1 minute interval

//	let response = tokio_test::block_on(provider.get_latest_quotes("AAPL", "1m")).unwrap();
	//extract just the latest valid quote summery
//	let quote = response.last_quote().unwrap();

//	let quote = quote.close;

//	let quote_close = format!("{:.2}", quote);

	let quote_current = "400";

	return quote_current;
}
