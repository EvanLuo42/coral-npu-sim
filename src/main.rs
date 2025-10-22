use crate::scalar::core::ScalarFrontend;

pub mod scalar;
pub mod vector;
pub mod matrix;
pub mod common;

fn main() {
    tracing_subscriber::fmt::init();
    let mut scalar_frontend = ScalarFrontend::new();
    for cycle in 0..4 {
        println!("\n===== Cycle {cycle} =====");
        scalar_frontend.tick()
    }
}
