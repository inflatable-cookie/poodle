//! Presentation metrics — per-size scalar resolvers shared by GPUI
//! components. Split into concern files; public surface unchanged via glob
//! re-export.

mod metrics_a;
mod metrics_b;
mod metrics_c;

pub use metrics_a::*;
pub use metrics_b::*;
pub use metrics_c::*;
