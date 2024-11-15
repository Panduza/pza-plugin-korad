panduza_platform_core::plugin_interface!("korad");

pub mod common;
mod ka3005p;
mod kd3005p;
mod kd3005p_fake;

// Export the producers of the plugin
//
pub fn plugin_producers() -> Vec<Box<dyn Producer>> {
    let mut producers: Vec<Box<dyn Producer>> = vec![];
    producers.push(ka3005p::producer::KA3005P::new());
    producers.push(kd3005p::producer::KD3005P::new());
    producers.push(kd3005p_fake::producer::Kd3005pFake::new());
    return producers;
}
