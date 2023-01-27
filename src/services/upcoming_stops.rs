pub mod config;
pub mod parse_itxpt;

pub mod data_handler;
pub mod simple_http_endpoint;
use log::{info, error};

use config::Config;
use realtime_communication::mqtt_client::{
    MqttClient,
    subscription,
    // subscription::SubscriptionSender,
    // subscription::SubscriptionClient,
};
use itxpt::{
    vehicle_journey_details::VehicleJourneyDetails,
};
use simple_http_endpoint::SimpleHttpEndpoint;
use data_handler::DataHandler;
use std::sync::{Arc, Mutex};

pub struct UpcomingStops<'a> {
    config: Config,
    mqtt_client: &'a mut MqttClient,
    endpoint: SimpleHttpEndpoint,
    data_handler: Arc<Mutex<DataHandler>>,
    thread_handler: Option<tokio::task::JoinHandle<()>>,
}

impl<'a> UpcomingStops<'a> {
    pub fn new(config: Config, mqtt_client: &'a mut MqttClient) -> UpcomingStops<'a> {
        let data_handler = Arc::new(Mutex::new(DataHandler::new()));

        let endpoint = SimpleHttpEndpoint::new();

        UpcomingStops {
            config,
            mqtt_client,
            endpoint,
            data_handler,
            thread_handler: None,
        }
    }

    pub fn start(&mut self) {
        let handler = self.data_handler.clone();

        let (subscription, mut subscription_client) = subscription::create_subscription::<VehicleJourneyDetails>(String::from("#"));
        self.mqtt_client.subscribe(subscription);

        self.thread_handler = Some(tokio::spawn(async move {
            while let Some(vehicle_journey_details)  = subscription_client.receive().await {
                UpcomingStops::handle_details(vehicle_journey_details, handler.clone());
            }
        }));
    }

    pub fn stop(&self) {
        match &self.thread_handler {
            None => info!("Cannot stop UpcomingStops service, not started."),
            Some(thread_handler) => {
                thread_handler.abort();
            }
        }

        // self.mqtt_client.unsubscribe(id);
    }

    pub fn handle_details(vehicle_journey_details: VehicleJourneyDetails, data_handler: Arc<Mutex<DataHandler>>) {
        let handler_lock = data_handler.lock();
        match handler_lock {
            Ok(mut handler) => {
                handler.set_vehicle_journey_details(vehicle_journey_details);
            }
            Err(e) => {
                error!("Failed to lock data handler {e:?}");
            }
        }
    }

    async fn start_endpoint(&mut self) {
        self.endpoint.start_server(8080, &self.data_handler);
    }
}
