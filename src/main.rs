use reqwest::{Client, Method, Url};
use reqwest;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};

use tokio::stream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
//use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures::{future, pin_mut, StreamExt};

use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};

use std::{time::SystemTime, time::UNIX_EPOCH, collections::HashMap, env};


use serde::{Deserialize, Serialize};
use serde_urlencoded;

use hmac::{Hmac, Mac};
use sha2::Sha256;



mod utils;
mod binance_us_api;

///////////MAIN PROGRAM////////////
#[tokio::main]
async fn main() {

    //Arbitrage function below for testing
    //WARNING THIS THING USES REAL MONEY AND WILL ACTUALLY SPEND YOUR CRYPTO
    println!("Binance US API Arbitrage Test");


    //attempt arbitrage 
    binance_us_api::arbitrage("ADA", &["BTC", "ETH", "USDC", "USDT"], 1.05).await;

}
////////////////END MAIN////////////////////

