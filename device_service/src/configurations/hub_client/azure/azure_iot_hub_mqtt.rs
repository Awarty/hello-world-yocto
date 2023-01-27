use super::super::HubClient;
use super::auth::{Auth};
use std::io::{ErrorKind, Error};
use async_trait::async_trait;
use mqtt::topic;
use paho_mqtt as mqtt;
use std::{env,thread,time::Duration,process};

pub struct AzureIoTHubMqtt {
    connection_string: String,
}

impl AzureIoTHubMqtt {
    pub fn new(connection_string: String) -> Self {
        AzureIoTHubMqtt {
            connection_string
        }
    }

    fn try_reconnect(cli: &mqtt::Client) -> bool {
        println!("Connection lost. Waiting to retry connection");
        for _ in 0..12 {
            thread::sleep(Duration::from_millis(5000));
            if cli.reconnect().is_ok() {
                println!("Successfully reconnected");
                return true;
            }
        }
        println!("Unable to reconnect after several attempts.");
        false
    }

    fn data_handler(msg: mqtt::Message) -> bool {
        println!("{}", msg);
        true
    }

    fn command_handler(msg: mqtt::Message) -> bool {
        let cmd = msg.payload_str();
        if cmd == "exit" {
            println!("Exit command received");
            false
        }
        else {
            println!("Received command: '{}'", cmd);
            true
        }
    }

    fn sub_id(id: i32) -> mqtt::Properties {
        mqtt::properties![
            mqtt::PropertyCode::SubscriptionIdentifier => id
        ]
    }
 
    fn connect(auth: &Auth) {
        let host = format!("ssl://{}:8883", auth.hostname());
        // let host = String::from("localhost");
        let client_id = auth.device_id();

        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(&host)
            // .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
            .client_id(client_id)
            .finalize();
            // .create_client();

        let client_result = mqtt::Client::new(create_opts);        

        //let mut client_result = mqtt::AsyncClient::new(create_opts);
        match client_result {
            Err(e) => {
                println!("Error creating MQTT client: {e:?}");
            }
            Ok(client) => {
                let rx = client.start_consuming();

                let expiry = chrono::offset::Utc::now() + chrono::Duration::hours(24);
                let password_result = auth.generate_sas_token(&expiry);

                match password_result {
                    Err(e) => {
                        println!("Failed to generate password from auth {e:?}");
                    }
                    Ok(password) => {
                        let user_name = format!(
                            "{}/{}/?api-version=2021-04-12",
                            auth.hostname(),
                            auth.device_id()
                        );
                        
                        let mut trust_store = env::current_dir().unwrap();
                        trust_store.push("digicert.pem");

                        if !trust_store.exists() {
                            println!("The trust store file does not exist: {:?}", trust_store);
                            println!("  Get a copy from \"paho.mqtt.c/test/ssl/digicert.pem\"");
                            process::exit(1);
                        }
                        // let trust_store = "/home/tmm/Documents/dev/places/APP-APPMOD-EDGE/GptEdge/device_service/digicert.pem";
                        //let trust_store = Auth:: 
                        let ssl_opts = mqtt::SslOptionsBuilder::new()
                            // File name of the trust store
                            .trust_store(trust_store).unwrap() // CString
                            // File name of the client's public key store.
                            // .key_store("").unwrap() // CString
                            // File name for the private key, if not in key store.
                            // .private_key(trust_store).unwrap() // CString:
                            // The list of cipher quites that the client presents to the server.
                            // .enable_cipher_suits() // CString
                            // The path to the CA certificates, if specified.
                            // .ca_path(trust_store).unwrap() // CString
                            // The list of ALPN protocols availabel to be negotiated.
                            // .protos() // vec<c_uchar>
                            .enable_server_cert_auth(true)
                            .verify(true)
                            // .ssl_version(mqtt::SslVersion::Tls_1_2)
                            .finalize();

                        // ssl_op

                        let conn_opts = mqtt::ConnectOptionsBuilder::new()                        
                            //.clean_start(false)
                            .user_name(&user_name)
                            .password(&password)
                            // .clean_session(true)
                            .ssl_options(ssl_opts)
                            // .properties(mqtt::properties![mqtt::PropertyCode::SessionExpiryInterval => 3600])
                            //.will_message(lwt)
                            .finalize();

                        let handler: Vec<fn(mqtt::Message) -> bool> = vec![AzureIoTHubMqtt::data_handler, AzureIoTHubMqtt::command_handler];

                        println!("Will try to connect to hostname: {}, client id: {}, username: {}, password: {}", host, client_id, user_name, password);

                        let rsp_result = client.connect(conn_opts);

                        match rsp_result {
                            Err(e) => {
                                println!("Failed to connect to broker {e:?}");
                            }
                            Ok(rsp) => {
                                if let Some(conn_rsp) = rsp.connect_response() {
                                    println!(
                                        "Connected to: '{}' with MQTT version {}",
                                        conn_rsp.server_uri, conn_rsp.mqtt_version
                                    );

                                    if conn_rsp.session_present {
                                        println!("  w/ client session already present on broker.");
                                    }
                                    else {
                                        // Register subscriptions on the server, using Subscription ID's.
                                        println!("Subscribing to topics...");
                          
                                        match client.subscribe_with_options("$iothub/twin/res/#", 1, None, AzureIoTHubMqtt::sub_id(1)) {
                                            Err(e) => {
                                                println!("Error when subscribing {e:?}");
                                            }
                                            Ok(..) => {

                                            }
                                        }
                                        match client.subscribe_with_options("command", 1, None, AzureIoTHubMqtt::sub_id(2)) {
                                            Err(e) => {
                                                println!("Error when subscribing {e:?}");
                                            }
                                            Ok(..) => {

                                            }
                                        }

                                        let message = mqtt::MessageBuilder::new()
                                            .topic("$iothub/twin/GET/?$rid={135}")
                                            .finalize();
                                        match client.publish(message) {
                                            Err(e) => {
                                                println!("Fucked up {e:?}")
                                            }
                                            Ok(r) => {

                                            }
                                        }
                                    }
                                }

                                // Just loop on incoming messages.
                                // If we get a None message, check if we got disconnected,
                                // and then try a reconnect.
                                println!("\nWaiting for messages...");
                                for msg in rx.iter() {
                                    if let Some(msg) = msg {
                                        // In a real app you'd want to do a lot more error checking and
                                        // recovery, but this should give an idea about the basics.

                                        let sub_id = msg
                                            .properties()
                                            .get_int(mqtt::PropertyCode::SubscriptionIdentifier)
                                            .expect("No Subscription ID") as usize;

                                        if !handler[sub_id - 1](msg) {
                                            break;
                                        }
                                    }
                                    else if client.is_connected() || !AzureIoTHubMqtt::try_reconnect(&client) {
                                        break;
                                    }
                                }

                                // If we're still connected, then disconnect now,
                                // otherwise we're already disconnected.
                                if client.is_connected() {
                                    println!("\nDisconnecting");
                                    client.disconnect(None).unwrap();
                                }
                                println!("Exiting");
                            }
                        }
                    }
                }
            }
        }

        loop {
            println!("Azure iot hub mqtt does stuff!");
            thread::sleep(Duration::from_millis(2500));
        }    

        // Ok(true)
    }
}

#[async_trait]
impl HubClient for AzureIoTHubMqtt {
    fn is_connected(&self) -> anyhow::Result<bool> {
        Ok(false)
    }

    async fn start(&self) -> anyhow::Result<bool> {
        let auth_result = Auth::new(&self.connection_string);
        match &auth_result {
            Err(e) => {
                println!("Auth failed, will not try to connect: {e:?}");
                Err(Error::new(ErrorKind::InvalidData, "Error: {e:?}"))
            }
            Ok(a) => {
                let auth = a.clone();
                tokio::spawn(async move {
                    AzureIoTHubMqtt::connect(&auth);
                });
                Ok(true)
            }
        }
    }

    async fn disconnect(&self) -> anyhow::Result<bool> {
        Ok(true)
    }
}
