use crate::common::driver::KoradDriver;
use panduza_platform_core::protocol::CommandResponseProtocol;
use panduza_platform_core::{
    spawn_on_command, BidirMsgAtt, Device, DeviceLogger, Interface, SiCodec, SiSettings,
};
use panduza_platform_core::{Error, SiAttServer};
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
    // let settings = SiSettings::new("V", 0, 30, 2);

    let control_voltage = interface
        .create_attribute("voltage")
        .with_rw()
        .finish_as_si("V", 0, 30, 2)
        .await?;

    let v = driver.lock().await.get_vset().await?;
    control_voltage.set_from_f32(v).await.unwrap();

    // //
    // //
    // let att_voltage = interface
    //     .create_attribute("voltage")
    //     .with_settings(settings.into())
    //     .message()
    //     .with_bidir_access()
    //     .finish_with_codec::<SiCodec>()
    //     .await;

    // let v = driver.lock().await.get_vset().await?;
    // att_voltage.set(SiCodec::from_f32(v, 2)).await.unwrap();

    //
    // la gestion des codec est penible
    // les settings à part c'est aussi bof
    //

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
    // Function ok
    Ok(())
}

///
///
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
