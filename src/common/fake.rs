use async_trait::async_trait;
use panduza_platform_core::{protocol::CommandResponseProtocol, Error};

///
///
pub struct Driver {}

impl Driver {
    /// Create a new instance of the driver
    ///
    pub fn open() -> Result<Self, Error> {
        Ok(Self {})
    }
}

#[async_trait]
impl CommandResponseProtocol for Driver {
    ///
    ///
    ///
    async fn send(&mut self, command: &String) -> Result<(), Error> {
        println!("Fake Send {:?}", command);
        Ok(())
    }

    ///
    ///
    ///
    async fn ask(&mut self, command: &String) -> Result<String, Error> {
        //
        //
        println!("Fake Ask {:?}", command);
        //
        //
        if *command == "*IDN".to_string() {
            return Ok("Fake Device !!!".to_string());
        }
        if *command == "VOUT1".to_string() {
            return Ok("2.0".to_string());
        }
        if *command == "ISET1?".to_string() {
            return Ok("2.0".to_string());
        }
        if *command == "VSET1?".to_string() {
            return Ok("2.0".to_string());
        }

        return Ok("1".to_string());
    }
}
