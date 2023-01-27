extern crate anyhow;

pub mod hub_client;
pub mod configuration_observer;

use std::rc::Weak;
use hub_client::HubClient;
use configuration_observer::ConfigurationObserver;
use log::{error,warn,info};

//#[derive(Default)]
pub struct Configuration<T: HubClient>
    where T: HubClient {
    hub_client: T,
    observers: Vec<Weak<dyn ConfigurationObserver>>,
}

impl<T: HubClient> Configuration<T> {
    pub fn new(hub_client: T) -> Self {
        Configuration {
            hub_client,
            observers: Vec::new(),
        }
    }

    // pub async fn report(&self, _value: String) -> anyhow::Result<bool> {
    //     Ok(true)
    // }

    pub async fn get_configuration(&mut self, _path: Option<&String>) -> anyhow::Result<String> {
        self.init().await;

        return Ok("".to_string());
    }

    pub fn register_observer(&mut self, observer: Weak<dyn ConfigurationObserver>) {
        self.observers.push(observer);
    }

    fn notify_observers(&self, path: &str, data: &str) {
        // self.observers.retain(|observer| {
        //     if observer.strong_count() <= 0 {
        //         return false;
        //     }
        //     
        //     // TODO Check if observers path matches the datas path.
        //     // TODO if it does than merge the current known config with this patch?
        //     true
        // });
    }

    async fn init(&mut self) {
        info!("Configuration: Init");
        // let connected = self.hub_client.is_connected();

        // match connected {
        //     Ok(v) => {
        //         if !v {
                    // let config_changes_result = self.hub_client.config_changes()?;
                    // config_changes_result.
        info!("Configuration: Will connection to hub");
                    self.hub_client.connect().await;
        
        // let f = |data| {
        //     info!("Callback {data}");
        //     // Move obserrver to own struct?
        //     self.notify_observers("lol", "data");
        // };

        info!("test 1");
        // self.hub_client.set_config_callback(f);
        self.hub_client.set_config_callback(|data| {
            error!("Callback: {data}");
            // self.notify_observers("abc", "lol");
        }).await;

        info!("test 2");
        //         }
        //     }
        //     Err(e) => println!("Error getting is connected from hub client: {e:?}"),
        // }
    }

    // pub fn register_configuration_observer(&self, observer: ConfigurationObserver) {}
    // pub fn unregister_configuration_observer(&self, observer: ConfigurationObserver) {}
}
