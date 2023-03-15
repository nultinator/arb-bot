use std::str::FromStr;
use reqwest::Url;

use crate::binance_us_api;
use crate::utils;
use crate::utils::get_input;

pub async fn scheduled_arb() {
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
        let spread = utils::get_input().parse::<f32>().unwrap();
        
        println!("Please choose a schedule");
        let schedule = utils::get_input().parse::<u64>().unwrap();
        println!("We'll try an arbitrage every {} seconds", schedule);

        let pairs = pairs;


        let mut running = true;
            while running {
                binance_us_api::arbitrage(&coin, pairs.to_owned(), spread).await;
                std::thread::sleep(std::time::Duration::from_secs(schedule));
            }
    }
}


pub fn listen_and_react() {
    println!("Please choose a base coin");
    let coin1 = get_input().to_ascii_uppercase();
    println!("Please choose a trading pair");
    let coin2 = get_input().to_ascii_uppercase();

    let url = format!("wss://stream.binance.us:9443/ws/{}{}@bookTicker", coin1.to_ascii_lowercase(), coin2.to_ascii_lowercase());

    let (mut socket, _response) = tungstenite::connect(Url::parse(&url)
        .unwrap())
        .expect("can't connect!");

    println!("HTTP STATUS {}", _response.status());

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
    }
}
