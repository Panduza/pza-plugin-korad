use super::device::KD3005PFakeDevice;
use panduza_platform_core::{DriverOperations, Producer};

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

    fn description(&self) -> String {
        "Virtual implementation of KD3005P Power Supply".to_string()
    }

    fn props(&self) -> panduza_platform_core::Props {
        panduza_platform_core::Props::default()
    }

    fn produce(&self) -> Result<Box<dyn DriverOperations>, panduza_platform_core::Error> {
        return Ok(Box::new(KD3005PFakeDevice::new()));
    }
}
