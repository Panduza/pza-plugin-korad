mod current;
mod options;
mod voltage;

use panduza_platform_core::{Device, Error};

use current::mount_current;
use voltage::mount_voltage;

///
///
///
pub async fn mount_control(mut device: Device) -> Result<(), Error> {
    //
    // Create attribute
    let itf_control = device.create_interface("control").finish();

    mount_voltage(device.clone(), itf_control.clone()).await?;
    mount_current(device.clone(), itf_control.clone()).await?;

    Ok(())
}
