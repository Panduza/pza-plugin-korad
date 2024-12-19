use super::ControlSettings;
use crate::common::driver::KoradDriver;
use async_trait::async_trait;
use panduza_platform_core::connector::serial::eol::Driver as SerialEolDriver;
use panduza_platform_core::connector::serial::Settings as SerialSettings;
use panduza_platform_core::connector::usb::Settings as UsbSettings;
use panduza_platform_core::Instance;
use panduza_platform_core::{
    log_info, log_info_mount_end, log_info_mount_start, DriverOperations, Error,
};
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

static DEVICE_VENDOR_ID: u16 = 0x0416;
static DEVICE_PRODUCT_ID: u16 = 0x5011;
static DEVICE_SERIAL_BAUDRATE: u32 = 9600; // We do not care... it is USB serial

#[derive(Default)]
///
///
pub struct Device {}

impl Device {
    /// Constructor
    ///
    pub fn new() -> Self {
        Self {}
    }

    /// Prepare serial settings
    ///
    pub async fn prepare_serial_settings(
        &mut self,
        instance: Instance,
    ) -> Result<SerialSettings, Error> {
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
        let serial_settings = SerialSettings::new()
            .set_port_name_from_json_or_usb_settings(&json_settings, &usb_settings)
            .map_err(|e| Error::Generic(e.to_string()))?
            .set_baudrate(DEVICE_SERIAL_BAUDRATE);

        Ok(serial_settings)
    }

    /// Try to mount the connector to reach the device
    ///
    pub fn mount_driver(
        &mut self,
        instance: Instance,
        serial_settings: SerialSettings,
    ) -> Result<Arc<Mutex<KoradDriver<SerialEolDriver>>>, Error> {
        let driver = SerialEolDriver::open(&serial_settings, vec![b'\n'])?;
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
impl DriverOperations for Device {
    ///
    ///
    ///
    async fn mount(&mut self, instance: Instance) -> Result<(), Error> {
        //
        //
        let logger = instance.logger.clone();
        log_info_mount_start!(logger);

        //
        //
        let serial_settings = self.prepare_serial_settings(instance.clone()).await?;
        log_info!(logger, "serial_settings = {:?}", serial_settings);

        //
        //
        let control_settings = self.prepare_control_settings(instance.clone()).await?;
        log_info!(logger, "control_settings = {:?}", control_settings);

        //
        //
        let driver = self.mount_driver(instance.clone(), serial_settings)?;

        //
        // Identity
        panduza_platform_core::std::attribute::idn::mount(instance.clone(), driver.clone()).await?;

        //
        //
        crate::common::control::mount(instance.clone(), driver.clone()).await?;

        //
        //
        crate::common::measure::mount(instance.clone(), driver.clone()).await?;

        //
        //
        log_info_mount_end!(logger);
        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, _instance: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
