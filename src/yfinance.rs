use serde_json::{Result, Value};
use ureq;

pub fn data() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
	let data = ureq::get("https://query1.finance.yahoo.com/v8/finance/chart/AAPL").call().into_string().unwrap();

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&data)?;

    // Access parts of the data by indexing with square brackets.
	println!("Market Price: {}", v["chart"]["result"][0]["meta"]["regularMarketPrice"]);

	Ok(())
}
