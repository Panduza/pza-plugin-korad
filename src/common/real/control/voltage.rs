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
    let att_voltage = interface
        .create_attribute("voltage")
        .with_settings(settings.into())
        .message()
        .with_bidir_access()
        .finish_with_codec::<SiCodec>()
        .await;

    let v = driver.lock().await.get_iset().await?;
    att_voltage.set(SiCodec::from_f32(v, 2)).await.unwrap();

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
async fn on_command(
    logger: DeviceLogger,
    mut value_value_attr: BidirMsgAtt<SiCodec>,
    driver: Arc<Mutex<KoradDriver>>,
) -> Result<(), Error> {
    while let Some(command) = value_value_attr.pop_cmd().await {
        //
        // Log
        logger.debug(format!("SI voltage command received '{:?}'", command));

        let v = command.into_f32()?;
        driver.lock().await.set_iset(v).await?;

        value_value_attr.set(command).await?;
    }
    Ok(())
}
