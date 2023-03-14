use crate::binance_us_api;
use crate::utils;

pub async fn scheduled_arb() {
    println!("Please add a base coin:");

    let coin = utils::get_input();
    println!("You entered: {:?}", coin);

    let mut pairs: Vec<String> = Vec::new();
    println!("Would you like to continue? (y/n):");
    let mut resp = utils::get_input();
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
            let new_pair = utils::get_input();
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
