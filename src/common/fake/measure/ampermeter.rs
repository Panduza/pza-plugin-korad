use panduza_platform_core::Error;
use panduza_platform_core::{
    spawn_on_command, BidirMsgAtt, Device, DeviceLogger, Interface, SiCodec, SiSettings,
};

///
///
///
pub async fn mount_ampermeter(mut device: Device, mut interface: Interface) -> Result<(), Error> {
    let settings = SiSettings::new("A", 0, 30, 2);

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
