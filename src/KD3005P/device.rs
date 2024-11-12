use std::fmt::format;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;

use panduza_platform_connectors::SerialSettings;
use panduza_platform_connectors::UsbSettings;
use panduza_platform_core::spawn_on_command;
use panduza_platform_core::BidirMsgAtt;
use panduza_platform_core::DeviceLogger;
use panduza_platform_core::Interface;
use panduza_platform_core::StringCodec;
use panduza_platform_core::StringListCodec;
use panduza_platform_core::TaskResult;
use panduza_platform_core::{Device, DeviceOperations, Error};
use serde_json::json;
use tokio::time::sleep;

static DEVICE_VENDOR_ID: u16 = 0x0416;
static DEVICE_PRODUCT_ID: u16 = 0x5011;
static DEVICE_SERIAL_BAUDRATE: u32 = 9600; // We do not care... it is USB serial

///
/// Device to control PicoHA Dio Board
///
pub struct KD3005PDevice {
    ///
    /// Device logger
    logger: Option<DeviceLogger>,
    ///
    /// Serial settings to connect to the pico
    serial_settings: Option<SerialSettings>,
    // ///
    // /// Connector to communicate with the pico
    // connector: Option<Connector>,

    // ///
    // ///
    // pico_connector: Option<PicoHaDioConnector>,
}

impl KD3005PDevice {
    ///
    /// Constructor
    ///
    pub fn new() -> Self {
        KD3005PDevice {
            logger: None,
            serial_settings: None,
            // connector: None,
            // pico_connector: None,
        }
    }

    ///
    /// Prepare settings of the device
    ///
    pub async fn prepare_settings(&mut self, device: Device) -> Result<(), Error> {
        // Get the device logger
        let logger = device.logger.clone();

        // Get the device settings
        let json_settings = device.settings().await.or(Some(json!({}))).unwrap();

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

    // ///
    // /// Try to mount the connector to reach the device
    // ///
    // pub async fn mount_connector(&mut self) -> Result<(), Error> {
    //     //
    //     // Recover settings
    //     let settings = self.serial_settings.as_ref().ok_or(Error::BadSettings(
    //         "Serial Settings not provided".to_string(),
    //     ))?;
    //     //
    //     // Try to get connector
    //     self.connector = Some(
    //         get_connector(settings)
    //             .await
    //             .map_err(|e| Error::Generic(e.to_string()))?,
    //     );
    //     //
    //     // Try to init it
    //     self.connector
    //         .as_ref()
    //         .ok_or(Error::BadSettings(
    //             "Connector is not initialized".to_string(),
    //         ))?
    //         .lock()
    //         .await
    //         .init()
    //         .await
    //         .map_err(|e| Error::Generic(e.to_string()))?;

    //     //
    //     self.pico_connector = Some(PicoHaDioConnector::new(
    //         self.logger.as_ref().unwrap().clone(),
    //         self.connector.as_ref().unwrap().clone(),
    //     ));

    //     Ok(())
    // }
}

#[async_trait]
impl DeviceOperations for KD3005PDevice {
    ///
    ///
    ///
    async fn mount(&mut self, mut device: Device) -> Result<(), Error> {
        //
        // Init logger
        self.logger = Some(device.logger.clone());

        //
        //
        self.prepare_settings(device.clone()).await?;

        //
        //
        // self.mount_connector().await?;

        // self.create_io_interfaces(device.clone()).await?;

        // self.pico_get_direction(2).await?;

        // une interface pour chaque io_%d
        //
        // io_%d/direction              meta : enum
        // io_%d/direction/choices      list of string
        // io_%d/direction/value        string
        // io_%d/value           (enum/string) set/get (when input cannot be set)
        // io_%d/trigger_read    (boolean) start an input reading (oneshot)
        //

        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut device: Device) {
        sleep(Duration::from_secs(5)).await;
    }
}
