
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinationViewModel {
    #[serde(rename = "routeName")]
    route_name: String,
    #[serde(rename = "text")]
    text: String,
    #[serde(rename = "alternativeText")]
    alternative_text: Vec<String>,
    #[serde(rename = "lineColor")]
    line_color: String,
    #[serde(rename = "hash")]
    hash: String
}

impl DestinationViewModel {
    pub fn new(route_name: impl Into<String>, text: impl Into<String>, alternative_text: Vec<String>, line_color: impl Into<String>) -> Self{
        DestinationViewModel {
            route_name: route_name.into(),
            text: text.into(),
            alternative_text: alternative_text,
            line_color: line_color.into(),
            hash: Uuid::new_v4().to_string()
        }
    }

    pub fn get_hash(&self) -> &str { &self.hash }
    pub fn verify_hash(&self, hash: &String) -> bool { hash.eq(&self.hash) }
}

