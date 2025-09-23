use async_trait::async_trait;
use panduza_platform_core::protocol::{AsciiCmdRespProtocol, BytesDialogProtocol};
// use panduza_platform_core::template::attribute::idn::IdnReader;
use panduza_platform_core::{log_trace, Error, Logger};
use std::time::Instant;

///
///
///
pub struct KoradDriver<SD> {
    /// Physical level driver
    ///
    driver: SD,

    /// Logger for the driver
    ///
    logger: Logger,
}

// #[async_trait]
// impl<SD: AsciiCmdRespProtocol> IdnReader for KoradDriver<SD> {
//     async fn read_idn(&mut self) -> Result<String, Error> {
//         //
//         // Measure perfs
//         let start = Instant::now();

//         //
//         // Perform request
//         let cmd = "*IDN?".to_string();
//         let response = self.driver.ask(&cmd).await?;

//         //
//         // Log
//         log_trace!(
//             self.logger,
//             "ASK <=> {:?} - {:?} - {:.2?}",
//             cmd,
//             response,
//             start.elapsed()
//         );

//         //
//         // End
//         Ok(response)
//     }
// }

impl<SD: BytesDialogProtocol> KoradDriver<SD> {
    ///
    /// Create a new driver
    ///
    pub fn new(driver: SD, logger: Logger) -> Self {
        Self {
            driver: driver,
            logger: logger,
        }
    }

    ///
    /// Get identity string
    ///
    pub async fn get_idn(&mut self) -> Result<String, Error> {
        //
        // Measure perfs
        let start = Instant::now();

        //
        // Perform request
        let cmd = bytes::Bytes::from("*IDN?");

        //
        // Log
        // log_trace!(
        //     self.logger,
        //     "ASK <=> {:?} - {:?} - {:.2?}",
        //     cmd.to_vec(),
        //     response,
        //     start.elapsed()
        // );

        //
        let response = self.driver.ask(cmd).await?;

        //
        // End
        // Ok(std::str::from_utf8(&response)?.to_string())

        Ok(String::from_utf8({ response }.to_vec()).unwrap())
    }

    ///
    /// Control current getter
    ///
    pub async fn get_iset(&mut self) -> Result<f32, Error> {
        //
        // Measure perfs
        let start = Instant::now();

        //
        // Perform request
        let cmd = bytes::Bytes::from("ISET1?");
        let response = self.driver.ask(cmd).await?;

        //
        // Log
        // log_trace!(
        //     self.logger,
        //     "ASK <=> {:?} - {:?} - {:.2?}",
        //     cmd,
        //     response,
        //     start.elapsed()
        // );

        //
        // End
        let response_str =
            String::from_utf8(response.to_vec()).map_err(|e| Error::Generic(format!("{:?}", e)))?;
        let value = response_str
            .trim()
            .parse::<f32>()
            .map_err(|e| Error::Generic(format!("{:?}", e)))?;
        Ok(value)
    }

    ///
    /// Control current getter
    ///
    pub async fn set_iset(&mut self, value: f32) -> Result<(), Error> {
        let cmd = format!("ISET1:{:.3}", value);
        self.driver.tell(bytes::Bytes::from(cmd)).await
    }

    ///
    /// Control current getter
    ///
    pub async fn get_vset(&mut self) -> Result<f32, Error> {
        //
        // Measure perfs
        let start = Instant::now();

        //
        // Perform request
        let cmd = bytes::Bytes::from("VSET1?");
        let response = self.driver.ask(cmd).await?;

        //
        // Log
        // log_trace!(
        //     self.logger,
        //     "ASK <=> {:?} - {:?} - {:.2?}",
        //     cmd,
        //     response,
        //     start.elapsed()
        // );

        //
        // End
        let response_str =
            String::from_utf8(response.to_vec()).map_err(|e| Error::Generic(format!("{:?}", e)))?;
        let value = response_str
            .trim()
            .parse::<f32>()
            .map_err(|e| Error::Generic(format!("{:?}", e)))?;
        Ok(value)
    }

    ///
    /// Control current getter
    ///
    pub async fn set_vset(&mut self, value: f32) -> Result<(), Error> {
        let cmd = format!("VSET1:{:.2}", value);
        self.driver.tell(bytes::Bytes::from(cmd)).await
    }

    ///
    ///
    ///
    pub async fn get_iout(&mut self) -> Result<f32, Error> {
        let cmd = bytes::Bytes::from("IOUT1?");
        let response = self.driver.ask(cmd).await?;
        let response_str =
            String::from_utf8(response.to_vec()).map_err(|e| Error::Generic(format!("{:?}", e)))?;
        let value = response_str
            .trim()
            .parse::<f32>()
            .map_err(|e| Error::Generic(format!("{:?}", e)))?;
        Ok(value)
    }

    ///
    ///
    ///
    pub async fn get_vout(&mut self) -> Result<f32, Error> {
        let cmd = bytes::Bytes::from("VOUT1?");
        let response = self.driver.ask(cmd).await?;
        // let response = self.driver.ask(&cmd).await?;
        let response_str =
            String::from_utf8(response.to_vec()).map_err(|e| Error::Generic(format!("{:?}", e)))?;
        let value = response_str
            .trim()
            .parse::<f32>()
            .map_err(|e| Error::Generic(format!("{:?}", e)))?;
        Ok(value)
    }

    ///
    ///
    ///
    pub async fn set_out(&mut self, value: bool) -> Result<(), Error> {
        match value {
            true => {
                let cmd = bytes::Bytes::from("OUT1");
                self.driver.tell(cmd).await
            }
            false => {
                let cmd = bytes::Bytes::from("OUT0");
                self.driver.tell(cmd).await
            }
        }
    }

    ///
    ///
    ///
    pub async fn get_out(&mut self) -> Result<bool, Error> {
        //
        // Measure perfs
        let start = Instant::now();

        //
        //
        let cmd = bytes::Bytes::from("STATUS?");
        let response = self.driver.ask(cmd).await?;

        //
        // Log
        // log_trace!(
        //     self.logger,
        //     "ASK <=> {:?} - {:?} (as bytes/{:?}) - {:.2?}",
        //     cmd,
        //     response,
        //     response.as_bytes(),
        //     start.elapsed()
        // );

        let byte = response[0];
        if (byte & (1 << 6)) == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    ///
    ///
    ///
    pub async fn set_beep(&mut self, value: bool) -> Result<(), Error> {
        match value {
            true => {
                let cmd = bytes::Bytes::from("BEEP1");
                self.driver.tell(cmd).await
            }
            false => {
                let cmd = bytes::Bytes::from("BEEP0");
                self.driver.tell(cmd).await
            }
        }
    }

    ///
    ///
    ///
    pub async fn get_beep(&mut self) -> Result<bool, Error> {
        let cmd = bytes::Bytes::from("STATUS?");
        let response = self.driver.ask(cmd).await?;
        let byte = response[0];
        if (byte & (1 << 4)) == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    ///
    ///
    ///
    pub async fn set_ocp(&mut self, value: bool) -> Result<(), Error> {
        //
        // Prepare command
        let mut cmd = bytes::Bytes::from("OCP0");
        if value {
            cmd = bytes::Bytes::from("OCP1");
        }

        //
        // Send
        log_trace!(self.logger, "SEND => {:?}", cmd);
        self.driver.tell(cmd).await
    }

    ///
    ///
    ///
    pub async fn set_ovp(&mut self, value: bool) -> Result<(), Error> {
        match value {
            true => {
                let cmd = bytes::Bytes::from("OVP1");
                self.driver.tell(cmd).await
            }
            false => {
                let cmd = bytes::Bytes::from("OVP0");
                self.driver.tell(cmd).await
            }
        }
    }
}
