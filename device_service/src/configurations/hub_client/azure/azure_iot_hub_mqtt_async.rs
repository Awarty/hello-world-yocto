use super::super::HubClient;
use super::auth::{Auth};

use paho_mqtt as mqtt;
use std::{env,thread,time::Duration};
use async_trait::async_trait;
use std::collections::HashMap;

pub struct AzureIoTHubMqttAsync {
    connection_string: String,
}

impl AzureIoTHubMqttAsync {
    const RECONNECTION_TIME_MS: u64 = 2500;
    const IOT_HUB_RESULT_TOPIC: &str = "$iothub/twin/res/";

    pub fn new(connection_string: String) -> Self {
        AzureIoTHubMqttAsync {
            connection_string,
        }
    }

    fn on_connect_success(client: &mqtt::AsyncClient, _msgid: u16) {
        println!("Connection succeeded");
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
        AzureIoTHubMqttAsync::get_full_twin(client);
    }

    // Callback for a failed attempt to connect to the server.
    // We simply sleep and then try again.
    //
    // Note that normally we don't want to do a blocking operation or sleep
    // from  within a callback. But in this case, we know that the client is
    // *not* conected, and thus not doing anything important. So we don't worry
    // too much about stopping its callback thread.
    fn on_connect_failure(client: &mqtt::AsyncClient, _msgid: u16, rc: i32) {
        println!("Connection attempt failed with error code {}.\n", rc);
        thread::sleep(Duration::from_millis(AzureIoTHubMqttAsync::RECONNECTION_TIME_MS));
        client.reconnect_with_callbacks(AzureIoTHubMqttAsync::on_connect_success, AzureIoTHubMqttAsync::on_connect_failure);
    }

    fn parse_response_message(topic: &str, payload: String) -> bool {
        if !topic.starts_with(AzureIoTHubMqttAsync::IOT_HUB_RESULT_TOPIC) {
            return false;
        }

        let a: &str = &topic[AzureIoTHubMqttAsync::IOT_HUB_RESULT_TOPIC.len()..topic.len()];

        let parts: Vec<&str> = a.split("?").collect();

        let result_code = parts[0];

        let mut args = HashMap::new();
        let args_parts: Vec<&str> = parts[1].split("&").collect();
        for arg in args_parts {
            let arg_parts: Vec<&str> = arg.split("=").collect();
            args.insert(arg_parts[0], arg_parts[1]);
        }

        println!("Result: {}, args: {:?}", result_code, args);

        return true;
    }

    fn connect(&self, auth: &Auth) {
        let expiry = chrono::offset::Utc::now() + chrono::Duration::hours(24);
        let password_result = auth.generate_sas_token(&expiry);

        match password_result {
            Err(e) => {
                println!("Failed to generate password from auth {e:?}");
            }
            Ok(password) => {
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

                client.set_connected_callback(|_client: &mqtt::AsyncClient| {
                    println!("MQTT client connected");
                });

                client.set_connection_lost_callback(|client: &mqtt::AsyncClient| {
                    println!("Connection lost. Attempting reconnect.");
                    thread::sleep(Duration::from_millis(AzureIoTHubMqttAsync::RECONNECTION_TIME_MS));
                    client.reconnect_with_callbacks(AzureIoTHubMqttAsync::on_connect_success, AzureIoTHubMqttAsync::on_connect_failure);
                });

                client.set_message_callback(|_client, msg| {
                    if let Some(msg) = msg {
                        let topic = msg.topic();
                        let payload_str = msg.payload_str().to_string();

                        println!("Got message, topics: {}, payload: {}", topic, payload_str);

                        if AzureIoTHubMqttAsync::parse_response_message(topic, payload_str) {

                        } else {

                        }

                        // if topic == "" {
                        // let data = cli.user_data().unwrap();
                        // if let Some(lock) = data
                        // }
                    }
                });

                let mut trust_store = env::current_dir().unwrap();
                trust_store.push("digicert.pem");

                if !trust_store.exists() {
                    println!("The trust store file does not exist: {:?}", trust_store);    
                }

                let ssl_opts = mqtt::SslOptionsBuilder::new()
                    .trust_store(trust_store).unwrap()
                    .enable_server_cert_auth(true)
                    .verify(true)
                    .finalize();

                // let lwt = mqtt::Message::new("test", "Async subscriber lost connection", 1);
                let user_name = format!("{}/{}/?api-version=2021-04-12", auth.hostname(), auth.device_id());
                let conn_opts = mqtt::ConnectOptionsBuilder::new()
                    // .will_message(lwt)
                    .user_name(&user_name)
                    .password(&password)
                    .ssl_options(ssl_opts)
                    .finalize();

                println!("Will try to connect to hostname: {}, client id: {}, user name: {}, password: {}", host, client_id, user_name, password);
                client.connect_with_callbacks(conn_opts, AzureIoTHubMqttAsync::on_connect_success, AzureIoTHubMqttAsync::on_connect_failure);

                        

                // let mut acc = 0; 
                // let mut running = true;
                // while running {
                // loop {
                    // if client.is_connected() {
                    //     println!("Client is connected!");
                    //     let topic = format!("$iothub/twin/PATCH/properties/reported/?$rid={}", acc);
                    //     let payload = format!("{{\"number\": \"{}\"}}", acc);
                    //     let report_msg = mqtt::Message::new(topic, payload, 1);
                    //     client.publish(report_msg);
                    // }
                    // else {
                    //     println!("Client is disconnected!");
                    // }
                    // acc = acc + 1;
                    // thread::sleep(Duration::from_millis(1000));
                    // match is_running_recv.try_recv() {
                        // Err(e) => println!("Error when getting is running: {e:?}"),
                        // Ok(is_running) => {
                            // running = is_running;
                        // }
                    // }
                // }
            }
        }
    }

    fn get_full_twin(client: &mqtt::AsyncClient) {
        println!("get full twin");
        // client.subscribe("#", 1);
        client.subscribe("$iothub/twin/res/#", 1);
        client.subscribe("$iothub/twin/PATCH/properties/desired/#", 1);
        // client.subscribe("iothub/twin/res/#", 1);
        let msg = mqtt::Message::new("$iothub/twin/GET/?$rid=123", "", 1);
        // let msg = mqtt::Message::new("fuuu", "", 1);
        let pub1 = client.publish(msg);
    }
}

#[async_trait]
impl HubClient for AzureIoTHubMqttAsync {
    fn is_connected(&self) -> anyhow::Result<bool> {
        Ok(false)
    }

    async fn start(&self) {
        let auth_result = Auth::new(&self.connection_string);
        match &auth_result {
            Err(e) => {
                println!("Auth failed, will not try to connect: {e:?}");
            }
            Ok(a) => {
                // let auth = a.clone();
                // let is_running_recv = self.exit_recv.clone();
                // if self.exit_send.
                // self.thread_handle = Some(tokio::spawn(async move {
                // tokio::spawn(async move {
                self.connect(a);

                    // loop {
                        // println!("Azure iot hub mqtt does stuff!");
                        // thread::sleep(Duration::from_millis(1000));
                    // }
                // });
            }
        }
    }

    async fn disconnect(&self) -> anyhow::Result<bool> {
        // match self.thread_handle {
        //     None => {
        //         println!("IoT Hub connection thread not created, can not force it to exit");
        //         Ok(false)
        //     },
        //     Some(handle) => {
        //         handle.await;
        //         Ok(true)
        //     }
        // }
        Ok(true)
    }

    // fn config_changes(&self) -> anyhow::Result<Receiver<String>> {

    // }
}
