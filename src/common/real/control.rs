mod current;
mod options;
mod voltage;
use crate::common::driver::KoradDriver;
use panduza_platform_core::{
    protocol::CommandResponseProtocol, spawn_on_command, BidirMsgAtt, BooleanCodec, Device,
    DeviceLogger, Error,
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
    // Create attribute
    let mut itf_control = device.create_interface("control").finish();

    current::mount(device.clone(), itf_control.clone(), driver.clone()).await?;
    voltage::mount(device.clone(), itf_control.clone(), driver.clone()).await?;
    options::mount(device.clone(), itf_control.clone(), driver.clone()).await?;

    //
    //
    let att_oe = itf_control
        .create_attribute("output_enable")
        .message()
        .with_bidir_access()
        .finish_with_codec::<BooleanCodec>()
        .await;

    let v = driver.lock().await.get_out().await?;
    att_oe.set(v).await.unwrap();

    //
    // Execute action on each command received
    let logger_for_cmd_event = device.logger.clone();
    let att_oe_for_cmd_event = att_oe.clone();
    spawn_on_command!(
        device,
        att_oe_for_cmd_event,
        on_command(
            logger_for_cmd_event.clone(),
            att_oe_for_cmd_event.clone(),
            driver.clone()
        )
    );
    Ok(())
}

///
///
///
async fn on_command<SD: CommandResponseProtocol>(
    logger: DeviceLogger,
    mut att_oe: BidirMsgAtt<BooleanCodec>,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    while let Some(command) = att_oe.pop_cmd().await {
        //
        // Log
        logger.debug(format!("SI voltage command received '{:?}'", command));

        driver.lock().await.set_out(command.value).await?;

        let v = driver.lock().await.get_out().await?;
        att_oe.set(v).await?;
    }
    Ok(())
}
