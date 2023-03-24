use std::fs::File;

use reqwest::StatusCode;
use reqwest;
use reqwest::header::{HeaderName, HeaderMap, HeaderValue};

use std::fs;
use std::path::Path;
use std::num::ParseFloatError;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{str::FromStr, ops::Deref, collections::HashMap};
use std::io::{stdout, Write, BufRead, BufReader, Error};

use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

use crate::utils;

pub const API_URL: &str = "https://api.binance.us";

pub fn get_creds() {
    let exists = Path::new(".config.json").exists();

    if exists {
        println!("config file found");

        let contents = fs::read_to_string(".config.json")
            .expect("could read config file");

        let keys: Value = serde_json::from_str(&contents).unwrap();
    

        let api_key = keys["api_key"].to_string().replace("\"", "");
        let secret =  keys["secret"].to_string().replace("\"", "");

        println!("API_KEY: {:?}", api_key);
        println!("SECRET: {:?}", secret);
    } else {
        println!("config not found");
        let mut settings = File::create(".config.json").expect("couldn't create file");
        println!("Please enter your api_key");
        let api_key = utils::get_input();
        println!("Please enter your secret");
        let secret = utils::get_input();
        println!("API KEY: {}", api_key);
        println!("SECRET: {}", secret);
        let json = serde_json::json!({
            "api_key": api_key,
            "secret": secret
    });
        println!("Writing to json file");
        println!("{}", json);
        write!(settings, "{}", json);
    }
}

//Retrieve UNIX time from server
pub async fn time() -> String {
    let result = reqwest::get("https://api.binance.us/api/v3/time")
        .await
        .expect("Something Happened")
        .text()
        .await
        .unwrap();
    let string = format!("{}", result);
    return string;
}

//Get exchange info
pub async fn get_exchange_info(coin1: &str, coin2: &str) -> Value {
    let url:String = format!("{}/api/v3/exchangeInfo?symbol={}{}", API_URL, coin1, coin2);
    println!("Getting Exchange Info");
    println!("Trying url: {}", url);
    let result = reqwest::get(&url)
        .await
        .expect("Something happened")
        .text()
        .await
        .unwrap();
    let string = format!("{}", result);
    let json: Value = serde_json::from_str(&string).unwrap();
    return json;
}

//Get recent trades for a given pair, example: get_trades("ETH", "BTC")
pub async fn get_trades(coin1: &str, coin2: &str) -> String {
    println!("Fetching Trades for Pair: {}/{}", coin1, coin2);
    let url = format!("{}/api/v3/trades?symbol={}{}", API_URL, coin1, coin2);
    println!("Trying {}", url);
    let result = reqwest::get(&url)
        .await
        .expect("Something happened")
        .text()
        .await
        .unwrap();
    let string = format!("{}", result);
    return string;
}
// Get current orderbook for a given pair, example get_orderbook_depth("ETH", "BTC")
pub async fn get_orderbook_depth(coin1: &str, coin2: &str) -> String {
    println!("Fetching Orderbook for {}/{}", coin1, coin2);
    let url = format!("{}/api/v3/depth?symbol={}{}", API_URL, coin1, coin2);
    println!("Trying {}", url);
    let result = reqwest::get(&url)
        .await
        .expect("Something Happened")
        .text()
        .await
        .unwrap();
    let string = format!("{}", result);
    return string;
}

pub async fn get_candles(coin1: &str, coin2: &str) -> String {
    println!("Fetching candlestick data for {}/{}", coin1, coin2);
    let url = format!("{}/api/v3/klines?symbol={}{}&interval=1m", API_URL, coin1, coin2);
    let result = reqwest::get(&url)
        .await
        .expect("Something happened")
        .text()
        .await
        .unwrap();
    let string = format!("{}", result);
    return string;
}

//get the price of coin1 denoted in coin2, example get_price("BTC", "USDT")
pub async fn get_price(coin1: &str, coin2: &str) -> f32 {
    //println!("Fetching price for {} denoted in {}", coin1, coin2);
    let url = format!("{}/api/v3/ticker/price?symbol={}{}", API_URL, coin1.to_ascii_uppercase(), coin2.to_ascii_uppercase());
    let result = reqwest::get(&url)
        .await
        .expect("Something happened")
        .text()
        .await
        .unwrap();
    let string = format!("{}", result.trim());
    let json: Value = serde_json::from_str(&string).unwrap();
    let jsonstring: String = json["price"].to_string().replace("\"", "");
    return jsonstring.parse::<f32>().unwrap();
}

pub async fn get_string(coin1: &str, coin2: &str) -> String {
    //println!("Fetching price for {} denoted in {}", coin1, coin2);
    let url = format!("{}/api/v3/ticker/price?symbol={}{}", API_URL, coin1, coin2);
    let result = reqwest::get(&url)
        .await
        .expect("Something happened")
        .text()
        .await
        .unwrap();
    let string = format!("{}", result.trim());
    let json: Value = serde_json::from_str(&string).unwrap();
    //let jsonstring: String = json["price"].to_string().replace("\"", "");
    return json.to_string();
}

pub async fn constraints_check(coin1: &str, coin2: &str) -> [f32; 7] {
    println!("Checking coinstraints for {} and {}", coin1, coin2);
    let info = get_exchange_info(coin1, coin2).await;
    
    let filters = serde_json::from_str::<Value>(&info["symbols"][0]["filters"].to_string()).unwrap();
    let maxprice = serde_json::from_str::<Value>(&info["symbols"][0]["filters"][0]["maxPrice"].to_string())
        .unwrap()
        .to_string()
        .replace("\"", "")
        .parse::<f64>();
    let minprice = serde_json::from_str::<Value>(&info["symbols"][0]["filters"][0]["minPrice"].to_string())
       .unwrap()
       .to_string()
       .replace("\"", "")
       .parse::<f64>();
    let min_notional = serde_json::from_str::<Value>(&info["symbols"][0]["filters"][3]["minNotional"].to_string())
       .unwrap()
       .to_string()
       .replace("\"", "")
       .parse::<f64>();
    let multiplier_up = serde_json::from_str::<Value>(&info["symbols"][0]["filters"][1]["multiplierUp"].to_string())
      .unwrap()
      .to_string()
      .replace("\"", "")
      .parse::<f64>();
    let multiplier_down = serde_json::from_str::<Value>(&info["symbols"][0]["filters"][1]["multiplierDown"].to_string())
     .unwrap()
     .to_string()
     .replace("\"", "")
     .parse::<f64>();
    let baseAssetPrecision = serde_json::from_str::<Value>(&info["symbols"][0]["baseAssetPrecision"].to_string())
     .unwrap()
     .to_string()
     .replace("\"", "")
     .parse::<f64>();
    let min_qty = serde_json::from_str::<Value>(&info["symbols"][0]["filters"][2]["minQty"].to_string())
     .unwrap()
     .to_string()
     .replace("\"", "")
     .parse::<f64>();
    let step_size: Result<f64, ParseFloatError> = Arc::new(serde_json::from_str::<Value>(&info["symbols"][0]["filters"][2]["stepSize"].to_string())
            .unwrap()
            .to_string()
            .replace("\"", ""))
            .parse::<f64>();
    //uncomment the following line for debugging purposes
    //println!("Full filters\n{:?}", filters);

    let maxprice_num: f32 = format!("{:?}", maxprice.ok().unwrap()).parse::<f32>().unwrap();
    let minprice_num: f32 = format!("{:?}", minprice.ok().unwrap()).parse::<f32>().unwrap();
    let min_notional_num: f32 = format!("{:?}", min_notional.ok().unwrap()).parse::<f32>().unwrap();
    let multiplier_up_num: f32 = format!("{:?}", multiplier_up.ok().unwrap()).parse::<f32>().unwrap();
    let base_asset_precision_num: f32 = format!("{:?}", baseAssetPrecision.ok().unwrap()).parse::<f32>().unwrap();
    let min_qty_num: f32 = format!("{:?}", min_qty.ok().unwrap()).parse::<f32>().unwrap();
    let step_size_num: f32 = format!("{:?}", step_size.ok().unwrap()).parse::<f32>().unwrap();

    return [maxprice_num, minprice_num, min_notional_num, multiplier_up_num, base_asset_precision_num, min_qty_num, step_size_num];
}


pub async fn arbitrage(api_key: &str, secret: &str, coin: &str, pairs: Vec<String>, spread: f32) {


    //let info = get_exchange_info(coin, "USDT").await;
    
    
    for pair in pairs.iter() {
        //get pricing data for the pair
        let data: f32 = get_price(coin, pair).await;
        //calculate the dollar value for the pair
        let dollarprice = get_price(pair, "USD").await;
        println!("{} dollarprice: {}", pair, dollarprice);
        println!("{} {}({}USD)", data, pair, data*dollarprice);
        //add fee to the calculation below

        println!("Target buy: {}", data*dollarprice*spread);
        if dollarprice*data < (data*dollarprice*spread) {
            println!("Arb detected, {}/{}", coin, pair);
            let array = constraints_check(coin, pair).await;
            let maxprice = array[0];
            let minprice = array[1];
            let min_notional = array[2];
            let multiplier_up = array[3];
            let baseAssetPrecision: i32 = array[4] as i32;
            let min_qty = array[5];
            let step_size: f32 = array[6] as f32;

            println!("Max Price: {}", maxprice);
            println!("Min Price: {}", minprice);
            println!("Min Notional: {}", min_notional);
            println!("Base Asset Precision: {}", baseAssetPrecision);
            println!("Going price {} {}", data, pair);
            println!("API Min Qty: {}", min_qty);
            println!("Min_Qty * price: {}", min_qty*minprice);
            println!("Multiplier_Up: {}", multiplier_up);
            println!("Multiplier_Up * price: {}", multiplier_up*minprice);
            println!("Stepsize: {}", step_size);


            let step_size_precision = utils::get_float_precision(step_size);

            println!("Stepsize (precision): {:?}", step_size_precision);

            println!("data * dollarprice = {}", data*dollarprice);
            let calc_min_qty =  utils::trim((min_notional / data), step_size_precision);
            println!("Calculated min_qty = {:?}", calc_min_qty);

            
            //buy the lower priced coin
            
            //sell coin into higher price pair


            let fmt_price = utils::trim(data, baseAssetPrecision);
            println!("Formatted price = {}", fmt_price);
            //let fmt_data = utils::trim(data, baseAssetPrecision);
            let fmt_qty = utils::trim(calc_min_qty*1.1, step_size_precision);
            
            place_order(api_key, secret, coin, pair, "BUY", fmt_qty, fmt_price).await;

            let price = utils::trim((data*dollarprice), 3);

            place_order(api_key, secret, coin, "USDT", "SELL", fmt_qty, price).await;


            //let jsoninfo = binance_us_api::get_exchange_info("ADA", pair).await;
        } else {
            println!("Pair trading tight, no Arb detected");
        }
        
    }
}


#[derive(Serialize)]
struct OrderRequest {
    symbol: String,
    side: String,
    r#type: String,
    time_in_force: String,
    quantity: f32,
    price: f32,
}

#[derive(Debug, Deserialize)]
struct OrderResponse {
    order_id: i64,
    client_order_id: String,
    transact_time: i64
}

async fn place_order(api_key: &str, secret: &str,coin1: &str, coin2: &str, side: &str, quantity: f32, price: f32,) {
    let api_key = api_key;
    let secret = secret;

    let client = reqwest::Client::new();

    let order_request = OrderRequest {
        symbol: format!("{}{}", coin1, coin2),
        side: side.to_string(),
        r#type: "LIMIT".to_string(),
        time_in_force: "GTC".to_string(),
        quantity: quantity,
        price: price
    };

    println!("Order symbol: {}", order_request.symbol);
    println!("Order Side: {}", order_request.side);
    println!("type: {}", order_request.r#type);
    println!("time_in_force: {}", order_request.time_in_force);
    println!("quantity: {}", order_request.quantity);
    println!("price: {}", order_request.price);

    let timestamp = SystemTime::now()
       .duration_since(UNIX_EPOCH)
       .unwrap()
       .as_millis();

    let mut query_params = HashMap::new();
        query_params.insert("symbol", order_request.symbol);
        query_params.insert("side", order_request.side);
        query_params.insert("type", order_request.r#type);
        query_params.insert("timeInForce", order_request.time_in_force);
        query_params.insert("quantity", order_request.quantity.to_string());
        query_params.insert("price", order_request.price.to_string());
        query_params.insert("timestamp", timestamp.to_string());

    let query_string = serde_urlencoded::to_string(&query_params).unwrap();

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(query_string.as_bytes());
    
    let signature = hex::encode(mac.finalize().into_bytes());

    let url = format!("{}/api/v3/order?{}&signature={}", API_URL, query_string, signature);

    println!("{}", url);

    println!("Signature {}", signature);


    let response = client
        .request(reqwest::Method::POST, &url)
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header("X-MBX-APIKEY", api_key)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("Response {:?}", response.to_string().replace("\"", ""));

}

pub fn create_signature(secret: &str) -> String {
    let secret_key = secret;

    let timestamp = SystemTime::now()
       .duration_since(UNIX_EPOCH)
       .unwrap()
       .as_millis();

    let mut query_params = HashMap::new();
        
        query_params.insert("timestamp", timestamp.to_string());

    let query_string = serde_urlencoded::to_string(&query_params).unwrap();

    let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes()).unwrap();
    mac.update(query_string.as_bytes());
    
    let signature = hex::encode(mac.finalize().into_bytes());

    return signature;
}



//UNDER CONSTRUCTION
/*
pub async fn get_account_info() -> String {
    let sig = create_signature();
    let api_key = API_KEY;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    
    let url = format!("{}/api/v3/account/signature?{}", API_URL, sig);
    println!("Trying url: {:?}", url);

    let client = reqwest::Client::new();

    let response = client
        .request(reqwest::Method::POST, &url)
        .header("X-MBX-APIKEY", api_key)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let response = format!("Response {:?}", response.to_string());

    return response;
}
*/
