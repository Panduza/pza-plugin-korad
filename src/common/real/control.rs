mod current;
mod options;
mod voltage;
use crate::common::driver::KoradDriver;
use panduza_platform_core::{
    protocol::CommandResponseProtocol, spawn_on_command, BooleanAttServer, Device, DeviceLogger,
    Error,
};
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
    // Start logging
    let logger = device.logger.clone();
    logger.info("Mounting 'control' class...");

    //
    // Create attribute
    let mut itf_control = device.create_interface("control").finish();

    current::mount(device.clone(), itf_control.clone(), driver.clone()).await?;
    voltage::mount(device.clone(), itf_control.clone(), driver.clone()).await?;
    options::mount(device.clone(), itf_control.clone(), driver.clone()).await?;

    //
    //
    let att_oe = itf_control
        .create_attribute("output_enable")
        .with_rw()
        .finish_as_boolean()
        .await?;

    let v = driver.lock().await.get_out().await?;
    att_oe.set(v).await.unwrap();

    //
    // Execute action on each command received
    let logger_2 = device.logger.clone();
    let att_oe_2 = att_oe.clone();
    spawn_on_command!(
        device,
        att_oe_2,
        on_command(logger_2.clone(), att_oe_2.clone(), driver.clone())
    );

    //
    // End of mount
    logger.info("Mounting 'control' class -> OK");
    Ok(())
}

///
///
///
async fn on_command<SD: CommandResponseProtocol>(
    logger: DeviceLogger,
    mut att_oe: BooleanAttServer,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    while let Some(command) = att_oe.pop_cmd().await {
        //
        // Log
        logger.debug(format!("SI voltage command received '{:?}'", command));

        driver.lock().await.set_out(command).await?;

        let v = driver.lock().await.get_out().await?;
        att_oe.set(v).await?;
    }
    Ok(())
}
