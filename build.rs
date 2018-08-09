extern crate bindgen;
extern crate cmake;

use cmake::Config;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    if cfg!(feature = "build") {
        cmake_build();
    }

    bind("bullet", "bt.+");
    bind("bullet3", "b3.+|cl_.+|clew.+");
}

fn cmake_build() {
    let run_tests = cfg!(feature = "ctest");
    let build_tests = if run_tests { "ON" } else { "OFF" };

    // NOTE: unit tests depend on some libraries in examples so we have to build them as well
    let build_examples = if run_tests || cfg!(feature = "examples") {
        "ON"
    } else {
        "OFF"
    };

    let (tests_path, libs_path) = cmake_build_target(build_tests, build_examples);

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

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=stdc++");
    }
}

#[cfg(target_os = "linux")]
fn cmake_build_target(build_tests: &str, build_examples: &str) -> (PathBuf, PathBuf) {
    let dst = Config::new("bullet")
        .define("BUILD_PYBULLET", "OFF")
        .define("BUILD_PYBULLET_NUMPY", "OFF")
        .define("BUILD_UNIT_TESTS", build_tests)
        .define("BUILD_CPU_DEMOS", build_examples)
        .define("BUILD_BULLET2_DEMOS", build_examples)
        .define("USE_GRAPHICAL_BENCHMARK", "OFF")
        .define("USE_DOUBLE_PRECISION", "ON")
        .cxxflag("-fkeep-inline-functions")
        //.env("VERBOSE", "1")
        .build();

    (dst.join("build"), dst.join("lib"))
}

#[cfg(target_os = "windows")]
fn cmake_build_target(build_tests: &str, build_examples: &str) -> (PathBuf, PathBuf) {
    let is_debug = cfg!(debug_assertions);
    let dst = Config::new("bullet_wrapper")
        .define("CMAKE_BUILD_TYPE", if is_debug { "Debug" } else { "Release" })
        .define("BUILD_PYBULLET", "OFF")
        .define("BUILD_PYBULLET_NUMPY", "OFF")
        .define("CMAKE_DEBUG_POSTFIX", "")
        .define("CMAKE_RELEASE_POSTFIX", "")
        .define("BUILD_UNIT_TESTS", build_tests)
        .define("BUILD_CPU_DEMOS", build_examples)
        .define("BUILD_BULLET2_DEMOS", build_examples)
        .define("USE_GRAPHICAL_BENCHMARK", "OFF")
        .define("USE_DOUBLE_PRECISION", "ON")
        .define("CMAKE_C_FLAGS_DEBUG", " /nologo /MTd")
        .define("CMAKE_CXX_FLAGS_DEBUG", " /nologo /MTd")
        .cxxflag("-fkeep-inline-functions")
        .env("VERBOSE", "1")
        //.define("WIN32", "ON")
        //.target("x86_64-pc-windows-gnu")
        .generator("Visual Studio 14 2015 Win64")
        .build();

    let libs_path = if is_debug {
        r"build\lib\Debug"
    } else {
        r"build\lib\Release"
    };

    (dst.join("bullet"), dst.join(libs_path))
}

fn bind(prefix: &str, templ: &str) {
    let out_path = Path::new("src").join(format!("{}.rs", prefix));

    if out_path.exists() && !cfg!(feature = "bind") {
        return;
    }

    let bindings = bindgen::Builder::default()
        .layout_tests(cfg!(feature = "layout_tests"))
        .clang_arg(r"-v")
        .clang_arg(r"-xc++")
        .clang_arg(r"-std=c++14")
        .clang_arg(r"-Ibullet/src")
        .clang_arg(r"-DBT_NO_SIMD_OPERATOR_OVERLOADS")
        .clang_arg(r"-DBT_ENABLE_CLSOCKET")
        .clang_arg(r"-DBT_ENABLE_ENET")
        .clang_arg(r"-DBT_USE_DOUBLE_PRECISION")
        .clang_arg(r"-DHAS_SOCKLEN_T")
        .clang_arg(r"-DPHYSICS_SERVER_DIRECT")
        .clang_arg(r"-DB3_USE_CLEW")
        .generate_inline_functions(true)
        .header(format!("{}.h", prefix))
        .whitelist_type(templ)
        .whitelist_function(templ)
        .whitelist_var(templ)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path)
        .expect(&format!("Couldn't write bindings for {}!", prefix));
}
