use crate::pentane::log_message;
use pentane::{PentaneSemVer, PentaneUUID, PluginInformation};
use sunset_rs::*;
use log::{info, warn};

mod pentane;

#[unsafe(no_mangle)]
#[used]
pub static Pentane_PluginInformation: PluginInformation = PluginInformation::new(
    b"Cars 2 Rich Presence",
    b"placeholder",
    PentaneUUID::from_str("ad7d9a00c72611f08de90242ac120002"),
    PentaneSemVer::new(0, 1, 0),
    PentaneSemVer::new(1, 0, 0),
);

#[unsafe(no_mangle)]
#[used]
pub static Pentane_PluginDependencyCount: usize = 0;
// Since we don't depend on any other plugins, we can skip exporting a `Pentane_PluginDependencies`.

// #[sunset_rs::hook(offset = 0x004c0440)]
// pub extern "thiscall" fn carsfrontend_setlevel_hook(this: *mut (), level: *const std::os::raw::c_char) {
//     let level_str = unsafe { std::ffi::CStr::from_ptr(level) };
     // .. do stuff with 
// }

#[unsafe(no_mangle)]
extern "C" fn Pentane_Main() {
    info!("[RichPresence]: god fucking dammit god fucking dammit god fu");
}