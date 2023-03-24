use std::str::FromStr;
use futures::StreamExt;
use reqwest::Url;
use tokio::io::AsyncWriteExt;
use tokio_binance::BINANCE_US_WSS_URL;

use crate::account;
use crate::binance_us_api;
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
      

    while running == false {
    
    println!("Please choose a base coin");
    let coin1 = get_input().to_ascii_uppercase();
    println!("Please choose a trading pair");
    let coin2 = get_input().to_ascii_uppercase();
    let pair = format!("{}{}", coin1, coin2);
    pairs.push(pair);
    println!("Would you like to add another streaming pair?Y/n");
    let resp = get_input();
    if resp.to_ascii_lowercase() == "n" {
        running = true;
    }
    }

    
    for pair in pairs {
        while running {

            let url = format!("wss://stream.binance.us:9443/ws/{}@bookTicker", pair.to_ascii_lowercase());

            let coin1 =format!("{}{}{}", 
            pair.to_string().chars().nth(0).unwrap(),
            pair.to_string().chars().nth(1).unwrap(),
            pair.to_string().chars().nth(2).unwrap()
        );
            let coin2 =format!("{}{}{}", 
            pair.to_string().chars().nth(3).unwrap(),
            pair.to_string().chars().nth(4).unwrap(),
            pair.to_string().chars().nth(5).unwrap()
    );
            

        println!("Trying: {}", url);


        let (socket, __) = tokio_tungstenite::connect_async(&url)
            .await
            .expect("Failed to connect");

        let (read, write) = socket.split();

        //let ws_to_stdout = {
            //read.for_each()(|message| async {
                //let data = message.unwrap().into_data();
                //tokio::io::stdout().write_all(&data).await.unwrap()
            //};)
        }


    let mut counter = 0;    
    };
}

pub fn terminate() {
    println!("Terminating program");
}




