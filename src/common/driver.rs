use panduza_platform_core::protocol::CommandResponseProtocol;
use panduza_platform_core::Error;

///
///
///
pub struct KoradDriver<SD> {
    driver: SD,
}

impl<SD: CommandResponseProtocol> KoradDriver<SD> {
    ///
    ///
    ///
    pub fn new(driver: SD) -> Self {
        Self { driver: driver }
    }

    ///
    /// Get identity string
    ///
    pub async fn get_idn(&mut self) -> Result<String, Error> {
        let cmd = "*IDN?".to_string();
        let response = self.driver.ask(&cmd).await?;
        Ok(response)
    }

    ///
    /// Control current getter
    ///
    pub async fn get_iset(&mut self) -> Result<f32, Error> {
        let cmd = "ISET1?".to_string();
        let response = self.driver.ask(&cmd).await?;

        let value = response
            .parse::<f32>()
            .map_err(|e| Error::Generic(format!("{:?}", e)))?;

        Ok(value)
    }

    ///
    /// Control current getter
    ///
    pub async fn set_iset(&mut self, value: f32) -> Result<(), Error> {
        let cmd = format!("ISET1:{:.3}", value);
        self.driver.send(&cmd).await
    }

    ///
    /// Control current getter
    ///
    pub async fn get_vset(&mut self) -> Result<f32, Error> {
        let cmd = "VSET1?".to_string();
        let response = self.driver.ask(&cmd).await?;

        let value = response
            .parse::<f32>()
            .map_err(|e| Error::Generic(format!("{:?}", e)))?;

        Ok(value)
    }

    ///
    /// Control current getter
    ///
    pub async fn set_vset(&mut self, value: f32) -> Result<(), Error> {
        let cmd = format!("VSET1:{:.2}", value);
        self.driver.send(&cmd).await
    }

    ///
    ///
    ///
    pub async fn get_iout(&mut self) -> Result<f32, Error> {
        let cmd = "IOUT1?".to_string();
        let response = self.driver.ask(&cmd).await?;
        let value = response
            .parse::<f32>()
            .map_err(|e| Error::Generic(format!("{:?}", e)))?;
        Ok(value)
    }

    ///
    ///
    ///
    pub async fn get_vout(&mut self) -> Result<f32, Error> {
        let cmd = "VOUT1?".to_string();
        let response = self.driver.ask(&cmd).await?;
        let value = response
            .parse::<f32>()
            .map_err(|e| Error::Generic(format!("{:?}", e)))?;
        Ok(value)
    }
}
