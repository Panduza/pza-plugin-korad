use crate::common::driver::KoradDriver;
use panduza_platform_core::protocol::CommandResponseProtocol;
use panduza_platform_core::Error;
use panduza_platform_core::{
    spawn_on_command, BidirMsgAtt, Device, DeviceLogger, Interface, SiCodec, SiSettings,
};
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
    let settings = SiSettings::new("A", 0, 5, 3);

    //
    //
    let att_current = interface
        .create_attribute("current")
        .with_settings(settings.into())
        .message()
        .with_bidir_access()
        .finish_with_codec::<SiCodec>()
        .await;

    let v = driver.lock().await.get_iset().await?;
    att_current.set(SiCodec::from_f32(v, 3)).await.unwrap();

    //
    // Execute action on each command received
    let logger_for_cmd_event = device.logger.clone();
    let att_current_for_cmd_event = att_current.clone();
    spawn_on_command!(
        device,
        att_current_for_cmd_event,
        on_command(
            logger_for_cmd_event.clone(),
            att_current_for_cmd_event.clone(),
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
async fn on_command<SD: CommandResponseProtocol + Send>(
    logger: DeviceLogger,
    mut value_value_attr: BidirMsgAtt<SiCodec>,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    while let Some(command) = value_value_attr.pop_cmd().await {
        //
        // Log
        logger.debug(format!("SI current command received '{:?}'", command));

        let v = command.into_f32()?;
        driver.lock().await.set_iset(v).await?;

        value_value_attr.set(command).await?;
    }
    Ok(())
}
