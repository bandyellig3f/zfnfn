pub mod cosmetics {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct CosmeticItem {
        pub name: String,
        pub item_type: String,
    }

    pub fn get_all_items() -> Vec<CosmeticItem> {
        vec![
            CosmeticItem { name: String::from("Renegade Raider"), item_type: String::from("outfit") },
            CosmeticItem { name: String::from("Take the L"), item_type: String::from("emote") },
        ]
    }
}