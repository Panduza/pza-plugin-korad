mod ampermeter;
mod voltmeter;

use crate::common::driver::KoradDriver;
use ampermeter::mount_ampermeter;
use panduza_platform_core::{Device, Error};
use std::sync::Arc;
use tokio::sync::Mutex;
use voltmeter::mount_voltmeter;

///
///
///
pub async fn mount_measure(mut device: Device) -> Result<(), Error> {
    //
    // Create attribute
    let itf_measure = device.create_interface("measure").finish();

    mount_ampermeter(device.clone(), itf_measure.clone()).await?;
    mount_voltmeter(device.clone(), itf_measure.clone()).await?;

    Ok(())
}
