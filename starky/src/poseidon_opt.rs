#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub mod general;

#[cfg(target_feature = "avx2")]
pub use self::x86_64::avx2_poseidon_gl::*;

#[cfg(not(target_feature = "avx2"))]
pub use self::general::*;
