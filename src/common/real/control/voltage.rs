use crate::common::driver::KoradDriver;
use panduza_platform_core::protocol::CommandResponseProtocol;
use panduza_platform_core::{spawn_on_command, Device, DeviceLogger, Interface};
use panduza_platform_core::{Error, SiAttServer};
use std::sync::Arc;
use tokio::sync::Mutex;

///
/// control/voltage
///
pub async fn mount<SD: CommandResponseProtocol + 'static>(
    mut device: Device,
    mut interface: Interface,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    //
    // Start logging
    let logger = device.logger.clone();
    logger.info("Mounting 'control/voltage' class...");

    //
    // Create the attribute
    let control_voltage = interface
        .create_attribute("voltage")
        .with_rw()
        .finish_as_si("V", 0, 30, 2)
        .await?;

    //
    // Init with a first value
    let v = driver.lock().await.get_vset().await?;
    control_voltage.set_from_f32(v).await?;

    //
    // Execute action on each command received
    let logger_for_cmd_event = device.logger.clone();
    let att_voltage_for_cmd_event = control_voltage.clone();
    spawn_on_command!(
        device,
        att_voltage_for_cmd_event,
        on_command(
            logger_for_cmd_event.clone(),
            att_voltage_for_cmd_event.clone(),
            driver.clone()
        )
    );

    //
    // End of mount
    logger.info("Mounting 'control/voltage' class -> OK");
    Ok(())
}

///
/// control/voltage => triggered when command is received
///
async fn on_command<SD: CommandResponseProtocol>(
    logger: DeviceLogger,
    mut att_server: SiAttServer,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    while let Some(command_result) = att_server.pop_cmd_as_f32().await {
        match command_result {
            Ok(v) => {
                logger.debug(format!("SI voltage command received '{:?}'", v));
                driver.lock().await.set_vset(v).await?;
                att_server.set_from_f32(v).await?;
            }
            Err(e) => {
                logger.error(format!("ERRRR '{:?}'", e));
            }
        }
    }
    Ok(())
}
