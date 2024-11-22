use crate::common::{driver::KoradDriver, fake::Driver as SerialFakeDriver};
use async_trait::async_trait;
use panduza_platform_core::{Instance, DriverOperations, Error};
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
    pub fn mount_driver(&mut self) -> Result<Arc<Mutex<KoradDriver<SerialFakeDriver>>>, Error> {
        let driver = SerialFakeDriver::open()?;

        let kdriver = KoradDriver::new(driver);

        Ok(Arc::new(Mutex::new(kdriver)))
    }
}

#[async_trait]
impl DriverOperations for KD3005PFakeDevice {
    ///
    ///
    ///
    async fn mount(&mut self, instance: Instance) -> Result<(), Error> {
        let driver = self.mount_driver()?;

        crate::common::real::identity::mount(instance.clone(), driver.clone()).await?;
        crate::common::real::control::mount(instance.clone(), driver.clone()).await?;
        crate::common::real::measure::mount(instance.clone(), driver.clone()).await?;

        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, _instance: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
