use super::device::KD3005PDevice;
use panduza_platform_core::{DeviceOperations, Producer};

pub struct KD3005P {}

impl KD3005P {
    pub fn new() -> Box<KD3005P> {
        Box::new(KD3005P {})
    }
}

impl Producer for KD3005P {
    fn manufacturer(&self) -> String {
        "korad".to_string()
    }

    fn model(&self) -> String {
        "KD3005P".to_string()
    }

    fn produce(&self) -> Result<Box<dyn DeviceOperations>, panduza_platform_core::Error> {
        return Ok(Box::new(KD3005PDevice::new()));
    }
}
