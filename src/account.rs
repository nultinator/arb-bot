use std::collections::HashMap;
use std::str::FromStr;
use serde_json::Value;

use tokio_binance::{AccountClient, BINANCE_US_URL, ID, WebSocketStream};

pub async fn get_balances(api_key: &str, secret: &str) -> HashMap<String, f32> {
    let client = AccountClient::connect(
        api_key,
        secret,
        BINANCE_US_URL
    );
    let response = client
        .expect("failed to fetch account")
        .get_account()
        .json::<Value>()
        .await
        .unwrap();
    let balances = response["balances"].to_string();

    let value = serde_json::Value::from_str(&balances).unwrap();
    let value = value.as_array();
    
    let mut dict: HashMap<String, f32> = HashMap::new();
    for balance in value.unwrap().iter() {
        let asset = balance.get("asset").unwrap().to_string().replace("\"", "");
        let amount = balance.get("free").unwrap().to_string().replace("\"", "").parse::<f32>().unwrap();
        dict.insert(asset, amount);
    }

    return dict;
}

pub async fn get_balance(api_key: &str, secret: &str, coin: &str) -> f32 {
    let map: HashMap<String, f32> = get_balances(api_key, secret).await;
    map[coin]
}