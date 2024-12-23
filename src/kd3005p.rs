pub mod device;

use panduza_platform_core::{DriverOperations, Producer};

pub use super::common::settings::Settings as ControlSettings;

#[derive(Default)]
pub struct Package {}

impl Package {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Producer for Package {
    fn manufacturer(&self) -> String {
        "korad".to_string()
    }

    fn model(&self) -> String {
        "KD3005P".to_string()
    }

    fn description(&self) -> String {
        "Driver for KD3005P Power Supply".to_string()
    }

    fn props(&self) -> panduza_platform_core::Props {
        //
        //
        let mut props = panduza_platform_core::Props::default();

        //
        //
        let min_max_voltage = panduza_platform_core::std::prop::min_max::Settings::new(
            "voltage",
            "voltage limit",
            0,
            30,
            None,
        );
        min_max_voltage.declare(&mut props);

        //
        //
        let min_max_voltage = panduza_platform_core::std::prop::min_max::Settings::new(
            "current",
            "current limit",
            0,
            3,
            None,
        );
        min_max_voltage.declare(&mut props);

        //
        //
        props
    }

    fn produce(&self) -> Result<Box<dyn DriverOperations>, panduza_platform_core::Error> {
        return Ok(Box::new(device::Device::new()));
    }
}
