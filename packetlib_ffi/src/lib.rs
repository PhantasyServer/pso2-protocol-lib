pub mod protocol;
// TODO: probably add connections and ppak

pub const API_VERSION: u32 = 1;
pub const PROTOCOL_VERSION: u32 = 1;

#[no_mangle]
pub extern "C" fn get_api_version() -> u32 {
    API_VERSION
}

#[no_mangle]
pub extern "C" fn get_protocol_version() -> u32 {
    PROTOCOL_VERSION
}
