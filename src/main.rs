use reqwest::Client;
use reqwest::Proxy;
use serde_json::json;
use std::time::Duration;
use tokio_socks::tcp::Socks5Stream;
use url::Url;

#[tokio::main]
async fn main() {
    let code = "3CE9mU3hDXC4c2Q6";

    let headers = json!({
        // chrome
        "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) discord/0.0.314 Chrome/124.0.6367.243 Electron/30.2.0 Safari/537.36",
        "timeout": 10000,
        "validateStatus": null,

    });
    let client = reqwest::Client::new();
    let res = client
        .get(format!("https://discord.com/api/v9/entitlements/gift-codes/{code}?with_application=false&with_subscription_plan=true"))
        .json(&headers)
        .send().await.unwrap();
    println!("{:?}", res.text().await.unwrap());
}
