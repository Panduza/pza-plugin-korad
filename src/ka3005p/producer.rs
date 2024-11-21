use super::device::KA3005PDevice;
use panduza_platform_core::{DriverOperations, Producer};

pub struct KA3005P {}

impl KA3005P {
    pub fn new() -> Box<KA3005P> {
        Box::new(KA3005P {})
    }
}

impl Producer for KA3005P {
    fn manufacturer(&self) -> String {
        "korad".to_string()
    }

    fn model(&self) -> String {
        "KA3005P".to_string()
    }

    fn description(&self) -> String {
        "Driver for KA3005P Power Supply".to_string()
    }

    fn props(&self) -> panduza_platform_core::Props {
        panduza_platform_core::Props::default()
    }

    fn produce(&self) -> Result<Box<dyn DriverOperations>, panduza_platform_core::Error> {
        return Ok(Box::new(KA3005PDevice::new()));
    }
}
