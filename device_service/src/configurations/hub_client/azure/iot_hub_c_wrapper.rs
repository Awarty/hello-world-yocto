// This may be replaced by a C-wrapper using azure-iot-sdk-c.

pub struct AzureIoTHubCWrapper {
    connection_string: String,

}

impl AzureIoTHubCWrapper {
    pub fn new(connection_string: String) -> Self {
        AzureIoTHub {
            connection_string,
        }
    }
}

impl HubClient for AzureIoTHubCWrapper {
    fn is_connected(&self) -> anyhow::Result<bool> {
        Ok(false)
    }

    //https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/iot_hub/examples/updatetwin.rs
    async fn connect(&self) -> anyhow::Result<bool> {
        Ok(false)
    }

    fn disconnect(&self) -> anyhow::Result<bool> {
        Ok(false)
    }
}