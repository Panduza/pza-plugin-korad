use super::device::KD3005PFakeDevice;
use panduza_platform_core::{DeviceOperations, Producer};

pub struct KD3005P_fake {}

impl KD3005P_fake {
    pub fn new() -> Box<KD3005P_fake> {
        Box::new(KD3005P_fake {})
    }
}

impl Producer for KD3005P_fake {
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
