// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::GString;
use gst_pbutils_sys;
use std::mem;
use EncodingTarget;

pub fn encoding_list_all_targets(categoryname: Option<&str>) -> Vec<EncodingTarget> {
    assert_initialized_main_thread!();
    unsafe {
        FromGlibPtrContainer::from_glib_full(gst_pbutils_sys::gst_encoding_list_all_targets(
            categoryname.to_glib_none().0,
        ))
    }
}

pub fn encoding_list_available_categories() -> Vec<GString> {
    assert_initialized_main_thread!();
    unsafe {
        FromGlibPtrContainer::from_glib_full(
            gst_pbutils_sys::gst_encoding_list_available_categories(),
        )
    }
}

pub fn pb_utils_get_element_description(factory_name: &str) -> Option<GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(gst_pbutils_sys::gst_pb_utils_get_element_description(
            factory_name.to_glib_none().0,
        ))
    }
}

pub fn pb_utils_get_sink_description(protocol: &str) -> Option<GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(gst_pbutils_sys::gst_pb_utils_get_sink_description(
            protocol.to_glib_none().0,
        ))
    }
}

pub fn pb_utils_get_source_description(protocol: &str) -> Option<GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(gst_pbutils_sys::gst_pb_utils_get_source_description(
            protocol.to_glib_none().0,
        ))
    }
}

pub fn plugins_base_version() -> (u32, u32, u32, u32) {
    assert_initialized_main_thread!();
    unsafe {
        let mut major = mem::uninitialized();
        let mut minor = mem::uninitialized();
        let mut micro = mem::uninitialized();
        let mut nano = mem::uninitialized();
        gst_pbutils_sys::gst_plugins_base_version(&mut major, &mut minor, &mut micro, &mut nano);
        (major, minor, micro, nano)
    }
}

pub fn plugins_base_version_string() -> Option<GString> {
    assert_initialized_main_thread!();
    unsafe { from_glib_full(gst_pbutils_sys::gst_plugins_base_version_string()) }
}
