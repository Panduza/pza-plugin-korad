use crate::common::identity;
use async_trait::async_trait;
use panduza_platform_connectors::SerialSettings;
use panduza_platform_core::DeviceLogger;
use panduza_platform_core::{Device, DeviceOperations, Error};
use std::time::Duration;
use tokio::time::sleep;

///
/// Device to control PicoHA Dio Board
///
pub struct KD3005PFakeDevice {
    ///
    /// Device logger
    logger: Option<DeviceLogger>,
    // ///
    // /// Connector to communicate with the pico
    // connector: Option<Connector>,

    // ///
    // ///
    // pico_connector: Option<PicoHaDioConnector>,
}

impl KD3005PFakeDevice {
    ///
    /// Constructor
    ///
    pub fn new() -> Self {
        KD3005PFakeDevice {
            logger: None,
            // connector: None,
            // pico_connector: None,
        }
    }

    // ///
    // /// Prepare settings of the device
    // ///
    // pub async fn prepare_settings(&mut self, device: Device) -> Result<(), Error> {
    //     // Get the device logger
    //     let logger = device.logger.clone();

    //     // Get the device settings
    //     let json_settings = device.settings().await.or(Some(json!({}))).unwrap();

    //     // Log debug info
    //     logger.info("Build interfaces for \"picoha.dio\" device");
    //     logger.info(format!("JSON settings: {:?}", json_settings));

    //     // Usb settings
    //     let usb_settings = UsbSettings::new()
    //         .set_vendor(PICOHA_VENDOR_ID)
    //         .set_model(PICOHA_PRODUCT_ID)
    //         .optional_set_serial_from_json_settings(&json_settings);
    //     logger.info(format!("USB settings: {}", usb_settings));

    //     // Serial settings
    //     self.serial_settings = Some(
    //         SerialSettings::new()
    //             .set_port_name_from_json_or_usb_settings(&json_settings, &usb_settings)
    //             .map_err(|e| Error::Generic(e.to_string()))?
    //             .set_baudrate(PICOHA_SERIAL_BAUDRATE),
    //     );

    //     Ok(())
    // }

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
impl DeviceOperations for KD3005PFakeDevice {
    ///
    ///
    ///
    async fn mount(&mut self, mut device: Device) -> Result<(), Error> {
        //
        // Init logger
        self.logger = Some(device.logger.clone());

        identity::mount_identity_fake(device.clone(), "FAKE - KD3005P").await;

        // "identity" // IDN string

        // "control" { // class
        //     "output_enable" // OUT (boolean) control

        //     "voltage": {// class - tag SI
        //         "value" // VSET
        //         "unit"  // String "V"
        //     },
        //     "current": {// class - tag SI
        //         "value" // ISET
        //         "unit"  // String "A"
        //     },

        //     "options": {// class
        //         "ocp" // OCP (boolean) control
        //         "ovp" // OVP (boolean) control
        //         "beep" // BEEP (boolean)
        //         "Lock" // status
        //         "mode" { // class enum string
        //             "choices" // ["C.C"] ["C.V"]
        //             "value"  // string
        //         },
        //         "Tracking" // ????  there is a ref in the doc but...
        //     }
        // },
        // "measure" { // class
        //     "voltmeter" { // class - tag SI
        //         "value" // VOUT
        //         "unit"  // String "V"
        //     },
        //     "ampermeter" {// class - tag SI
        //         "value" // IOUT
        //         "unit"  // String "A"
        //     }
        // },

        // KORAD KD3005P V6.8 SN:03471643

        // self.prepare_settings(device.clone()).await?;
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
