
#![deny(trivial_casts)]

//! Bindings to the X-Plane plugin SDK
//!

extern crate xplm_sys;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate lazy_static;

#[cfg(all(target_os = "macos", not(feature = "xplm210")))]
extern crate hfs_paths;

use std::ffi::CString;

/// FFI utilities
mod ffi;
/// Plugin macro
mod plugin_macro;
/// Path conversion
mod paths;

#[doc(hidden)]
/// Utilities that the xplane_plugin macro-generated code uses
///
// These must be public so that code in other crates can access them
pub mod internal;

/// Plugin creation and management
pub mod plugin;
/// Flight loop callbacks
#[cfg(feature = "xplm210")]
// TODO: Flight loop implementation that supports SDK 1.0
pub mod flight_loop;
/// Commands
#[cfg(feature = "xplm200")]
pub mod command;
/// Datarefs
pub mod data;
/// Error detection
#[cfg(feature = "xplm200")]
pub mod error;
/// SDK feature management
#[cfg(feature = "xplm200")]
pub mod feature;
/// User interface menus
#[cfg(feature = "xplm200")]
pub mod menu;
/// Low-level drawing callbacks
pub mod draw;
/// Relatively low-level windows
pub mod window;
/// 2D user interface geometry
pub mod geometry;

/// Writes a message to the X-Plane log.txt file
///
/// No line terminator is added.
pub fn debug<S: Into<String>>(message: S) {
    use xplm_sys::XPLMDebugString;
    match CString::new(message.into()) {
        Ok(message_c) => unsafe { XPLMDebugString(message_c.as_ptr()) },
        Err(_) => unsafe {
            XPLMDebugString(b"[xplm] Invalid debug message\n\0".as_ptr() as *const _)
        },
    }
}

/// Attempts to locate a symbol. If it exists, returns a pointer to it
pub fn find_symbol<S: Into<String>>(name: S) -> *mut std::os::raw::c_void {
    use std::ptr;
    match std::ffi::CString::new(name.into()) {
        Ok(name_c) => {
            unsafe {
                xplm_sys::XPLMFindSymbol(name_c.as_ptr())
            }
        }
        Err(_) => ptr::null_mut()
    }
}
