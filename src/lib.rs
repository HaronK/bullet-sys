#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]
#![warn(unused_extern_crates)]

// Bullet

#[cfg(target_os = "linux")]
mod bullet_linux;

#[cfg(target_os = "linux")]
pub use bullet_linux::*;

#[cfg(target_os = "windows")]
mod bullet_windows;

#[cfg(target_os = "windows")]
pub use bullet_windows::*;

// Bullet3

#[cfg(target_os = "linux")]
mod bullet3_linux;

#[cfg(target_os = "linux")]
pub use bullet3_linux::*;

#[cfg(target_os = "windows")]
mod bullet3_windows;

#[cfg(target_os = "windows")]
pub use bullet3_windows::*;

#[test]
pub fn test() {
    unsafe {
        let mut ghostObject = btPairCachingGhostObject::new();
        let boxHalfExtents = btVector3::new1(&1f64, &1f64, &1f64);
        let mut convexShape = btBoxShape::new(&boxHalfExtents);

        let up = btVector3::new1(&1f64,&0f64,&0f64);
        let tested = btKinematicCharacterController::new(&mut ghostObject, &mut convexShape._base._base._base, 1f64, &up);

        assert_eq!(-9.8 * 3.0, *tested.getGravity().x());
        assert_eq!(0.0, *tested.getGravity().y());
        assert_eq!(0.0, *tested.getGravity().z());
    }
}
