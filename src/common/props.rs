///
///
///
struct PowerSupplyProps {
    min_voltage: f32,
    max_voltage: f32,
    min_current: f32,
    max_current: f32,
}

impl PowerSupplyProps {
    pub fn new() -> Self {
        Self {
            min_voltage: 0.0,
            max_voltage: 0.0,
            min_current: 0.0,
            max_current: 0.0,
        }
    }
}
