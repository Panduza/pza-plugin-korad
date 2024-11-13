mod beep;
mod lock;
mod mode;
mod ocp;
mod ovp;

use panduza_platform_core::{Device, Error, Interface};

///
///
///
pub async fn mount_options(mut device: Device, mut interface: Interface) -> Result<(), Error> {
    //
    // Create attribute
    let itf_options = interface.create_interface("options").finish();

    ovp::mount(device.clone(), itf_options.clone()).await?;
    ocp::mount(device.clone(), itf_options.clone()).await?;
    beep::mount(device.clone(), itf_options.clone()).await?;
    lock::mount(device.clone(), itf_options.clone()).await?;

    Ok(())
}
