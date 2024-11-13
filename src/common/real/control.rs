mod current;
mod options;
mod voltage;
use crate::common::driver::KoradDriver;
use panduza_platform_core::{Device, Error};
use std::sync::Arc;
use tokio::sync::Mutex;

///
///
///
pub async fn mount(mut device: Device, driver: Arc<Mutex<KoradDriver>>) -> Result<(), Error> {
    //
    // Create attribute
    let itf_control = device.create_interface("control").finish();

    // mount_voltage(device.clone(), itf_control.clone()).await?;
    current::mount(device.clone(), itf_control.clone(), driver.clone()).await?;

    // mount_options(device.clone(), itf_control.clone()).await?;

    Ok(())
}
