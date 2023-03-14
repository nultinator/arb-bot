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
use std::str::FromStr;
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
    let strategies: [&str; 2] = ["scheduled_arb", "listen and react"];

    let mut counter = 0;

    println!("Please select a strategy from the list: ");
    for strategy in strategies.iter() {
    println!("{} {}",counter, strategy);
        counter += 1;
    }
    let resp = utils::get_input().parse::<usize>().unwrap();
    println!("You have selected: {} {}",resp, strategies[resp]);
    match resp {
        0 => strategies::scheduled_arb().await,
        1 => strategies::listen_and_react(),
        __=> println!("Please select a valid strategy"),
    }
}




////////////////END MAIN////////////////////

