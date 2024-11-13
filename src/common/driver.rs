use std::time::Duration;

use panduza_platform_connectors::{serial::generic::Driver as SerialDriver, SerialSettings};
use panduza_platform_core::Error;

///
///
///
pub struct KoradDriver {
    driver: SerialDriver,
}

impl KoradDriver {
    pub fn open(settings: &SerialSettings) -> Result<Self, Error> {
        let driver = SerialDriver::open(settings).map_err(|e| Error::Wtf)?;

        Ok(Self { driver: driver })
    }

    pub async fn get_idn(&mut self) -> Result<String, Error> {
        let mut response: [u8; 512] = [0; 512];

        let cmd = "*IDN?\n".as_bytes();

        let count = self
            .driver
            .write_then_read_until(cmd, &mut response, '\n' as u8)
            .await
            .map_err(|e| Error::Wtf)?;

        println!("{:?}", response[..count].to_vec());

        let string_slice = String::from_utf8(response[..count].to_vec()).unwrap();
        let string = string_slice.to_string();

        Ok(string)
    }
}
