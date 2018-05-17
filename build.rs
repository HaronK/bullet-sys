extern crate bindgen;
extern crate cmake;

use cmake::Config;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if cfg!(feature = "build") {
        cmake_build();
    }

    if cfg!(feature = "bind") {
        let bindings = bindgen::Builder::default()
            .clang_arg("-v")
            .clang_arg("-x")
            .clang_arg("c++")
            .clang_arg("-Ibullet3/src")
            .header("bullet3.h")
            .generate()
            .expect("Unable to generate bindings");

        let out_path = PathBuf::from("src/");
        bindings
            .write_to_file(out_path.join("bullet3.rs"))
            .expect("Couldn't write bindings!");
    }
}

fn cmake_build() {
        let run_tests = cfg!(feature = "tests");
        let build_tests = if run_tests { "ON" } else { "OFF" };

        // NOTE: unit testsdepend on some libraries in examples so we have to build them as well
        let build_examples = if run_tests || cfg!(feature = "examples") {
            "ON"
        } else {
            "OFF"
        };

    let dst = if cfg!(target_os = "linux") {
        cmake_build_linux(build_tests, build_examples)
    } else if cfg!(target_os = "windows") {
        cmake_build_windows(build_tests, build_examples)
    } else {
        panic!("Unsupported OS!");
    };

    if run_tests {
        let _ = Command::new("ctest")
            .current_dir(dst.join("build"))
            .spawn()
            .unwrap();
    }

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );

    let libs = vec![
        "Bullet2FileLoader",
        "Bullet3Collision",
        "Bullet3Common",
        "Bullet3Dynamics",
        "Bullet3Geometry",
        "Bullet3OpenCL_clew",
        "BulletCollision",
        "BulletDynamics",
        "BulletFileLoader",
        "BulletInverseDynamics",
        "BulletInverseDynamicsUtils",
        "BulletRobotics",
        "BulletSoftBody",
        "BulletWorldImporter",
        "BulletXmlWorldImporter",
        "ConvexDecomposition",
        "GIMPACTUtils",
        "HACD",
        "LinearMath",
    ];

    for lib in libs {
        println!("cargo:rustc-link-lib=static={}", lib);
    }
}

fn cmake_build_linux(build_tests: &str, build_examples: &str) -> PathBuf {
    Config::new("bullet3")
        .define("BUILD_PYBULLET", "OFF")
        .define("BUILD_PYBULLET_NUMPY", "OFF")
        .define("BUILD_UNIT_TESTS", build_tests)
        .define("BUILD_CPU_DEMOS", build_examples)
        .define("BUILD_BULLET2_DEMOS", build_examples)
        .define("USE_GRAPHICAL_BENCHMARK", "OFF")
        .define("USE_DOUBLE_PRECISION", "ON")
        .build()
}

fn cmake_build_windows(build_tests: &str, build_examples: &str) -> PathBuf {
    Config::new("bullet3_wrapper")
        .define("BUILD_PYBULLET", "OFF")
        .define("BUILD_PYBULLET_NUMPY", "OFF")
        .define("BUILD_UNIT_TESTS", build_tests)
        .define("BUILD_CPU_DEMOS", build_examples)
        .define("BUILD_BULLET2_DEMOS", build_examples)
        .define("USE_GRAPHICAL_BENCHMARK", "OFF")
        .define("USE_DOUBLE_PRECISION", "ON")
        .define("CMAKE_C_FLAGS_DEBUG", " /nologo /MDd")
        .define("CMAKE_CXX_FLAGS_DEBUG", " /nologo /MDd")
        //.define("WIN32", "ON")
        //.target("x86_64-pc-windows-gnu")
        .generator("Visual Studio 14 2015 Win64")
        .build()
}
