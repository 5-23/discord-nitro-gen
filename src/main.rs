use rand::seq::IteratorRandom;
use reqwest::blocking::Client;
use serde_json::Value;
use std::{thread, time::Duration};

fn main() {
    let client = Client::new();

    loop {
        let code = code_gen();

        let res = client
            .get(format!("https://discord.com/api/v9/entitlements/gift-codes/{code}?with_application=false&with_subscription_plan=true"))
            .send()
            .unwrap();

        let res_json = res.json::<Value>().unwrap();

        match res_json["message"].as_str().unwrap() {
            "You are being rate limited." => {}
            "Unknown Gift Code" => {}
            "This gift has been redeemed already." => {}

            "The resource is being rate limited." => {
                let retry_after_ms = res_json["retry_after"].as_f64().unwrap() * (0..2000).choose(&mut rand::thread_rng()).unwrap() as f64 + 60000.;
                println!("Rate limited for {} ms", retry_after_ms);
                thread::sleep(Duration::from_millis(retry_after_ms as u64));
            }
            "You are being blocked from accessing our API temporarily due to exceeding global rate limits. Refer to https://discord.com/developers/docs/topics/rate-limits for more information." => {                
                let retry_after_ms = res_json["retry_after"].as_f64().unwrap() * (0..2000).choose(&mut rand::thread_rng()).unwrap() as f64 + 60000.;
                println!("Rate limited for {} ms", retry_after_ms);
                thread::sleep(Duration::from_millis(retry_after_ms as u64));
            }
            _ => {
                println!("{}: {}", code, res_json);
            }
        }

        thread::sleep(Duration::from_millis(
            (1000..2000).choose(&mut rand::thread_rng()).unwrap(),
        ));
    }
}

fn code_gen() -> String {
    let code_rng = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    let mut code = String::new();
    for _ in 0..16 {
        code.push(code_rng.chars().choose(&mut rng).unwrap());
    }
    code
}
