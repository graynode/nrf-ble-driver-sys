use nrf_ble_driver_sys::ffi;
use std::ffi::CString;

const DEFAULT_BUADRATE: u32 = 1_000_000;

fn create_adapter(port_name: &str) -> *mut ffi::adapter_t {
    let port = CString::new(port_name).unwrap();
    unsafe {
        let phy = ffi::sd_rpc_physical_layer_create_uart(
            port.as_ptr(),
            DEFAULT_BUADRATE,
            ffi::sd_rpc_flow_control_t_SD_RPC_FLOW_CONTROL_NONE,
            ffi::sd_rpc_parity_t_SD_RPC_PARITY_NONE,
        );

        if phy.is_null() {
            panic!("phy is null");
        }

        let link_layer = ffi::sd_rpc_data_link_layer_create_bt_three_wire(phy, 250);
        if link_layer.is_null() {
            panic!("link layer is null");
        }

        let transport_layer = ffi::sd_rpc_transport_layer_create(link_layer, 1500);
        if transport_layer.is_null() {
            panic!("transport layer is null");
        }

        let adapter = ffi::sd_rpc_adapter_create(transport_layer);
        if adapter.is_null() {
            panic!("Adapter is null");
        }

        return adapter;
    }
}

fn main() {
    let adapter = create_adapter("/dev/ttyACM0");
}
