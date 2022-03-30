extern crate vcpkg;
use cmake;
use std::env;

fn find_dependencies() -> bool {
    env::var("VCPKG_ROOT").unwrap();

    let mut config = vcpkg::Config::new();
    if config.find_package("asio").is_err() {
        println!("cargo:warning=vcpkg: error finding asio");
        return false;
    }

    if config.find_package("catch2").is_err() {
        println!("cargo:warning=vcpkg: error finding catch2");
        return false;
    }

    if config.find_package("spdlog").is_err() {
        println!("cargo:warning=vcpkg: error finding spdlog");
        return false;
    }

    return true;
}

fn build_sources() -> bool {
    let dest = cmake::build("src/pc-ble-driver");
    let lib_folder = dest.join("lib");
    println!("cargo:warning=dst: {}", dest.display());
    println!("cargo:rustc-link-search=native={}", lib_folder.display());
    println!("cargo:rustc-link-lib=nrf-ble-driver-sd_api_v6");
    return true;

}

fn main() {
    if !find_dependencies() {
        println!("cargo:warning=Error finding dependencies");
        return;
    } else {
        println!("cargo:warning=Found dependencies");
    }

    if build_sources() {
        println!("built sources");
    }
}
