extern crate anyhow;

use async_trait::async_trait;

pub mod azure;

#[async_trait]
pub trait HubClient {
    fn is_connected(&self) -> anyhow::Result<bool>;
    async fn connect(&mut self) -> anyhow::Result<bool>;
    async fn disconnect(&self) -> anyhow::Result<bool>;
    async fn set_config_callback(&self, callback: impl FnMut(String) + Send);
    fn report(&self, path: String, payload: String);
}
