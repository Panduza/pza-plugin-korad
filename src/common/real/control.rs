mod current;
mod options;
mod voltage;
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
    let itf_control = device.create_interface("control").finish();

    current::mount(device.clone(), itf_control.clone(), driver.clone()).await?;
    voltage::mount(device.clone(), itf_control.clone(), driver.clone()).await?;
    options::mount(device.clone(), itf_control.clone(), driver.clone()).await?;

    Ok(())
}
