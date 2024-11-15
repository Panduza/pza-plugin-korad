use panduza_platform_core::{
    spawn_on_command, BidirMsgAtt, Device, DeviceLogger, Interface, SiCodec, SiSettings,
};
use panduza_platform_core::{AttOnlyMsgAtt, Error};
use std::sync::Arc;

use panduza_platform_core::protocol::CommandResponseProtocol;
use panduza_platform_core::BooleanCodec;
use tokio::sync::Mutex;

use crate::common::driver::KoradDriver;

///
///
///
pub async fn mount<SD: CommandResponseProtocol + 'static>(
    mut device: Device,
    mut interface: Interface,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    let mut c_interface = interface.create_interface("current").finish();

    let settings = SiSettings::new("A", 0, 3, 3);

    //
    //
    let att_current = c_interface
        .create_attribute("value")
        .with_settings(settings.into())
        .message()
        .with_att_only_access()
        .finish_with_codec::<SiCodec>()
        .await;

    att_current.set(5).await.unwrap();

    //
    //
    let att_trigger = c_interface
        .create_attribute("trigger")
        .message()
        .with_bidir_access()
        .finish_with_codec::<BooleanCodec>()
        .await;

    //
    // Execute action on each command received
    let logger_for_cmd_event = device.logger.clone();
    let att_trigger_for_cmd_event = att_trigger.clone();
    spawn_on_command!(
        device,
        att_trigger_for_cmd_event,
        on_command(
            logger_for_cmd_event.clone(),
            att_trigger_for_cmd_event.clone(),
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
    mut value_value_attr: BidirMsgAtt<BooleanCodec>,
    value_si_attr: AttOnlyMsgAtt<SiCodec>,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    while let Some(command) = value_value_attr.pop_cmd().await {
        //
        // Log
        logger.debug(format!("Trigger current received '{:?}'", command));

        let trigger = command.value;

        let v = driver.lock().await.get_iout().await?;

        value_value_attr.set(trigger).await?;

        value_si_attr.set(SiCodec::from_f32(v, 3)).await?;
    }
    Ok(())
}
