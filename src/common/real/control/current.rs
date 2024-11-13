use crate::common::driver::KoradDriver;
use panduza_platform_core::Error;
use panduza_platform_core::{
    spawn_on_command, BidirMsgAtt, Device, DeviceLogger, Interface, SiCodec, SiSettings,
};
use std::sync::Arc;
use tokio::sync::Mutex;

///
///
///
pub async fn mount(
    mut device: Device,
    mut interface: Interface,
    driver: Arc<Mutex<KoradDriver>>,
) -> Result<(), Error> {
    let settings = SiSettings::new("V", 0, 30);

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
    att_current.set(v).await.unwrap();

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
async fn on_command(
    logger: DeviceLogger,
    mut value_value_attr: BidirMsgAtt<SiCodec>,
    driver: Arc<Mutex<KoradDriver>>,
) -> Result<(), Error> {
    while let Some(command) = value_value_attr.pop_cmd().await {
        //
        // Log
        logger.debug(format!("SI current command received '{:?}'", command));

        // driver.lock().await.set_iset(command.into());

        value_value_attr.set(command).await?;

        // if command.value == "0".to_string() {
        //     // connector.pico_set_bus_id(0).await?;

        // } else if command.value == "1".to_string() {
        //     // connector.pico_set_bus_id(1).await?;
        //     value_value_attr.set("1".to_string()).await?;
        // } else {
        //     logger.error(format!("BAD BUS ID !!!! {:?}", command.value));
        // }
    }
    Ok(())
}
