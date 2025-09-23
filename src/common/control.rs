pub use super::ControlSettings;

// mod current;
// mod options;
mod voltage;
use crate::common::driver::KoradDriver;
use futures::FutureExt;
use panduza_platform_core::{
    log_debug_mount_end, log_debug_mount_start, protocol::BytesDialogProtocol, Container, Error,
    Instance, Logger,
};
use std::sync::Arc;
use tokio::sync::Mutex;

///
///
///
pub async fn mount<SD: BytesDialogProtocol + 'static>(
    mut instance: Instance,
    driver: Arc<Mutex<KoradDriver<SD>>>,
    control_settings: ControlSettings,
) -> Result<(), Error> {
    //
    // Create attribute
    let mut itf_control = instance.create_class("control").finish().await;
    let logger = itf_control.logger().clone();
    log_debug_mount_start!(logger);

    // current::mount(
    //     instance.clone(),
    //     itf_control.clone(),
    //     driver.clone(),
    //     control_settings.clone(),
    // )
    // .await?;
    voltage::mount(
        instance.clone(),
        &mut itf_control.clone(),
        driver.clone(),
        control_settings.clone(),
    )
    .await?;
    // options::mount(instance.clone(), itf_control.clone(), driver.clone()).await?;

    //
    //
    let att_oe = itf_control
        .create_attribute("output_enable")
        .with_rw()
        .start_as_boolean()
        .await?;

    let v = driver.lock().await.get_out().await?;
    att_oe.set(v).await?;

    //
    // Execute action on each command received
    let logger_2 = instance.logger().clone();
    let att_oe_2 = att_oe.clone();
    att_oe.add_callback({
        move |command| {
            let att_boolean_alert = att_oe_2.clone();
            async move {
                // log_info!(att_boolean_alert.logger(), "Alert simulation triggered");
                att_boolean_alert
                    .trigger_alert("Simulated alert triggered for testing purposes")
                    .await;
            }
            .boxed()
        }
    });

    //
    // End of mount
    log_debug_mount_end!(logger);
    Ok(())
}
