pub use super::ControlSettings;

use crate::common::driver::KoradDriver;
use async_trait::async_trait;
use panduza_platform_core::connector::serial::time_lock::Driver as SerialTimeLockDriver;
use panduza_platform_core::connector::serial::Settings as SerialSettings;
use panduza_platform_core::connector::usb::Settings as UsbSettings;
use panduza_platform_core::Instance;
use panduza_platform_core::{log_info, DriverOperations, Error};
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

static DEVICE_VENDOR_ID: u16 = 0x0416;
static DEVICE_PRODUCT_ID: u16 = 0x5011;
static DEVICE_SERIAL_BAUDRATE: u32 = 9600; // We do not care... it is USB serial

///
/// Device to control PicoHA Dio Board
///
pub struct KA3005PDevice {
    ///
    /// Serial settings to connect to the pico
    serial_settings: Option<SerialSettings>,
}

impl KA3005PDevice {
    ///
    /// Constructor
    ///
    pub fn new() -> Self {
        KA3005PDevice {
            serial_settings: None,
        }
    }

    ///
    /// Prepare settings of the device
    ///
    pub async fn prepare_settings(&mut self, instance: Instance) -> Result<(), Error> {
        // Get the device logger
        let logger = instance.logger.clone();

        // Get the device settings
        let json_settings = instance.settings().await.or(Some(json!({}))).unwrap();

        // Log debug info
        logger.info(format!("JSON settings: {:?}", json_settings));

        // Usb settings
        let usb_settings = UsbSettings::new()
            .set_vendor(DEVICE_VENDOR_ID)
            .set_model(DEVICE_PRODUCT_ID)
            .optional_set_serial_from_json_settings(&json_settings);
        logger.info(format!("USB settings: {}", usb_settings));

        // Serial settings
        self.serial_settings = Some(
            SerialSettings::new()
                .set_port_name_from_json_or_usb_settings(&json_settings, &usb_settings)
                .map_err(|e| Error::Generic(e.to_string()))?
                .set_baudrate(DEVICE_SERIAL_BAUDRATE),
        );

        Ok(())
    }

    ///
    /// Try to mount the connector to reach the device
    ///
    pub fn mount_driver(
        &mut self,
        instance: Instance,
    ) -> Result<Arc<Mutex<KoradDriver<SerialTimeLockDriver>>>, Error> {
        //
        // Recover settings
        let settings = self.serial_settings.as_ref().ok_or(Error::BadSettings(
            "Serial Settings not provided".to_string(),
        ))?;

        let driver = SerialTimeLockDriver::open(settings, Duration::from_millis(300))?;

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
impl DriverOperations for KA3005PDevice {
    ///
    ///
    ///
    async fn mount(&mut self, instance: Instance) -> Result<(), Error> {
        //
        //
        let logger = instance.logger.clone();

        //
        //
        self.prepare_settings(instance.clone()).await?;

        //
        //
        let control_settings = self.prepare_control_settings(instance.clone()).await?;
        log_info!(logger, "control_settings = {:?}", control_settings);

        let driver = self.mount_driver(instance.clone())?;

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
