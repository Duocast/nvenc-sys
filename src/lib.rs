//! NvEnc API binding library (unstable)
//! This exposes a subset of the CUDA library necessary to use the NVENC API.
//! with CUDA surfaces

mod nvenc;
pub use nvenc::*;

pub mod encoder;
pub mod input_buffer;
pub mod bitstream;

#[cfg(feature = "cuda")]
pub mod cuda;
