#[cfg(feature = "smallvec")]
mod smallvec;
#[cfg(feature = "default")]
mod stdvec;

#[cfg(feature = "smallvec")]
pub use smallvec::SmallM2M;
#[cfg(feature = "default")]
pub use stdvec::M2M;
