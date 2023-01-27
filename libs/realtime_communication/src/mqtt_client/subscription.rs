use tokio::sync::mpsc::{Sender, Receiver};
use tokio::sync::mpsc;
use uuid::Uuid;
use std::marker::PhantomData;
use serde_json;
use log::error;

pub fn create_subscription<T: serde::de::DeserializeOwned>(topic: String) -> (SubscriptionSender, SubscriptionClient<T>) {
    let (sender, mut receiver) = mpsc::channel(42);

    let id = Uuid::new_v4();

    (SubscriptionSender {
        id: id.clone(),
        topic,
        sender,
    },
    SubscriptionClient {
        id: id.clone(),
        receiver,
        phantom: PhantomData,
    })
}


pub struct SubscriptionSender {
    id: Uuid,
    topic: String,
    sender: Sender<String>,
}

impl SubscriptionSender {
    pub fn id(&self) -> &Uuid { &self.id }

    pub fn topic(&self) -> &str { &self.topic.as_str() }

    pub fn send(&self, data: String) {

    }
}

pub struct SubscriptionClient<T: serde::de::DeserializeOwned> {
    id: Uuid,
    receiver: Receiver<String>,
    phantom: PhantomData<T>,
}

impl<T: serde::de::DeserializeOwned> SubscriptionClient<T> {

    pub fn id(&self) -> &Uuid { &self.id }

    pub async fn receive(&mut self) -> Option<T> {
        // let data = self.receiver.recv().await;
        if let Some(data) = self.receiver.recv().await {
            let ret: Result<T, serde_json::Error> = serde_json::from_str(&data.as_str());
            match ret {
                Ok(ret) => Some(ret),
                Err(e) => {
                    error!("Failed to parse MQTT message: {e:?}");
                    None
                }
            }
        } else {
            None
        }
    }
}

