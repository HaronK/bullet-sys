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
            .enable_cxx_namespaces()
            .layout_tests(false)
            .clang_arg(r"-v")
            .clang_arg(r"-xc++")
            .clang_arg(r"-std=c++11")
            .clang_arg(r"-Ibullet3/src")
            .clang_arg(r"-DBT_NO_SIMD_OPERATOR_OVERLOADS")
            .header(r"bullet3.h")
            .whitelist_type(r"bt.+")
            .whitelist_function(r"bt.+")
            .whitelist_var(r"bt.+")
            .generate()
            .expect("Unable to generate bindings");

        let out_path = PathBuf::from("src").join(format!("bullet3_{}.rs", get_os_name()));
        bindings
            .write_to_file(out_path)
            .expect("Couldn't write bindings!");
    }
}

fn get_os_name() -> String {
    if cfg!(target_os = "linux") {
        "linux".to_owned()
    } else if cfg!(target_os = "windows") {
        "windows".to_owned()
    } else {
        panic!("Unsupported OS!");
    }
}

fn cmake_build() {
    let run_tests = cfg!(feature = "tests");
    let build_tests = if run_tests { "ON" } else { "OFF" };

    // NOTE: unit tests depend on some libraries in examples so we have to build them as well
    let build_examples = if run_tests || cfg!(feature = "examples") {
        "ON"
    } else {
        "OFF"
    };

    let (tests_path, libs_path) = if cfg!(target_os = "linux") {
        cmake_build_linux(build_tests, build_examples)
    } else if cfg!(target_os = "windows") {
        cmake_build_windows(build_tests, build_examples)
    } else {
        panic!("Unsupported OS!");
    };

    if run_tests {
        let _ = Command::new("ctest")
            .current_dir(tests_path)
            .spawn()
            .unwrap();
    }

    println!("cargo:rustc-link-search=native={}", libs_path.display());

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

fn cmake_build_linux(build_tests: &str, build_examples: &str) -> (PathBuf, PathBuf) {
    let dst = Config::new("bullet3")
        .define("BUILD_PYBULLET", "OFF")
        .define("BUILD_PYBULLET_NUMPY", "OFF")
        .define("BUILD_UNIT_TESTS", build_tests)
        .define("BUILD_CPU_DEMOS", build_examples)
        .define("BUILD_BULLET2_DEMOS", build_examples)
        .define("USE_GRAPHICAL_BENCHMARK", "OFF")
        .define("USE_DOUBLE_PRECISION", "ON")
        .build();

    (dst.join("build"), dst.join("lib"))
}

fn cmake_build_windows(build_tests: &str, build_examples: &str) -> (PathBuf, PathBuf) {
    let dst = Config::new("bullet3_wrapper")
        .define("BUILD_PYBULLET", "OFF")
        .define("BUILD_PYBULLET_NUMPY", "OFF")
        .define("CMAKE_DEBUG_POSTFIX", "")
        .define("CMAKE_RELEASE_POSTFIX", "")
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
        .build();

    let libs_path = if cfg!(debug_assertions) {
        r"build\lib\Debug"
    } else {
        r"build\lib\Release"
    };

    (dst.join("bullet3"), dst.join(libs_path))
}

// trait BindgenWitelister {
//     fn whitelist_types<T: AsRef<Path>>(self, arg: T) -> Self;
//     fn whitelist_functions<T: AsRef<Path>>(self, arg: T) -> Self;
// }