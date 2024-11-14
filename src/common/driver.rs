use panduza_platform_core::drivers::serial::SerialDriver;
use panduza_platform_core::drivers::serial::Settings as SerialSettings;
use panduza_platform_core::Error;

///
///
///
pub struct KoradDriver<SD> {
    driver: SD,
}

impl<SD: SerialDriver> KoradDriver<SD> {
    ///
    ///
    ///
    pub fn new(driver: SD) -> Self {
        Self { driver: driver }
    }

    pub async fn get_idn(&mut self) -> Result<String, Error> {
        let mut response: [u8; 512] = [0; 512];

        // let cmd = "*IDN?\n".as_bytes();
        let cmd = "*IDN?".as_bytes();

        let count = self.driver.write_then_read(cmd, &mut response).await?;

        println!("{:?}", response[..count].to_vec());

        // count -1 because we remove the '\n'
        let string_slice = String::from_utf8(response[..count - 1].to_vec()).unwrap();
        let string = string_slice.to_string();

        Ok(string)
    }

    ///
    /// Control current getter
    ///
    pub async fn get_iset(&mut self) -> Result<f32, Error> {
        let mut response: [u8; 512] = [0; 512];

        // let cmd = "ISET1?\n".as_bytes();
        let cmd = "ISET1?".as_bytes();

        let count = self.driver.write_then_read(cmd, &mut response).await?;

        println!("{:?}", response[..count].to_vec());

        let string_slice = String::from_utf8(response[..count - 1].to_vec()).unwrap();
        let string = string_slice.to_string();

        let value = string
            .parse::<f32>()
            .map_err(|e| Error::Generic(format!("{:?}", e)))?;

        Ok(value)
    }

    ///
    /// Control current getter
    ///
    pub async fn set_iset(&mut self, value: f32) -> Result<(), Error> {
        // let mut response: [u8; 512] = [0; 512];

        // let pp = format!("ISET1:{:.3}\n", value);
        let pp = format!("ISET1:{:.3}", value);
        let cmd = pp.as_str().as_bytes();

        // println!("cmd -> {:?}", cmd);

        let _count = self.driver.write(cmd).await?;

        Ok(())
    }

    ///
    /// Control current getter
    ///
    pub async fn get_vset(&mut self) -> Result<f32, Error> {
        let mut response: [u8; 512] = [0; 512];

        // let cmd = "VSET1?\n".as_bytes();
        let cmd = "VSET1?".as_bytes();

        let count = self.driver.write_then_read(cmd, &mut response).await?;

        println!("{:?}", response[..count].to_vec());

        let string_slice = String::from_utf8(response[..count - 1].to_vec()).unwrap();
        let string = string_slice.to_string();

        let value = string
            .parse::<f32>()
            .map_err(|e| Error::Generic(format!("{:?}", e)))?;

        Ok(value)
    }

    ///
    /// Control current getter
    ///
    pub async fn set_vset(&mut self, value: f32) -> Result<(), Error> {
        // let mut response: [u8; 512] = [0; 512];

        // let pp = format!("VSET1:{:.2}\n", value);
        let pp = format!("VSET1:{:.2}", value);
        let cmd = pp.as_str().as_bytes();

        // println!("cmd -> {:?}", cmd);

        let _count = self.driver.write(cmd).await.map_err(|_e| Error::Wtf)?;

        Ok(())
    }

    ///
    ///
    ///
    pub async fn get_iout(&mut self) -> Result<f32, Error> {
        let mut response: [u8; 512] = [0; 512];

        // let cmd = "IOUT1?\n".as_bytes();
        let cmd = "IOUT1?".as_bytes();

        let count = self
            .driver
            .write_then_read(cmd, &mut response)
            .await
            .map_err(|_e| Error::Wtf)?;

        println!("{:?}", response[..count].to_vec());

        let string_slice = String::from_utf8(response[..count - 1].to_vec()).unwrap();
        let string = string_slice.to_string();

        let value = string
            .parse::<f32>()
            .map_err(|e| Error::Generic(format!("{:?}", e)))?;

        Ok(value)
    }

    ///
    ///
    ///
    pub async fn get_vout(&mut self) -> Result<f32, Error> {
        let mut response: [u8; 512] = [0; 512];

        // let cmd = "VOUT1?\n".as_bytes();
        let cmd = "VOUT1?".as_bytes();

        let count = self
            .driver
            .write_then_read(cmd, &mut response)
            .await
            .map_err(|_e| Error::Wtf)?;

        println!("{:?}", response[..count].to_vec());

        let string_slice = String::from_utf8(response[..count - 1].to_vec()).unwrap();
        let string = string_slice.to_string();

        let value = string
            .parse::<f32>()
            .map_err(|e| Error::Generic(format!("{:?}", e)))?;

        Ok(value)
    }
}
