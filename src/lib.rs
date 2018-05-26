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
