use std::str::FromStr;
use futures::StreamExt;
use reqwest::Url;
use tokio::io::AsyncWriteExt;
use tokio_binance::BINANCE_US_WSS_URL;
use tokio_binance::types::OrderBookParams;

use crate::account;
use crate::binance_us_api;
use crate::binance_us_api::constraints_check;
use crate::binance_us_api::get_price;
use crate::utils;
use crate::utils::get_input;

use tokio_binance::{AccountClient, BINANCE_US_URL, ID, WebSocketStream};
use tokio_binance::{Channel, Interval};

pub async fn scheduled_arb(api_key: &str, secret: &str) {
    println!("Please add a base coin:");

    let coin = utils::get_input().to_ascii_uppercase();
    println!("You entered: {:?}", coin);

    let mut pairs: Vec<String> = Vec::new();
    println!("Would you like to continue? (y/n):");
    let mut resp = utils::get_input().to_ascii_uppercase();
    if resp.to_ascii_lowercase() == "n" {
        println!("Exiting program");
        }
    else {
        while resp.to_ascii_lowercase()!= "n" {
            println!("Would you like to add a new pair? (y/n)");
            resp = utils::get_input().to_lowercase();
            if resp == "n" {
                println!("Exiting program");
                break;
            }
            println!("Please add a trading pair:");
            let new_pair = utils::get_input().to_ascii_uppercase();
            pairs.push(new_pair.to_string());
            println!("Your current pairs are: {}", pairs.join(","));
        }
        println!("Please choose a spread percentage");
        let mut spread = utils::get_input().parse::<f32>().unwrap();
        while spread > 1.0 {
            println!("Are you sure you want to buy high and sell low?");
            println!("Please enter a lower target spread ");
            spread = utils::get_input().parse::<f32>().unwrap();
        }
        
        println!("Please choose a schedule");
        let schedule = utils::get_input().parse::<u64>().unwrap();
        println!("We'll try an arbitrage every {} seconds", schedule);

        let pairs = pairs;


        let mut running = true;
            while running {
                binance_us_api::arbitrage(api_key, secret, &coin, pairs.to_owned(), spread).await;
                std::thread::sleep(std::time::Duration::from_secs(schedule));
            }
    }
}


pub async fn listen_and_react(api_key: &str, secret: &str) {
    let mut pairs: Vec<String> = Vec::new();

    let mut running = false;
    println!("Please enter a coin ticker");
    let coin1 = get_input().to_ascii_lowercase();
    println!("Please enter another coin");
    let coin2 = get_input().to_ascii_lowercase();

    let url = format!("wss://stream.binance.us:9443/ws/{}{}@bookTicker", coin1.to_ascii_lowercase(), coin2.to_ascii_lowercase());


    println!("Trying: {}", url);

    let (mut socket, _response) = tungstenite::connect(Url::parse(&url)
        .unwrap())
        .expect("can't connect!");


    println!("HTTP STATUS {}", _response.status());

    let mut counter: i32 = 0;


    loop {
        let msg = 
            socket.read_message().expect("error reading message");

            let json = 
                serde_json::Value::from_str(&msg.to_string()).expect("failed to parse message");
            println!("Message # {:?}", counter);
            counter += 1;
            let ask_price = match json["a"].to_string().replace("\"", "").parse::<f32>() {
                Ok(n) => n,
                __=> continue 
            };
            let ask_qty = match json["A"].to_string().replace("\"", "").parse::<f32>() {
                Ok(n) => n,
                __=> continue
            };
            println!("Ask price/qty {:?}/{:?}", ask_price, ask_qty);
            let bid_price = match json["b"].to_string().replace("\"", "").parse::<f32>() {
                Ok(n) => n,
                __ => continue
            };
            let bid_qty = match json["B"].to_string().replace("\"", "").parse::<f32>() {
                Ok(n) => n,
                __ => continue
            };
            println!("Bid price/qty {:?}/{:?}", bid_price, bid_qty);
            let balance1 = account::get_balance(api_key, secret, &coin1.to_ascii_uppercase()).await;
            let balance2 = account::get_balance(api_key, secret, &coin2.to_ascii_uppercase()).await;
            println!("{} balance: {}", coin1, balance1);
            println!("{} balance {}", coin2, balance2);
        }
}


pub async fn triangle_arb(api_key: &str, secret: &str) {
    let mut pairs: Vec<String> = Vec::new();

    let mut running = false;

    println!("Please choose a base coin");
    let coin = utils::get_input().to_ascii_lowercase();
      
    println!("Please enter a spread percentage");

    let spread = utils::get_input().parse::<f32>().expect("Please enter a valid number");
    while running == false {
    
        println!("Please choose a trading pair");
        let pair = get_input().to_ascii_uppercase();
        pairs.push(pair);
        println!("Would you like to add another trading pair?Y/n");
        let resp = get_input();
        if resp.to_ascii_lowercase() == "n" {
            running = true;
        }
    }

    let url = format!("wss://stream.binance.us:9443/ws/{}{}@bookTicker", coin, pairs[0].to_ascii_lowercase());


    let (mut socket, _response) = tungstenite::connect(Url::parse(&url)
        .unwrap())
        .expect("can't connect!");


    println!("HTTP STATUS {}", _response.status());
  

    println!("Trying: {}", url);

    let mut counter = 0;


     loop {
        let msg = 
            socket.read_message().expect("error reading message");

            let json = 
                serde_json::Value::from_str(&msg.to_string()).expect("failed to parse message");
            println!("Message # {:?}", counter);
            counter += 1;
            let ask_price = match json["a"].to_string().replace("\"", "").parse::<f32>() {
                Ok(n) => n,
                __=> continue 
            };
            let ask_qty = match json["A"].to_string().replace("\"", "").parse::<f32>() {
                Ok(n) => n,
                __=> continue
            };
            println!("Ask price/qty {:?}/{:?}", ask_price, ask_qty);
            let bid_price = match json["b"].to_string().replace("\"", "").parse::<f32>() {
                Ok(n) => n,
                __ => continue
            };
            let bid_qty = match json["B"].to_string().replace("\"", "").parse::<f32>() {
                Ok(n) => n,
                __ => continue
            };
            println!("Bid price/qty {:?}/{:?}", bid_price, bid_qty);
            binance_us_api::arbitrage(api_key, secret, &coin.to_ascii_uppercase(), pairs.clone(), spread).await;
            
        }
}


pub async fn DCA(api_key: &str, secret: &str) {
    println!("DCA Starting");
    println!("What coin would you like to buy?");
    let coin1 = get_input().to_ascii_uppercase();
    println!("What coin would you like to buy with?");
    let coin2 = get_input().to_ascii_uppercase();
    println!("You are choosing to buy {} with {}", coin1, coin2);
    println!("Please choose a schedule: min/hour/day/custom");

    
    let schedule = get_input().to_ascii_lowercase();
    let mut seconds = 0;
    if schedule == "min" {
        seconds = 60;
    } else if schedule == "hour" {
        seconds = 3600;
    } else if schedule == "day" {
        seconds = 86400;
    } else if schedule == "custom" {
        println!("Please enter the time (in seconds) that you would like between each buy");
        let input = get_input().parse::<i32>().expect("Please enter an integer");
        seconds = input;
    } else {
        println!("Please select a valid schedule");
        let input = get_input().parse::<i32>().expect("Please enter an integer");
        seconds = input;
    }

    println!("We'll try a buy order every {} seconds", seconds);
    println!("Checking constraints for {}/{}", coin1, coin2);
    let array = constraints_check(&coin1, &coin2).await;


            let mut running = false;

            let maxprice = array[0];
            let minprice = array[1];
            let min_notional = array[2];
            let multiplier_up = array[3];
            let baseAssetPrecision: i32 = array[4] as i32;
            let min_qty = array[5];
            let step_size: f32 = array[6] as f32;

            let data = binance_us_api::get_price(&coin1, &coin2).await;
            println!("\n\n\n");
            println!("CONSTRAINTS");
            println!("Max Price: {}", maxprice);
            println!("Min Price: {}", minprice);
            println!("Min Notional: {}", min_notional);
            println!("Base Asset Precision: {}", baseAssetPrecision);
            println!("API Min Qty: {}", min_qty);
            println!("Min_Qty * price: {}", min_qty*minprice);
            println!("Multiplier_Up: {}", multiplier_up);
            println!("Multiplier_Up * price: {}", multiplier_up*minprice);
            println!("Stepsize: {}", step_size);

            println!("\n\n\nPlease choose an order quantity larger than 'Calaculated minimum' listed below");
            let data = get_price(&coin1, &coin2).await;
            let precision = utils::get_float_precision(step_size);
            let calc_minimum = utils::trim((min_notional / data), precision);


            println!("Calculated min quantity: {}", calc_minimum);


            let order_qty = get_input().parse::<f32>().expect("Please enter a valid quantity");
            println!("We'll try to buy {} {} every {} seconds using {}", order_qty, coin1, seconds, coin2);

            let running = true;


            while running {
                let price = get_price(&coin1, &coin2).await;
                binance_us_api::place_order(api_key, secret, &coin1, &coin2, "BUY", order_qty, price).await;
                std::thread::sleep(std::time::Duration::from_secs(seconds.try_into().unwrap()));
            }

}

pub fn terminate() {
    println!("Terminating program");
}




