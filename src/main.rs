use binance_us_api::get_creds;
use reqwest::{Client, Method, Url};
use reqwest;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};

use serde_json::Value;
use tokio::stream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::watch::error;
use tungstenite;
use futures::{future, pin_mut, StreamExt, FutureExt};


use same_file::Handle;

use std::path::Path;
use std::fs::{File, self};
use std::io::{Write, BufRead, BufReader, Error, Read};
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
    
    /*
    binance_us_api::get_creds();
    
    let settings = fs::read_to_string(".config.json").unwrap();
    let json: Value = serde_json::from_str(&settings).unwrap();

    let api_key = json["api_key"].to_string().replace("\"", "");
    let secret = json["secret"].to_string().replace("\"", "");
    //let buffered = BufReader::new(file);
    */
    
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

