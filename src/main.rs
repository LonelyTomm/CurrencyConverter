use std::io;
use reqwest;
use serde::Deserialize;

fn main() {
    let rate = get_curr_rate();

    loop {
        println!("Insert a value in KZT to convert to USD:");
        let mut amount = String::new();
        io::stdin().read_line(&mut amount).expect("Read error!");

        let amount: f64 = match amount.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Parse error!Expected a number");
                break
            }
        };

        let amount_converted: f64 = amount / rate;
        println!("Amount in USD is {:.2}", amount_converted);
    }
}

fn get_curr_rate() -> f64 {
    let client = reqwest::blocking::Client::new();

    #[derive(Deserialize)]
    struct Info {
        rate: f64
    };

    #[derive(Deserialize)]
    struct Resp {
        info: Info
    };

    let response = 
        client.get("https://api.exchangerate.host/convert?from=USD&to=KZT")
        .send().expect("Request error");

    let json: Resp = response.json().expect("parse json error");

    json.info.rate
}