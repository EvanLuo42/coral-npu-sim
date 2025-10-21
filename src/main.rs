use crate::scalar::core::ScalarFrontend;

pub mod scalar;
pub mod vector;
pub mod matrix;
pub mod common;

fn main() {
    tracing_subscriber::fmt::init();
    let mut scalar_frontend = ScalarFrontend::new();
    for _ in 0..4 {
        scalar_frontend.tick()
    }
}
