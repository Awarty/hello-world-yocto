use super::super::HubClient;
use async_trait::async_trait;
use super::auth::Auth;

use paho_mqtt as mqtt;
use std::{thread, process, time::Duration};

pub struct IoTHub {
    connection_string: String,
    //(online_status_sender, online_status_receiver): (Sender<bool>, Receiver<bool>)
}

impl IoTHub {
    pub fn new(connection_string: String) -> Self {
        IoTHub {
            connection_string,
            //online_status = oneshot::channel(),
        }
    }

    async fn connect_to_hub(connection_string: &String) {
        // https://learn.microsoft.com/en-us/azure/iot-hub/iot-hub-mqtt-support#using-the-mqtt-protocol-directly-as-a-device
        let auth_result = Auth::new(connection_string);

        match auth_result {
            Err(e) => {
                println!("Failed to validate connection string: {e:?}");
            }
            Ok(auth) => {
                let client_id = auth.device_id();
                let host = format!(
                    "{}://{}:{}",
                    "ssl".to_string(),
                    auth.hostname(),
                    "8883".to_string()
                );
                let user_name = format!(
                    "{}/{}/?api-version=2021-04-12",
                    auth.device_id(),
                    auth.hostname()
                );
                let expiry = chrono::offset::Utc::now() + chrono::Duration::hours(24);
                let password_result = auth.generate_sas_token(&expiry);
                //let client_id = "test";
                //let host = "tcp://test.mosquitto.org:1883";
                //let host = "tcp://test.mosquitto.org:8883";

                //let host = "ssl://broker.emqx.io:8883";
                //let host = "tcp://broker.emqx.io:1883";
                //let host = "ws://broker.emqx.io:8083";

                match password_result {
                    Err(e) => {
                        println!("Failed to generate SAS token: {e:?}");
                        return;
                    }
                    Ok(password) => {
                        let password1 = "SharedAccessSignature=SharedAccessSignature sr=syscomiotedgehubdev.azure-devices.net%2Fdevices%2Ftmm-2022-12-04&sig=YM9Vf%2FGNTJ6RdJt%2BEje3g2%2F20mLdO%2Bu3yqgZlVjzQt0%3D&se=1670948440";
                        println!("Will try to connect to IoT Hub using, host: {}, client id: {}, user name: {} and password {} and password1 {}", host, client_id, user_name, password, password1);

                        let create_opts = mqtt::CreateOptionsBuilder::new()
                            .server_uri(host)
                            .client_id(client_id)
                            .finalize();

                        let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
                            println!("Error creating the client: {:?}", e);
                            process::exit(1);
                        });

                        // Set a closure to be called whenever the client connection is established.
                        cli.set_connected_callback(|_cli: &mqtt::AsyncClient| {
                            println!("Connected.");
                        });

                         // Set a closure to be called whenever the client loses the connection.
                        // It will attempt to reconnect, and set up function callbacks to keep
                        // retrying until the connection is re-established.
                        cli.set_connection_lost_callback(|cli: &mqtt::AsyncClient| {
                            println!("Connection lost. Attempting reconnect.");
                            thread::sleep(Duration::from_millis(2500));
                            //tokio::time::sleep(Duration::from_millis(2500));
                            cli.reconnect_with_callbacks(IoTHub::on_connect_success, IoTHub::on_connect_failure);
                        });


                        // Attach a closure to the client to receive callback
                        // on incoming messages.
                        cli.set_message_callback(|_cli,msg| {
                            if let Some(msg) = msg {
                                let topic = msg.topic();
                                let payload_str = msg.payload_str();
                                println!("{} - {}", topic, payload_str);
                            }
                        });

                        // Define the set of options for the connection
                        let lwt = mqtt::Message::new("test", "Async subscriber lost connection", 1);

                        let conn_opts = mqtt::ConnectOptionsBuilder::new()
                            .keep_alive_interval(Duration::from_secs(20))
                            .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
                            .user_name(user_name)
                            //.password(password)
                            .password(password1)
                            .clean_session(true)
                            .will_message(lwt)
                            .finalize();

                        // Make the connection to the broker
                        println!("Connecting to the MQTT server...");
                        cli.connect_with_callbacks(conn_opts, IoTHub::on_connect_success, IoTHub::on_connect_failure);

                        // Define the set of options for the connection
                        // let lwt = mqtt::Message::new(
                        //     "test",
                        //     "Async subscriber lost connection",
                        //     mqtt::QOS_1,
                        // );

                        // let ssl_opts = mqtt::SslOptionsBuilder::new()
                        //     .trust_store(trust_store.to_str().unwrap())
                        //     .key_store(key_store.to_str().unwrap())
                        //     .finalize();

            //             let conn_opts = mqtt::ConnectOptionsBuilder::new()
            //                 .keep_alive_interval(Duration::from_secs(30))
            //                 // .user_name(user_name)
            //                 // .password(password)
            //                 .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
            //                 // .clean_session(false)
            //                 // .will_message(lwt)
            //                 // .ssl_options(ssl_opts)
            //                 .finalize();

            //             // Make the connection to the broker
            //             println!("Connecting to the MQTT server...");
            //             cli.connect(conn_opts).await?;

            //             // println!("Subscribing to topics: {:?}", topics);
            //             // cli.subscribe_many(topics, qos).await?;

            //             // Just loop on incoming messages.
            //             println!("Waiting for messages...");

            //             // Note that we're not providing a way to cleanly shut down and
            //             // disconnect. Therefore, when you kill this app (with a ^C or
            //             // whatever) the server will get an unexpected drop and then
            //             // should emit the LWT message.

            //             while let Some(msg_opt) = strm.next().await {
            //                 if let Some(msg) = msg_opt {
            //                     println!("fan {}", msg);
            //                 } else {
            //                     // A "None" means we were disconnected. Try to reconnect...
            //                     println!("Lost connection. Attempting reconnect.");
            //                     while let Err(err) = cli.reconnect().await {
            //                         println!("Error reconnecting: {}", err);
            //                         tokio::time::sleep(Duration::from_millis(1000)).await;
            //                     }
            //                 }
            //             }
                    }
                }
            }
        }

        // let topics: &[&str] = &["test", "hello"];
        // let qos: &[i32] = &[1, 1];

        // let host = env::args()
        //     .nth(1)
        //     .unwrap_or_else(|| "tcp://localhost:1883".to_string());

        // // Create the client. Use an ID for a persistent session.
        // // A real system should try harder to use a unique ID.
        // let create_opts = mqtt::CreateOptionsBuilder::new()
        //     .server_uri(host)
        //     .client_id("rust_async_subscribe")
        //     .finalize();

        // // Create the client connection
        // let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        //     println!("Error creating the client: {:?}", e);
        //     process::exit(1);
        // });

        // if let Err(err) = block_on(async {
        //     // Get message stream before connecting.
        //     let mut strm = cli.get_stream(25);

        //     // Define the set of options for the connection
        //     let lwt = mqtt::Message::new("test", "Async subscriber lost connection", mqtt::QOS_1);

        //     let conn_opts = mqtt::ConnectOptionsBuilder::new()
        //         .keep_alive_interval(Duration::from_secs(30))
        //         .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
        //         .clean_session(false)
        //         .will_message(lwt)
        //         .finalize();

        //     // Make the connection to the broker
        //     println!("Connecting to the MQTT server...");
        //     cli.connect(conn_opts).await?;

        //     println!("Subscribing to topics: {:?}", topics);
        //     cli.subscribe_many(topics, qos).await?;

        //     // Just loop on incoming messages.
        //     println!("Waiting for messages...");

        //     // Note that we're not providing a way to cleanly shut down and
        //     // disconnect. Therefore, when you kill this app (with a ^C or
        //     // whatever) the server will get an unexpected drop and then
        //     // should emit the LWT message.

        //     while let Some(msg_opt) = strm.next().await {
        //         if let Some(msg) = msg_opt {
        //             println!("{}", msg);
        //         } else {
        //             // A "None" means we were disconnected. Try to reconnect...
        //             println!("Lost connection. Attempting reconnect.");
        //             while let Err(err) = cli.reconnect().await {
        //                 println!("Error reconnecting: {}", err);
        //                 tokio::time::sleep(Duration::from_millis(1000)).await;
        //             }
        //         }
        //     }

        //     // Explicit return type for the async block
        //     Ok::<(), mqtt::Error>(())
        // }) {
        //     eprintln!("{}", err);
        // }
    }

    // Callback for a successful connection to the broker.
    // We subscribe to the topic(s) we want here.
    fn on_connect_success(cli: &mqtt::AsyncClient, _msgid: u16) {
        println!("Connection succeeded");
        // Subscribe to the desired topic(s).
        // cli.subscribe_many(TOPICS, QOS);
        // println!("Subscribing to topics: {:?}", TOPICS);
        // TODO: This doesn't yet handle a failed subscription.
    }

    // Callback for a failed attempt to connect to the server.
    // We simply sleep and then try again.
    //
    // Note that normally we don't want to do a blocking operation or sleep
    // from  within a callback. But in this case, we know that the client is
    // *not* conected, and thus not doing anything important. So we don't worry
    // too much about stopping its callback thread.
    fn on_connect_failure(cli: &mqtt::AsyncClient, _msgid: u16, rc: i32) {
        println!("Connection attempt failed with error code {}.\n", rc);
        // tokio::time::sleep(Duration::from_millis(2500));
        thread::sleep(Duration::from_millis(2500));
        cli.reconnect_with_callbacks(IoTHub::on_connect_success, IoTHub::on_connect_failure);
    }
}

#[async_trait]
impl HubClient for IoTHub {
    fn is_connected(&self) -> anyhow::Result<bool> {
        Ok(false)
    }

    async fn start(&self) {
        let connection_string = self.connection_string.clone();
        tokio::spawn(async move {
            IoTHub::connect_to_hub(&connection_string).await;

            loop {
                thread::sleep(Duration::from_millis(1000));
            }
        });
    }

    async fn disconnect(&self) -> anyhow::Result<bool> {
        Ok(true)
    }
}
