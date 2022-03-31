extern crate vcpkg;
use cmake::Config;
use std::env;

const NRF_BLE_RELEASE_VERSION: &str = "4.1.4";
#[cfg(any(target_os = "linux", target_os = "macos"))]
const NRF_BLE_DRIVER_LIB_NAME: &str = "nrf-ble-driver-sd_api_v6";
#[cfg(target_os = "windows")]
#[cfg(not(debug_assertions))]
const NRF_BLE_DRIVER_LIB_NAME: &str = "nrf-ble-driver-sd_api_v6-mt-static-4_1_4";
#[cfg(target_os = "windows")]
#[cfg(debug_assertions)]
const NRF_BLE_DRIVER_LIB_NAME: &str = "nrf-ble-driver-sd_api_v6-mt-static-gd-4_1_4";



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
    let dest = Config::new("src/pc-ble-driver")
    .define("NRF_BLE_DRIVER_VERSION", NRF_BLE_RELEASE_VERSION)
    .build();

    let lib_folder = dest.join("lib");
    println!("cargo:warning=dst: {}", dest.display());
    println!("cargo:rustc-link-search=native={}", lib_folder.display());
    println!("cargo:rustc-link-lib=static={}", NRF_BLE_DRIVER_LIB_NAME);
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
