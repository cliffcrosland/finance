use serde_json::Value;
use ureq;

pub fn quote(symbol: &str) -> std::result::Result<String, String> {
    let url = format!("https://query1.finance.yahoo.com/v8/finance/chart/{}", symbol);

    let response = ureq::get(&url).call();
    if !response.ok() {
        return Err(format!("Error getting stock. status: {}, status_text: {}",
                             response.status(), response.status_text()));
    }

    let data = match response.into_string() {
        Ok(data) => data,
        Err(error) => {
            return Err(format!("Error transforming to string: {}", error));
        }
    };

    // Parse the string of data into serde_json::Value.
    let parsed_json: Value = match serde_json::from_str(&data) {
        Ok(parsed_json) => parsed_json,
        Err(error) => {
            return Err(format!("Error parsing json: {}", error));
        }
    };

    let regular_market_price  = match &parsed_json["chart"]["result"][0]["meta"]["regularMarketPrice"] {
        Value::Number(regular_market_price) => regular_market_price,
        _ => {
            return Err("Unable to find regularMarketPrice in parsed JSON".to_string());
        }
    };
    match regular_market_price.as_f64() {
        Some(f64_value) => Ok(f64_value.to_string()),
        None => Err("Unexpected regularMarketPrice. Not a float 64".to_string()),
    }
}
