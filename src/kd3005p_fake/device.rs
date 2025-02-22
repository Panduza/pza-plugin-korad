use crate::{
    common::{driver::KoradDriver, fake::Driver as SerialFakeDriver},
    ControlSettings,
};
use async_trait::async_trait;
use panduza_platform_core::{log_info, DriverOperations, Error, Instance};
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::sleep};

///
/// Device to control PicoHA Dio Board
///
pub struct KD3005PFakeDevice {}

impl KD3005PFakeDevice {
    ///
    /// Constructor
    ///
    pub fn new() -> Self {
        KD3005PFakeDevice {}
    }

    ///
    /// Try to mount the connector to reach the device
    ///
    pub fn mount_driver(
        &mut self,
        instance: Instance,
    ) -> Result<Arc<Mutex<KoradDriver<SerialFakeDriver>>>, Error> {
        let driver = SerialFakeDriver::open()?;

        let kdriver = KoradDriver::new(driver, instance.logger.clone());

        Ok(Arc::new(Mutex::new(kdriver)))
    }

    ///
    ///
    pub async fn prepare_control_settings(
        &mut self,
        instance: Instance,
    ) -> Result<ControlSettings, Error> {
        //
        //
        let instance_settings = instance.settings().await;

        //
        //
        let mut control_settings = ControlSettings::new();
        control_settings.override_with_instance_settings(&instance_settings)?;

        //
        //
        Ok(control_settings)
    }
}

#[async_trait]
impl DriverOperations for KD3005PFakeDevice {
    ///
    ///
    ///
    async fn mount(&mut self, instance: Instance) -> Result<(), Error> {
        let logger = instance.logger.clone();

        let driver = self.mount_driver(instance.clone())?;

        //
        //
        let control_settings = self.prepare_control_settings(instance.clone()).await?;
        log_info!(logger, "control_settings = {:?}", control_settings);

        //
        // Identity
        panduza_platform_core::std::attribute::idn::mount(instance.clone(), driver.clone()).await?;

        crate::common::control::mount(instance.clone(), driver.clone(), control_settings.clone())
            .await?;
        crate::common::measure::mount(instance.clone(), driver.clone()).await?;

        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, _instance: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
