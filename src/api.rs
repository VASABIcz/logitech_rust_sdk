use std::net::Shutdown;
use std::os::raw::c_int;

pub const LOGI_DEVICETYPE_MONOCHROME_ORD: i32 = 0;
pub const LOGI_DEVICETYPE_RGB_ORD: i32 = 1;
pub const LOGI_DEVICETYPE_PERKEY_RGB_ORD: i32 = 2;

pub const LOGI_DEVICETYPE_MONOCHROME: i32 = 1 << LOGI_DEVICETYPE_MONOCHROME_ORD;
pub const LOGI_DEVICETYPE_RGB: i32 = 1 << LOGI_DEVICETYPE_RGB_ORD;
pub const LOGI_DEVICETYPE_PERKEY_RGB: i32 = 1 << LOGI_DEVICETYPE_PERKEY_RGB_ORD;
pub const LOGI_DEVICETYPE_ALL: i32 = LOGI_DEVICETYPE_MONOCHROME | LOGI_DEVICETYPE_RGB | LOGI_DEVICETYPE_PERKEY_RGB;

#[cfg_attr(all(target_os = "windows", target_env = "msvc"), link(name=r"./src/lib/LogitechLEDLib", kind="static"))]
extern "cdecl" {
    #[link_name = "?LogiLedInit@@YA_NXZ"]
    fn Init() -> bool;
    #[link_name = "?LogiLedSetTargetDevice@@YA_NH@Z"]
    fn SetTargetDevice(device: c_int) -> bool;
    #[link_name = "?LogiLedSetLighting@@YA_NHHH@Z"]
    fn setLightning(r: c_int, g: c_int, b: c_int) -> bool;
    #[link_name = "?LogiLedShutdown@@YAXXZ"]
    fn Shutdown();
}


pub fn init() -> bool {
    unsafe { Init() }
}

pub fn shutdown() {
    unsafe { Shutdown() }
}
pub fn set_lightning(r: i32, g: i32, b: i32) -> bool {
    unsafe { setLightning(r, g, b)}
}
pub fn set_device(device: i32) -> bool {
    unsafe { SetTargetDevice(device) }
}