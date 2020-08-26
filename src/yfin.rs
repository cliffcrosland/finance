use serde_json::{Value, Result};
use ureq;

pub fn quote(symbol:String) -> String {

	let url = format!("https://query1.finance.yahoo.com/v8/finance/chart/{}/", symbol);

	let data = ureq::get(&url).call().into_string().unwrap();

	//Parse the string of data into serde_json::Value.
	let v = serde_json::from_str(&data);

	if v.is_ok() {
		let p: Value = v.unwrap();
		let marketprice = format!("{}", p["chart"]["result"][0]["meta"]["regularMarketPrice"]);
		return marketprice;
	} else {
		let error = "Could not get data";
		println!("{}", error);
		return error.to_string();
	}
}
