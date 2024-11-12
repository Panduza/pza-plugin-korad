use panduza_platform_core::{Device, StringCodec};

///
///
///
pub async fn mount_identity(mut device: Device) {
    //
    // Create attribute
    let att_identity = device
        .create_attribute("identity")
        .message()
        .with_att_only_access()
        .finish_with_codec::<StringCodec>()
        .await;
}

///
///
///
pub async fn mount_identity_fake<N: Into<String>>(mut device: Device, fake_value: N) {
    //
    // Create attribute
    let att_identity = device
        .create_attribute("identity")
        .message()
        .with_att_only_access()
        .finish_with_codec::<StringCodec>()
        .await;

    att_identity.set(fake_value.into()).await.unwrap();
}
