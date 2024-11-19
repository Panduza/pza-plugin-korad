use crate::common::driver::KoradDriver;
use panduza_platform_core::protocol::CommandResponseProtocol;
use panduza_platform_core::Error;
use panduza_platform_core::{spawn_on_command, BooleanAttServer, Device, DeviceLogger, Interface};
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
    //
    let att_voltage = interface
        .create_attribute("ocp")
        .with_wo()
        .with_info(
            r#"Enable/Disable the Over Current Protection.
        'Write Only' because the power supply does not given any read function on it"#,
        )
        .finish_as_boolean()
        .await?;

    //
    // Execute action on each command received
    let logger_2 = device.logger.clone();
    let att_voltage_2 = att_voltage.clone();
    spawn_on_command!(
        device,
        att_voltage_2,
        on_command(logger_2.clone(), att_voltage_2.clone(), driver.clone())
    );

    //
    // Function ok
    Ok(())
}

///
///
///
async fn on_command<SD: CommandResponseProtocol + 'static>(
    logger: DeviceLogger,
    mut value_value_attr: BooleanAttServer,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    while let Some(command) = value_value_attr.pop_cmd().await {
        //
        // Log
        logger.debug(format!("OCP command received '{:?}'", command));
        driver.lock().await.set_ocp(command).await?;
    }
    Ok(())
}
