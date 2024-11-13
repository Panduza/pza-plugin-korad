// mod current;
// mod options;
// mod voltage;

use panduza_platform_core::{Device, Error};

// use current::mount_current;
// use voltage::mount_voltage;

///
///
///
pub async fn mount_measure(mut device: Device) -> Result<(), Error> {
    //
    // Create attribute
    let itf_measure = device.create_interface("measure").finish();

    // mount_voltage(device.clone(), itf_control.clone()).await?;
    // mount_current(device.clone(), itf_control.clone()).await?;

    Ok(())
}
