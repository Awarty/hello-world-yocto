
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CallViewModel {
    #[serde(rename = "stopId")]
    stop_id: String,
    #[serde(rename = "stopName")]
    stop_name: String,
    #[serde(rename = "stopSequence")]
    stop_sequence: u32,
    #[serde(rename = "arrivalTime")]
    arrival_time: DateTime<Utc>,
    #[serde(rename = "dropOff")]
    drop_off: bool,
    #[serde(rename = "index")]
    index: u32
}

impl CallViewModel {
    pub fn new(stop_id: String, stop_name: String, stop_sequence: u32, arrival_time: DateTime<Utc>, drop_off: bool, index: u32) -> Self{
        CallViewModel {
            stop_id: stop_id,
            stop_name: stop_name,
            stop_sequence: stop_sequence,
            arrival_time: arrival_time,
            drop_off: drop_off,
            index: index
        }
    }
}


#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CallsViewModel {
    #[serde(rename = "calls")]
    calls: Vec<CallViewModel>,
    #[serde(rename = "hash")]
    hash: String
}

impl CallsViewModel {
    pub fn new(calls: Vec<CallViewModel>) -> Self{
        CallsViewModel {
            calls: calls,
            hash: Uuid::new_v4().to_string()
        }
    }

    pub fn get_hash(&self) -> &str { &self.hash }
    
    pub fn verify_hash(&self, hash: &String) -> bool { hash.eq(&self.hash) }
}





