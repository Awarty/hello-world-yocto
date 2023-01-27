
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DisplayModeViewModel {
    #[serde(rename = "mode")]
    display_mode: u32,
    hash: String
}

impl DisplayModeViewModel {
    pub fn new(display_mode: u32) -> Self{
        DisplayModeViewModel {
            display_mode: display_mode,
            hash: Uuid::new_v4().to_string()
        }
    }

    pub fn get_hash(&self) -> &str { &self.hash }

    pub fn verify_hash(&self, hash: &String) -> bool { hash.eq(&self.hash) }
}


