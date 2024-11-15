use super::device::KD3005PFakeDevice;
use panduza_platform_core::{DeviceOperations, Producer};

pub struct Kd3005pFake {}

impl Kd3005pFake {
    pub fn new() -> Box<Kd3005pFake> {
        Box::new(Kd3005pFake {})
    }
}

impl Producer for Kd3005pFake {
    fn manufacturer(&self) -> String {
        "korad".to_string()
    }

    fn model(&self) -> String {
        "KD3005P_fake".to_string()
    }

    fn produce(&self) -> Result<Box<dyn DeviceOperations>, panduza_platform_core::Error> {
        return Ok(Box::new(KD3005PFakeDevice::new()));
    }
}
