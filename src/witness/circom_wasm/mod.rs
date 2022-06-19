mod circom_wasmer;
mod circom_base;

pub use circom_base::CircomBase;
pub use circom_base::Circom;

#[cfg(feature = "circom-2")]
pub use circom_base::Circom2;

pub use circom_wasmer::Wasm;