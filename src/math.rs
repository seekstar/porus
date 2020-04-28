pub use core::f64::consts::PI;

#[must_use]
pub fn fabs(x: f64) -> f64 {
    unsafe { core::intrinsics::fabsf64(x) }
}

#[must_use]
pub fn cos(x: f64) -> f64 {
    unsafe { core::intrinsics::cosf64(x) }
}

#[must_use]
pub fn powi(x: f64, y: i32) -> f64 {
    unsafe { core::intrinsics::powif64(x, y) }
}

#[must_use]
pub fn round(x: f64) -> f64 {
    unsafe { core::intrinsics::roundf64(x) }
}

#[must_use]
pub fn sin(x: f64) -> f64 {
    unsafe { core::intrinsics::sinf64(x) }
}

#[must_use]
pub fn sqrt(x: f64) -> f64 {
    unsafe { core::intrinsics::sqrtf64(x) }
}
