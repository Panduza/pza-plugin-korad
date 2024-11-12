use futures::TryFutureExt;
use panduza_platform_connectors::{serial::generic::Driver as SerialDriver, SerialSettings};
use panduza_platform_core::Error;

///
///
///
pub struct KaDriver {
    driver: SerialDriver,
}

impl KaDriver {
    pub fn new(settings: &SerialSettings) -> Result<Self, Error> {
        let mut driver = SerialDriver::new(settings);

        driver
            .init()
            .map_err(|e| Error::Generic(format!("{:?}", e)))?;

        Ok(Self { driver: driver })
    }

    pub async fn get_idn(&self) -> Result<String, Error> {
        // let response: [u8];

        // self.driver.write_then_read("*IDN?", response);

        Ok("String".to_string())
    }
}
