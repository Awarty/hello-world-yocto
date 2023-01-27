mod configurations;
mod dps;

use configurations::Configuration;
use configurations::hub_client::azure::azure_iot_hub::AzureIoTHub;

use crate::dps::register;

use tokio;
use tokio::time::Duration;

use std::thread;

use log::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init_logger();

    // let connection_string = "HostName=syscomiotedgehubdev.azure-devices.net;DeviceId=JAT-device;SharedAccessKey=ejya76FGU7VS8CLgSzW9tuMHjQ1KJISgnnB/HCgOUI0=\\0".to_string();
    let connection_string1 = "HostName=syscomiotedgehubdev.azure-devices.net;DeviceId=tmm-2022-12-04;SharedAccessKey=lU4/4Z542gx+sx540AHD0JUS8nlGOmfn0lnTOETebqA=".to_string();

    let hub_client = AzureIoTHub::new(connection_string1)?;

    let mut config: Configuration<AzureIoTHub> = Configuration::new(hub_client);
    let config_key = "hej".to_string();
    let hej_config = config.get_configuration(Some(&config_key)).await;

    info!("{:?}-config says: {:?}", config_key, hej_config);

    let mac_address = mac_address::get_mac_address()
        .unwrap()
        .unwrap()
        .to_string()
        .replace(":", "-");
    let architecture = std::env::consts::ARCH;

    let register_result = register(&mac_address.as_str(), &architecture, "a1b2c3").await;
    match register_result {
        Ok(v) => {

        },
        Err(e) => {
            error!("Register error {e:?}");
        }
    }

    let mut count = 0u32;

    info!("Main: Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        thread::sleep(Duration::from_millis(1000));
        info!("Main: {}", count);
    }

    Ok(())
}
