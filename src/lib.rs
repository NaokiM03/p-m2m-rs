#[cfg(feature = "std")]
mod stdvec;

#[cfg(feature = "std")]
pub use stdvec::M2M;

#[cfg(feature = "smallvec")]
mod smallvec;

#[cfg(feature = "smallvec")]
pub use smallvec::SmallM2M;
