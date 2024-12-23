use panduza_platform_core::std::prop::min_max::Settings as MinMaxSettings;
use panduza_platform_core::{Error, InstanceSettings};

///
///
#[derive(Debug, Clone)]
pub struct Settings {
    pub min_max_voltage: MinMaxSettings,
    pub min_max_current: MinMaxSettings,
}

impl Settings {
    ///
    ///
    pub fn new() -> Self {
        Self {
            min_max_voltage: MinMaxSettings::new("voltage", "voltage limit", 0, 30, None),
            min_max_current: MinMaxSettings::new("current", "current limit", 0, 3, None),
        }
    }

    pub fn min_voltage(&self) -> f64 {
        return self.min_max_voltage.min;
    }
    pub fn max_voltage(&self) -> f64 {
        return self.min_max_voltage.max;
    }
    pub fn min_current(&self) -> f64 {
        return self.min_max_current.min;
    }
    pub fn max_current(&self) -> f64 {
        return self.min_max_current.max;
    }

    ///
    ///
    pub fn override_with_instance_settings(
        &mut self,
        settings: &Option<InstanceSettings>,
    ) -> Result<(), Error> {
        self.min_max_voltage
            .override_with_instance_settings(settings)?;
        self.min_max_current
            .override_with_instance_settings(settings)?;
        Ok(())
    }
}
