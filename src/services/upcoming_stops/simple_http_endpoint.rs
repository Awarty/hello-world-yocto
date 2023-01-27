mod filters;
mod handlers;

use log::{info};
use super::data_handler::DataHandler;
use std::sync::{Arc, Mutex};

pub struct SimpleHttpEndpoint{
    join_handler: Option<tokio::task::JoinHandle<()>>
}

impl SimpleHttpEndpoint {
    pub fn new() -> Self {
        SimpleHttpEndpoint{
            join_handler: None
        }
    }

    pub fn start_server(&mut self, port: u16, data_handler: &Arc<Mutex<DataHandler>>) -> Option<&tokio::task::JoinHandle<()>> {

        let handler = data_handler.clone();
        let join_handler = tokio::spawn(async move {
            info!("Started HTTP server.");
            let gpt_server_endpoints = filters::endpoints(handler);
            warp::serve(gpt_server_endpoints)
                .run(([127, 0, 0, 1], port))
            .await;
        });

        self.join_handler = Some(join_handler);

        match &self.join_handler {
            Some(handler) => Some(&handler),
            None => None
        }
    }

    pub fn stop_server(&self) -> bool {
        if let Some(handler) = &self.join_handler {
            handler.abort();
            true
        } else {
            false
        }
    }
}
