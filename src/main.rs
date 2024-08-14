use rand::seq::IteratorRandom;
use reqwest::Proxy;
use serde_json::json;

#[tokio::main]
async fn main() {
    loop {
        let code = code_gen();

        let client = reqwest::Client::new();
        let res = client
            .get(format!("https://discord.com/api/v9/entitlements/gift-codes/{code}?with_application=false&with_subscription_plan=true"))
            .send().await.unwrap();
        let res_json = res.json::<serde_json::Value>().await.unwrap();
        match res_json["message"].as_str().unwrap() {
            "You are being rate limited." => {}
            "Unknown Gift Code" => {}
            "This gift has been redeemed already." => {}
            
            "The resource is being rate limited." => {
                println!("Rate limited");
                std::thread::sleep(std::time::Duration::from_micros(
                    (res_json["retry_after"].as_f64().unwrap() * 1000.0) as u64,
                ));
            }
            "You are being blocked from accessing our API temporarily due to exceeding global rate limits. Refer to https://discord.com/developers/docs/topics/rate-limits for more information." => {
                println!("Rate limited");
                std::thread::sleep(std::time::Duration::from_micros(
                    (res_json["retry_after"].as_f64().unwrap() * 1000.0) as u64,
                ));
            }
            _ => {
                println!("{}: {}", code, res_json);
            }
        }
        std::thread::sleep(std::time::Duration::from_micros(
            (0..2000).choose(&mut rand::thread_rng()).unwrap(),
        ));
    }
}

fn code_gen() -> String {
    let code_rng = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    let mut code = String::new();
    for i in 0..16 {
        code.push(code_rng.chars().choose(&mut rng).unwrap());
    }
    code
}
