use super::super::HubClient;
use super::auth::{Auth};

use paho_mqtt as mqtt;
use std::{env,thread,time::Duration};
use async_trait::async_trait;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use log::{error,warn,info};
use std::sync::Arc;
use futures::{executor::block_on, stream::StreamExt};

pub struct AzureIoTHub {
    connection_string: String,
    auth: Auth,
    mqtt_client: Option<mqtt::AsyncClient>,
    message_receiver: Option<Arc<mqtt::AsyncReceiver<Option<mqtt::Message>>>>,
}

impl AzureIoTHub {
    const RECONNECTION_TIME_MS: u64 = 2500;
    const IOT_HUB_RESULT_TOPIC: &str = "$iothub/twin/res/";

    pub fn new(connection_string: String) -> Result<Self, Error> {
        let auth_result = Auth::new(&connection_string);
        match auth_result {
            Err(e) => {
                Err(Error::new(ErrorKind::InvalidData, "Auth failed, will not try to connect: {e:?}"))
            }
            Ok(auth) => {
                Ok(AzureIoTHub {
                    connection_string,
                    auth,
                    mqtt_client: None,
                    message_receiver: None
                })
            }
        }
    }

    fn create_mqtt_client(&self, auth: &Auth) -> anyhow::Result<mqtt::AsyncClient> {
        let host = format!("ssl://{}:8883", auth.hostname());
        // let host = String::from("localhost");

        let client_id = auth.device_id();

        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(&host)
            .client_id(client_id)
            .finalize();
        let client = mqtt::AsyncClient::new(create_opts).unwrap();
        // let client = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        // println!("Error creating the MQTT clinet: {e:?}");
        // process.exit(1);
        // });
        Ok(client)
    }

    fn setup_mqtt_client(&mut self, mut client: mqtt::AsyncClient) -> mqtt::AsyncClient {
        info!("Will setup callback for MQTT client for IoT hub connection");
        client.set_connected_callback(|_client: &mqtt::AsyncClient| {
            info!("MQTT client connected");
        });

        client.set_connection_lost_callback(|client: &mqtt::AsyncClient| {
            info!("Connection lost. Attempting reconnect.");
            thread::sleep(Duration::from_millis(AzureIoTHub::RECONNECTION_TIME_MS));
            client.reconnect_with_callbacks(AzureIoTHub::on_connect_success, AzureIoTHub::on_connect_failure);
        });

        self.message_receiver = Some(Arc::new(client.get_stream(30)));

        client
    }

    fn connect_to_hub(&self, client: &mqtt::AsyncClient, auth: &Auth) -> anyhow::Result<bool> {
        let mut trust_store = env::current_dir().unwrap();
        trust_store.push("digicert.pem");

        if !trust_store.exists() {
            warn!("The trust store file does not exist: {:?}", trust_store);    
        }

        let ssl_opts = mqtt::SslOptionsBuilder::new()
            .trust_store(trust_store).unwrap()
            .enable_server_cert_auth(true)
            .verify(true)
            .finalize();

        let expiry = chrono::offset::Utc::now() + chrono::Duration::hours(24);
        let password_result = auth.generate_sas_token(&expiry);

        // let lwt = mqtt::Message::new("test", "Async subscriber lost connection", 1);
        match password_result {
            Err(e) => {
                warn!("Failed to generate password from auth {e:?}");
                Err(e)
            }
            Ok(password) => {
                let user_name = format!("{}/{}/?api-version=2021-04-12", auth.hostname(), auth.device_id());
                let conn_opts = mqtt::ConnectOptionsBuilder::new()
                    // .will_message(lwt)
                    .user_name(&user_name)
                    .password(&password)
                    .ssl_options(ssl_opts)
                    .finalize();

                info!("Will try to connect to hostname: {}, client id: {}, user name: {}, password: {}", auth.hostname(), client.client_id(), user_name, password);
                client.connect_with_callbacks(conn_opts, AzureIoTHub::on_connect_success, AzureIoTHub::on_connect_failure);
                Ok(true)
            }
        }
    }            

    fn create_subscriptions(client: &mqtt::AsyncClient) {
        // client.subscribe("#", 1);
        client.subscribe("$iothub/twin/res/#", 1);
        client.subscribe("$iothub/twin/PATCH/properties/desired/#", 1);
        // client.subscribe("iothub/twin/res/#", 1);
    }

    fn on_connect_success(client: &mqtt::AsyncClient, _msgid: u16) {
        info!("Connection succeeded");
        // let data = client.user_data().unwrap();

        // if let Some(lock) = data.downcast_ref::<UserTopics>() {
        // let topics = lock.read().unwrap();
        // println!("Subscribing to topics: {:?}", topics);

        // Create a QoS vector, same len as # topics
        // let qos = vec![QOS; topics.len()];
        // Subscribe to the desired topic(s).
        // client.subscribe_many(&topics, &qos);
        // TODO: This doesn't yet handle a failed subscription.
        // }
        AzureIoTHub::create_subscriptions(client);
        AzureIoTHub::get_full_twin(client);
    }

    // Callback for a failed attempt to connect to the server.
    // We simply sleep and then try again.
    //
    // Note that normally we don't want to do a blocking operation or sleep
    // from  within a callback. But in this case, we know that the client is
    // *not* conected, and thus not doing anything important. So we don't worry
    // too much about stopping its callback thread.
    fn on_connect_failure(client: &mqtt::AsyncClient, _msgid: u16, rc: i32) {
        warn!("Connection attempt failed with error code {}.\n", rc);
        thread::sleep(Duration::from_millis(AzureIoTHub::RECONNECTION_TIME_MS));
        client.reconnect_with_callbacks(AzureIoTHub::on_connect_success, AzureIoTHub::on_connect_failure);
    }

    fn parse_response_message(topic: &str, payload: String) -> bool {
        if !topic.starts_with(AzureIoTHub::IOT_HUB_RESULT_TOPIC) {
            return false;
        }

        let a: &str = &topic[AzureIoTHub::IOT_HUB_RESULT_TOPIC.len()..topic.len()];

        let parts: Vec<&str> = a.split("?").collect();

        let result_code = parts[0];

        let mut args = HashMap::new();
        let args_parts: Vec<&str> = parts[1].split("&").collect();
        for arg in args_parts {
            let arg_parts: Vec<&str> = arg.split("=").collect();
            args.insert(arg_parts[0], arg_parts[1]);
        }

        info!("Result: {}, args: {:?}", result_code, args);

        return true;
    }

    fn get_full_twin(client: &mqtt::AsyncClient) {
        info!("get full twin");
        AzureIoTHub::request_data_from_twin(client, "$iothub/twin/GET");
    }

    fn request_data_from_twin(client: &mqtt::AsyncClient, topic: &str) {
        let id = 123;
        let msg = mqtt::Message::new(format!("{topic}/?$rid={id}"), "", 1);
        // let pub = self.mqtt_client.publish(msg);
        client.publish(msg);
    }
}

#[async_trait]
impl HubClient for AzureIoTHub {
    fn is_connected(&self) -> anyhow::Result<bool> {
        Ok(false)
    }

    async fn connect(&mut self) -> anyhow::Result<bool> {
        let mut mqtt_client = self.create_mqtt_client(&self.auth)?;
        mqtt_client = self.setup_mqtt_client(mqtt_client);
        self.connect_to_hub(&mqtt_client, &self.auth)?;
        self.mqtt_client = Some(mqtt_client);

        Ok(true)
    }


    async fn disconnect(&self) -> anyhow::Result<bool> {
        // self.mqtt_client.disconnect();
        Ok(true)
    }

    async fn set_config_callback(&self, mut callback: impl FnMut(String) + Send) {
        info!("1");
        // let mut a = self.message_receiver;
        info!("2");
        if let Some(message_receiver) = self.message_receiver {
            info!("3");
            while let Some(mut message_result) = message_receiver.next().await {
                info!("4");
                if let Some(message) = message_result {
                    error!("Message: {message}");
                    callback("hej: {message.topic}".to_string());
                }
            }
        }
    }

    fn report(&self, path: String, payload: String) {

    }
}
