use reqwest::{Client, Method, Url};
use reqwest;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};

use tokio::stream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::watch::error;
use tungstenite;
use futures::{future, pin_mut, StreamExt, FutureExt};


use std::io::Read;
use std::rc::Rc;
use std::{time::SystemTime, time::UNIX_EPOCH, collections::HashMap, env};


use serde::{Deserialize, Serialize};
use serde_urlencoded;

use hmac::{Hmac, Mac};
use sha2::Sha256;



mod utils;
mod binance_us_api;
mod strategies;

///////////MAIN PROGRAM////////////
#[tokio::main]
async fn main() {
    let strategies: [&str; 1] = ["scheduled_arb"];

    println!("Please select a strategy from the list: ");
    for strategy in strategies.iter() {
        let mut counter: i32 = 0;
    println!("{} {}",counter, strategy);
        counter += 1;
    }
    let resp = utils::get_input().parse::<i32>().unwrap();
    println!("You have selected: {}",resp);
    match resp {
        0 => strategies::scheduled_arb().await,
        __=> println!("Please select a valid strategy"),
    }

}
    //Arbitrage function below for testing
    //binance_us_api::arbitrage("ADA", &["BTC", "ETH", "USDC", "USDT"], 1.05  ).await;
    //println!("Binance US API Arbitrage Test");


    //Websocket Code....NOT WORKING YET
    /*
    let url = Url::parse("wss://binance.us:9443/ws/adabtc@bookTicker").unwrap();

    let request = Client::connect(url).unwrap();

    let response = request.send().unwrap();

    response.validate().unwrap();

    let mut client = response.begin();

    let (mut sender, mut receiver) = client.split();

    for message in receiver.incoming_messages() {
        let message: Message = message.unwrap();
        println!("Message: {:?}", message);
    }



    //let mut running = true;

    //println!("Connected to socket");
    */

    
////////////////END MAIN////////////////////

