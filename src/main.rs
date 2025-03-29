use std::fs;
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CosmeticItem {
    name: String,
    item_type: String,
}

struct GameClient {
    api_url: String,
}

impl GameClient {
    fn new() -> Self {
        GameClient {
            api_url: String::from("https://api.fortnite.com"),
        }
    }

    async fn unlock_item(&self, item: &CosmeticItem) -> Result<(), String> {
        let client = reqwest::Client::new();
        let response = client.post(format!("{}/unlock", self.api_url))
            .json(item)
            .send()
            .await;

        match response {
            Ok(res) if res.status().is_success() => Ok(()),
            Ok(_) => Err(String::from("Failed to unlock item")),
            Err(_) => Err(String::from("Network error")),
        }
    }
}

#[tokio::main]
async fn main() {
    let client = GameClient::new();
    let items = vec![
        CosmeticItem { name: String::from("Skull Trooper"), item_type: String::from("outfit") },
        CosmeticItem { name: String::from("Floss"), item_type: String::from("emote") },
    ];

    for item in items {
        match client.unlock_item(&item).await {
            Ok(_) => println!("Unlocked: {}", item.name),
            Err(e) => eprintln!("Error unlocking {}: {}", item.name, e),
        }
    }

    Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg("Fortnite")
        .spawn()
        .expect("Failed to start Fortnite");
}