use std::collections::HashMap;

pub async fn alert(msg: &str) {
    let webhook = std::env::var("DISCORD_WEBHOOK").expect("missing DISCORD_WEBHOOK");

    let max_length = 1900.min(msg.len());
    let message = msg[..max_length].to_string();
    let mut map = HashMap::new();
    map.insert("content", message.to_string());
    let client = reqwest::Client::new();
    let res = client.post(webhook.to_string()).json(&map).send().await;
    match res {
        Ok(_) => {}
        Err(err) => {
            println!("Could not send alert to discord, err: {}", err);
            println!("Message: {}", message);
        }
    }
}
