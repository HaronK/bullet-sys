#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]
#![allow(missing_docs)]
#![warn(unused_extern_crates)]

pub mod bullet {
    include!("bullet.rs");
}

pub mod bullet3 {
    include!("bullet3.rs");
}

#[cfg(test)]
mod tests {
    use super::bullet::*;

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
}
