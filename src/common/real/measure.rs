mod ampermeter;
mod voltmeter;

use crate::common::driver::KoradDriver;
use panduza_platform_core::{protocol::CommandResponseProtocol, Device, Error};
use std::sync::Arc;
use tokio::sync::Mutex;

///
///
///
pub async fn mount<SD: CommandResponseProtocol + 'static>(
    mut device: Device,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    //
    // Create attribute
    let itf_measure = device.create_interface("measure").finish();

    ampermeter::mount(device.clone(), itf_measure.clone(), driver.clone()).await?;
    voltmeter::mount(device.clone(), itf_measure.clone(), driver.clone()).await?;

    Ok(())
}
