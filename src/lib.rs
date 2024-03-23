#[cfg(feature = "default")]
mod default;

#[cfg(feature = "default")]
pub use default::M2M;

#[cfg(feature = "smallvec")]
pub use pseudo_small_m2m::SmallM2M;
