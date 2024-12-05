use crate::common::driver::KoradDriver;
use panduza_platform_core::protocol::AsciiCmdRespProtocol;
use panduza_platform_core::{log_info, Error, SiAttServer};
use panduza_platform_core::{spawn_on_command, Class, Instance, InstanceLogger};
use std::sync::Arc;
use tokio::sync::Mutex;

///
/// control/current
///
pub async fn mount<SD: AsciiCmdRespProtocol + 'static>(
    mut instance: Instance,
    mut class: Class,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    //
    // Start logging
    let logger = instance.logger.clone();
    logger.info("Mounting 'control/current' class...");

    //
    // Create the attribute
    let att_server = class
        .create_attribute("current")
        .with_rw()
        .with_info(r#"Allow to read & write the current limit value of the power supply"#)
        .finish_as_si("A", 0, 5, 3)
        .await?;

    //
    // Init with a first value
    let v = driver.lock().await.get_iset().await?;
    att_server.set_from_f32(v).await?;

    //
    // Execute action on each command received
    let logger_2 = logger.clone();
    let att_server_2 = att_server.clone();
    spawn_on_command!(
        "on_command => control/current",
        instance,
        att_server_2,
        on_command(logger_2.clone(), att_server_2.clone(), driver.clone())
    );

    //
    // End of mount
    logger.info("Mounting 'control/current' class -> OK");
    Ok(())
}

///
/// control/current => triggered when command is received
///
async fn on_command<SD: AsciiCmdRespProtocol>(
    logger: InstanceLogger,
    mut att_server: SiAttServer,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    while let Some(command_result) = att_server.pop_cmd_as_f32().await {
        match command_result {
            Ok(v) => {
                log_info!(logger, "'control/current' - command received '{:?}'", v);
                driver.lock().await.set_iset(v).await?;
                att_server.set_from_f32(v).await?;
                let real_value = driver.lock().await.get_iset().await?;
                att_server.set_from_f32(real_value).await?;
            }
            Err(e) => {
                let alert = format!("'control/current' - warning on received command '{:?}'", e);
                logger.warn(&alert);
                att_server.send_alert(alert).await;
                let real_value = driver.lock().await.get_iset().await?;
                att_server.set_from_f32(real_value).await?;
            }
        }
    }
    Ok(())
}
