mod beep;
mod lock;
mod mode;
mod ocp;
mod ovp;
use crate::common::driver::KoradDriver;
use panduza_platform_core::{Device, Error, Interface};
use std::sync::Arc;
use tokio::sync::Mutex;

///
///
///
pub async fn mount_options(
    mut device: Device,
    mut interface: Interface,
    driver: Arc<Mutex<KoradDriver>>,
) -> Result<(), Error> {
    //
    // Create attribute
    let itf_options = interface.create_interface("options").finish();

    ovp::mount(device.clone(), itf_options.clone(), driver.clone()).await?;
    ocp::mount(device.clone(), itf_options.clone(), driver.clone()).await?;
    beep::mount(device.clone(), itf_options.clone(), driver.clone()).await?;
    lock::mount(device.clone(), itf_options.clone(), driver.clone()).await?;
    mode::mount(device.clone(), itf_options.clone(), driver.clone()).await?;

    Ok(())
}
