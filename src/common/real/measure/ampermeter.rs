use crate::common::driver::KoradDriver;
use panduza_platform_core::protocol::CommandResponseProtocol;
use panduza_platform_core::Error;
use panduza_platform_core::{log_info, spawn_on_command, Device, DeviceLogger, Interface};
use panduza_platform_core::{BooleanAttServer, SiAttServer};
use std::sync::Arc;
use tokio::sync::Mutex;

///
///
///
pub async fn mount<SD: CommandResponseProtocol + 'static>(
    mut device: Device,
    mut interface: Interface,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    //
    // Create interface
    let mut c_interface = interface.create_interface("current").finish();

    //
    //
    let att_current = c_interface
        .create_attribute("value")
        .with_ro()
        .with_info(r#"Hold values returned from the ampermeter"#)
        .finish_as_si("A", 0, 3, 3)
        .await?;

    //
    // Init with a first value
    att_current.set_from_f32(0.0).await.unwrap();

    //
    //
    let att_trigger = c_interface
        .create_attribute("trigger")
        .with_rw()
        .with_info(
            r#"Trigger a measure in the ampermeter, the data is published on the 'value' topic"#,
        )
        .finish_as_boolean()
        .await?;

    //
    // Execute action on each command received
    let logger_2 = device.logger.clone();
    let att_trigger_2 = att_trigger.clone();
    spawn_on_command!(
        device,
        att_trigger_2,
        on_command(
            logger_2.clone(),
            att_trigger_2.clone(),
            att_current.clone(),
            driver.clone()
        )
    );

    //
    // Function ok
    Ok(())
}

///
///
///
async fn on_command<SD: CommandResponseProtocol>(
    logger: DeviceLogger,
    mut att_trigger: BooleanAttServer,
    att_current: SiAttServer,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    while let Some(command) = att_trigger.pop_cmd().await {
        //
        // Log
        log_info!(
            logger,
            "'measure/current/trigger' - command received '{:?}'",
            command
        );

        if command == true {
            let v = driver.lock().await.get_iout().await?;
            att_current.set_from_f32(v).await?;
            att_trigger.set(true).await?;
        }
    }
    Ok(())
}
