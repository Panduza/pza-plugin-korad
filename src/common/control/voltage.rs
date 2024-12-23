use crate::common::driver::KoradDriver;
use panduza_platform_core::protocol::AsciiCmdRespProtocol;
use panduza_platform_core::{
    log_debug_mount_end, log_debug_mount_start, log_info, Container, Error, Logger, SiAttServer,
};
use panduza_platform_core::{spawn_on_command, Class, Instance};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::ControlSettings;

/// control/voltage
///
pub async fn mount<SD: AsciiCmdRespProtocol + 'static>(
    mut instance: Instance,
    mut class: Class,
    driver: Arc<Mutex<KoradDriver<SD>>>,
    control_settings: ControlSettings,
) -> Result<(), Error> {
    //
    // Create the attribute
    let att_server = class
        .create_attribute("voltage")
        .with_rw()
        .with_info(r#"Allow to read & write the voltage limit value of the power supply"#)
        .finish_as_si(
            "V",
            control_settings.min_voltage(),
            control_settings.max_voltage(),
            2,
        )
        .await?;
    let logger = att_server.logger();
    log_debug_mount_start!(logger);

    //
    // Init with a first value
    let mut v = driver.lock().await.get_vset().await?;
    if v < control_settings.min_voltage() as f32 {
        v = control_settings.min_voltage() as f32;
        driver.lock().await.set_vset(v).await?;
    }
    if v > control_settings.max_voltage() as f32 {
        v = control_settings.max_voltage() as f32;
        driver.lock().await.set_vset(v).await?;
    }
    att_server.set_from_f32(v).await?;

    //
    // Execute action on each command received
    let logger_2 = logger.clone();
    let att_server_2 = att_server.clone();
    spawn_on_command!(
        "on_command => control/voltage",
        instance,
        att_server_2,
        on_command(logger_2.clone(), att_server_2.clone(), driver.clone())
    );

    //
    // End of mount
    log_debug_mount_end!(logger);
    Ok(())
}

///
/// control/voltage => triggered when command is received
///
async fn on_command<SD: AsciiCmdRespProtocol>(
    logger: Logger,
    mut att_server: SiAttServer,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    while let Some(command_result) = att_server.pop_cmd_as_f32().await {
        match command_result {
            Ok(v) => {
                log_info!(logger, "command received '{:?}'", v);
                driver.lock().await.set_vset(v).await?;
                att_server.set_from_f32(v).await?;
                let real_value = driver.lock().await.get_vset().await?;
                att_server.set_from_f32(real_value).await?;
            }
            Err(e) => {
                let alert = format!("warning on received command '{:?}'", e);
                logger.warn(&alert);
                att_server.send_alert(alert).await;
                let real_value = driver.lock().await.get_vset().await?;
                att_server.set_from_f32(real_value).await?;
            }
        }
    }
    Ok(())
}
