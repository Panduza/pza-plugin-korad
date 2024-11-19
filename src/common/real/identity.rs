use crate::common::driver::KoradDriver;
use panduza_platform_core::protocol::CommandResponseProtocol;
use panduza_platform_core::{Device, Error};
use std::sync::Arc;
use tokio::sync::Mutex;

///
///
///
pub async fn mount<SD: CommandResponseProtocol>(
    mut device: Device,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    //
    // Create attribute
    let att_identity = device
        .create_attribute("identity")
        .with_ro()
        .with_info("Identity string of the power supply")
        .finish_as_string()
        .await?;

    //
    // Just init
    let idn = driver.lock().await.get_idn().await?;
    att_identity.set(idn).await?;

    Ok(())
}
