use std::io;
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;

fn main() {
    let rates = get_rates();

    loop {
        println!("Insert a currency string to convert KZT to:");
        let mut currency = String::new();
        io::stdin().read_line(&mut currency).expect("Read error!");
        let currency = currency.trim().to_uppercase();

        if rates.contains_key(&currency) {
            println!("Found currency");
        } else {
            println!("Not found currency:(");
            continue;
        }

        println!("Insert a value in KZT to convert to {}:", currency);
        let mut amount = String::new();
        io::stdin().read_line(&mut amount).expect("Read error!");

        let amount: f64 = match amount.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Parse error!Expected a number");
                break
            }
        };
        
        let rate = rates.get(&currency).expect("No such key");
        let amount_converted: f64 = amount * rate;

        println!("Amount in {} is {:.2}", currency, amount_converted);
    }
}

fn get_rates() -> HashMap<String, f64> {
    let client = reqwest::blocking::Client::new();

    let response = 
        client.get("https://api.exchangerate.host/latest?base=KZT")
        .send().expect("Request error");

    #[derive(Deserialize)]
    struct Resp {
        rates: HashMap<String, f64>
    };

    let json: Resp = response.json().expect("parse json error");

    json.rates
}