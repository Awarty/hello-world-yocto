pub mod subscription;

use paho_mqtt as mqtt;
use log::{debug, info, warn, error};
use std::{thread,time::Duration};
use futures::{executor::block_on, stream::StreamExt};
use uuid::Uuid;
use subscription::SubscriptionSender;
use std::sync::{Arc, Mutex};
use std::todo;

pub struct MqttClient {
    mqtt_client: Option<mqtt::AsyncClient>,
    subscriptions: Arc<Mutex<Vec<SubscriptionSender>>>,
}

impl MqttClient {
    const RECONNECTION_TIME_MS: u64 = 2500;

    pub fn new() -> Self {
        MqttClient {
            mqtt_client: None,
            subscriptions: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn create_mqtt_client(&self, host: String, port: u32, client_id: String) -> mqtt::AsyncClient {
        let host = format!("{}:{}", host, port);
        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(host)
            .client_id(client_id)
            .finalize();
        mqtt::AsyncClient::new(create_opts).unwrap()
    }

    fn setup_mqtt_client(&self, client: &mqtt::AsyncClient) { 
        info!("Setting up MQTT client for localhost.");
        client.set_connected_callback(|_client: &mqtt::AsyncClient| {
            info!("MQTT Client Connected.");
        });

        client.set_connection_lost_callback(|_client: &mqtt::AsyncClient| {
            info!("MQTT Client lost connection.");
            thread::sleep(Duration::from_millis(MqttClient::RECONNECTION_TIME_MS));
            _client.reconnect_with_callbacks(MqttClient::on_connect_success, MqttClient::on_connect_failure);
        });
    }

    async fn connect_to_broker(&self, mut client: mqtt::AsyncClient) -> mqtt::AsyncClient {

        let mut message_receiver = client.get_stream(30);
        let conn_opts = mqtt::ConnectOptionsBuilder::new().finalize();

        client.connect_with_callbacks(conn_opts, MqttClient::on_connect_success, MqttClient::on_connect_failure).await;

        let subscriptions = self.subscriptions.clone();
        tokio::spawn(async move {
            loop {
                let _ = tokio::select! {
                    Some(message_result) = message_receiver.next() => {
                        if let Some(message) = message_result {
                            MqttClient::handle_message(&message, subscriptions.clone());
                            warn!("Message: {message}");
                        }
                    }
                    // Some(subscriptions)
                };
            }

            // while let Some(message_result) = message_receiver.next().await {
            //     if let Some(message) = message_result {
            //         warn!("Message: {message}");
            //     }
            // }
        });

        client
    }

    fn topic_compare(subscription_topic: &str, message_topic: &str) -> bool {
        if subscription_topic == message_topic {
            return true;
        }

        // TODO
        // subscription_topic can contains wildcard +, e.g:
        // message_topic abc/123/xzy/ should match subscription_topic abc/+/xzy/
        // message_topic abc/123/xzy/ should match subscription_topic abc/#, # etc.
        todo!();

        false
    }

    fn handle_message(message: &mqtt::Message, subscriptions: Arc<Mutex<Vec<SubscriptionSender>>>) {
        let subscriptions_lock = subscriptions.lock();
        match subscriptions_lock {
            Ok(subs) => {
                for sub in subs.iter() {
                    if MqttClient::topic_compare(sub.topic(), message.topic()) {
                        sub.send(String::from(message.payload_str()));
                    }
                }
            },
            Err(e) => {
                error!("Failed to handle message, {e:?}");
            }
        }
    }

    pub async fn connect_to_localhost(&mut self, client_id: String) -> anyhow::Result<()> {
        self.connect(String::from("localhost"), 1883, client_id).await
    }

    pub async fn connect(&mut self, host: String, port: u32, client_id: String) -> anyhow::Result<()> {
        let mut mqtt_client = self.create_mqtt_client(host, port, client_id);
        self.setup_mqtt_client(&mqtt_client);
        mqtt_client = self.connect_to_broker(mqtt_client).await;

        self.mqtt_client = Some(mqtt_client);
        info!("Successful setup of MQTT client.");

        if let Some(client) = &self.mqtt_client {
            client.subscribe("#", 1);
        }
        Ok(())
    }

    pub fn subscribe(&mut self, subscription: SubscriptionSender) -> bool {
        let subscription_lock = self.subscriptions.lock();
        match subscription_lock {
            Ok(mut subscriptions) => {
                subscriptions.push(subscription);
                return true;
            },
            Err(e) => {
                error!("Failed to subscribe, {e:?}");
                return false;
            }
        }
    }

    pub fn unsubscribe(&mut self, id: Uuid) -> bool {
        let subscription_lock = self.subscriptions.lock();
        match subscription_lock {
            Ok(mut subscriptions) => {
                if let Some(index) = subscriptions.iter().position(|x| *x.id() == id) {
                    subscriptions.remove(index);
                    return true;
                }
                return true;
            },
            Err(e) => {
                error!("Failed to unsubscribe, {e:?}");
                return false;
            }
        }
    }

    pub async fn publish_msg_mqtt(&self, topic: &str, msg: &str) {
        let mqtt_msg = mqtt::Message::new(topic, msg, mqtt::QOS_1);
        match &self.mqtt_client {
            None => warn!("Cannot publish to broker, client is not created"),
            Some(client) => {
                client.publish(mqtt_msg).await;
            }
        }
        info!("Published on topic: '{}' the message: '{}'", topic, msg);
    }

    fn create_subscriptions(client: &mqtt::AsyncClient) {
        // client.subscribe("#", 1);

        debug!("Created all subscriptions.");
    }

    fn on_connect_success(client: &mqtt::AsyncClient, _msgid: u16) {
        info!("Connection succeeded.");
        MqttClient::create_subscriptions(client);
    }

    fn on_connect_failure(_client: &mqtt::AsyncClient, msgid: u16, rc: i32) {
        warn!("Connection failed msgid: {msgid} rs {rc}.");
    }
}

