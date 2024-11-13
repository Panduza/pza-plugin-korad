use crate::common::driver::KoradDriver;
use panduza_platform_core::{Device, Error, StringCodec};
use std::sync::Arc;
use tokio::sync::Mutex;

///
///
///
pub async fn mount(mut device: Device, driver: Arc<Mutex<KoradDriver>>) -> Result<(), Error> {
    //
    // Create attribute
    let att_identity = device
        .create_attribute("identity")
        .message()
        .with_att_only_access()
        .finish_with_codec::<StringCodec>()
        .await;

    let idn = driver.lock().await.get_idn().await?;

    att_identity.set(idn).await?;

    Ok(())
}
