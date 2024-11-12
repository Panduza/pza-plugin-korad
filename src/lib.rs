panduza_platform_core::plugin_interface!("korad");

mod KA3005P;
mod KD3005P;
mod KD3005P_fake;

// Export the producers of the plugin
//
pub fn plugin_producers() -> Vec<Box<dyn Producer>> {
    let mut producers: Vec<Box<dyn Producer>> = vec![];
    producers.push(KA3005P::producer::KA3005P::new());
    producers.push(KD3005P::producer::KD3005P::new());
    producers.push(KD3005P_fake::producer::KD3005P_fake::new());
    return producers;
}
