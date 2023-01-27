mod services;
mod models;

use services::upcoming_stops::{
    UpcomingStops,
    config::Config
};
use services::upcoming_stops_mock::UpcomingStopsMock;

use realtime_communication::mqtt_client::{
    MqttClient, 
    subscription,
    subscription::{
        SubscriptionSender, 
        SubscriptionClient
    }
};
use std::thread;
use tokio::time::Duration;
use log::info;

#[tokio::main]
async fn main_() {
    logger::init_logger();
   
    let mut mqtt_client = MqttClient::new();


    let (subscription_a, subscription_client_a): (SubscriptionSender, SubscriptionClient<String>) = subscription::create_subscription::<String>(String::from("#"));
    mqtt_client.subscribe(subscription_a);

    let connection_status = mqtt_client.connect_to_localhost(String::from("passenger_information_service")).await;
    
    let (subscription_b, subscription_client_b): (SubscriptionSender, SubscriptionClient<String>) = subscription::create_subscription::<String>(String::from("test"));
    mqtt_client.subscribe(subscription_b);

    let config = Config::new(3030);
    let upcoming_stops = UpcomingStops::new(config, &mut mqtt_client);

    loop {
        thread::sleep(Duration::from_millis(1000));
        // mqtt_client.publish_msg_mqtt("test", "Hello Rust MQTT world!2").await;
        info!("Program doing stuff!");
    }
}

#[tokio::main]
async fn main() {
    logger::init_logger();

    let mut upcoming_stops_mock = UpcomingStopsMock::new();
    upcoming_stops_mock.start_mock_mqtt();
    upcoming_stops_mock.start_server();

    loop {
        thread::sleep(Duration::from_millis(1000));
        // mqtt_client.publish_msg_mqtt("test", "Hello Rust MQTT world!2").await;
        info!("Program doing stuff!");
    }
}

