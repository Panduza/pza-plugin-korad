use crate::common::driver::KoradDriver;
use panduza_platform_core::protocol::CommandResponseProtocol;
use panduza_platform_core::{
    spawn_on_command, BidirMsgAtt, BooleanCodec, Device, DeviceLogger, Interface,
};
use panduza_platform_core::{CmdOnlyMsgAtt, Error};
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
        .create_attribute("ovp")
        .message()
        .with_cmd_only_access()
        .finish_with_codec::<BooleanCodec>()
        .await;

    //
    // Execute action on each command received
    let logger_for_cmd_event = device.logger.clone();
    let att_voltage_for_cmd_event = att_voltage.clone();
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
    // Function ok
    Ok(())
}

///
///
///
async fn on_command<SD: CommandResponseProtocol>(
    logger: DeviceLogger,
    mut value_value_attr: CmdOnlyMsgAtt<BooleanCodec>,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    while let Some(command) = value_value_attr.pop_cmd().await {
        //
        // Log
        logger.debug(format!("SI ovp command received '{:?}'", command));
        driver.lock().await.set_ovp(command.value).await?;
    }
    Ok(())
}
