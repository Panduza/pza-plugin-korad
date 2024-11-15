use panduza_platform_core::Error;
use panduza_platform_core::{
    spawn_on_command, BidirMsgAtt, Device, DeviceLogger, Interface, SiCodec, SiSettings,
};
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
    let settings = SiSettings::new("A", 0, 3, 3);

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
