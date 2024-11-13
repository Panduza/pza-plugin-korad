use panduza_platform_core::Error;
use panduza_platform_core::{
    spawn_on_command, BidirMsgAtt, Device, DeviceLogger, Interface, SiCodec, SiSettings,
};

///
///
///
pub async fn mount_voltmeter(mut device: Device, mut interface: Interface) -> Result<(), Error> {
    let settings = SiSettings::new("V", 0, 30);

    //
    //
    let att_current = interface
        .create_attribute("ampermeter")
        .with_settings(settings.into())
        .message()
        .with_att_only_access()
        .finish_with_codec::<SiCodec>()
        .await;

    att_current.set(5).await.unwrap();

    //
    // Function ok
    Ok(())
}
