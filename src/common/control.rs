pub use super::ControlSettings;

mod current;
mod options;
mod voltage;
use crate::common::driver::KoradDriver;
use panduza_platform_core::{
    log_debug_mount_end, log_debug_mount_start, protocol::AsciiCmdRespProtocol, spawn_on_command,
    BooleanAttServer, Container, Error, Instance, Logger,
};
use std::sync::Arc;
use tokio::sync::Mutex;

///
///
///
pub async fn mount<SD: AsciiCmdRespProtocol + 'static>(
    mut instance: Instance,
    driver: Arc<Mutex<KoradDriver<SD>>>,
    control_settings: ControlSettings,
) -> Result<(), Error> {
    //
    // Create attribute
    let mut itf_control = instance.create_class("control").finish().await;
    let logger = itf_control.logger().clone();
    log_debug_mount_start!(logger);

    current::mount(
        instance.clone(),
        itf_control.clone(),
        driver.clone(),
        control_settings.clone(),
    )
    .await?;
    voltage::mount(
        instance.clone(),
        itf_control.clone(),
        driver.clone(),
        control_settings.clone(),
    )
    .await?;
    options::mount(instance.clone(), itf_control.clone(), driver.clone()).await?;

    //
    //
    let att_oe = itf_control
        .create_attribute("output_enable")
        .with_rw()
        .finish_as_boolean()
        .await?;

    let v = driver.lock().await.get_out().await?;
    att_oe.set(v).await?;

    //
    // Execute action on each command received
    let logger_2 = instance.logger.clone();
    let att_oe_2 = att_oe.clone();
    spawn_on_command!(
        "on_command => control",
        instance,
        att_oe_2,
        on_command(logger_2.clone(), att_oe_2.clone(), driver.clone())
    );

    //
    // End of mount
    log_debug_mount_end!(logger);
    Ok(())
}

///
///
///
async fn on_command<SD: AsciiCmdRespProtocol>(
    logger: Logger,
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
