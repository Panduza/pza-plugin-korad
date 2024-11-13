use panduza_platform_core::Error;
use panduza_platform_core::{
    spawn_on_command, BidirMsgAtt, Device, DeviceLogger, EnumCodec, EnumSettings, Interface,
};

///
///
///
pub async fn mount(mut device: Device, mut interface: Interface) -> Result<(), Error> {
    let s = EnumSettings::from(vec!["C.C", "C.V"]);

    //
    //
    let att_voltage = interface
        .create_attribute("mode")
        .message()
        .with_bidir_access()
        .finish_with_codec::<EnumCodec>()
        .await;

    att_voltage.set("C.C").await.unwrap();

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
    mut value_value_attr: BidirMsgAtt<EnumCodec>,
) -> Result<(), Error> {
    while let Some(command) = value_value_attr.pop_cmd().await {
        //
        // Log
        logger.debug(format!("SI ovp command received '{:?}'", command));

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