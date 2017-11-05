// This file was generated by gir (94e079d) from gir-files (???)
// DO NOT EDIT

use Error;
use Object;
use Structure;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Plugin(Object<ffi::GstPlugin>): Object;

    match fn {
        get_type => || ffi::gst_plugin_get_type(),
    }
}

impl Plugin {
    //pub fn add_dependency<'a, 'b, 'c, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>, R: Into<Option<&'c str>>>(&self, env_vars: P, paths: Q, names: R, flags: /*Ignored*/PluginDependencyFlags) {
    //    unsafe { TODO: call ffi::gst_plugin_add_dependency() }
    //}

    //pub fn add_dependency_simple<'a, 'b, 'c, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>, R: Into<Option<&'c str>>>(&self, env_vars: P, paths: Q, names: R, flags: /*Ignored*/PluginDependencyFlags) {
    //    unsafe { TODO: call ffi::gst_plugin_add_dependency_simple() }
    //}

    pub fn get_cache_data(&self) -> Option<Structure> {
        unsafe {
            from_glib_none(ffi::gst_plugin_get_cache_data(self.to_glib_none().0))
        }
    }

    pub fn get_description(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_plugin_get_description(self.to_glib_none().0))
        }
    }

    pub fn get_filename(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_plugin_get_filename(self.to_glib_none().0))
        }
    }

    pub fn get_license(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_plugin_get_license(self.to_glib_none().0))
        }
    }

    pub fn get_origin(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_plugin_get_origin(self.to_glib_none().0))
        }
    }

    pub fn get_package(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_plugin_get_package(self.to_glib_none().0))
        }
    }

    pub fn get_release_date_string(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_plugin_get_release_date_string(self.to_glib_none().0))
        }
    }

    pub fn get_source(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_plugin_get_source(self.to_glib_none().0))
        }
    }

    pub fn get_version(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_plugin_get_version(self.to_glib_none().0))
        }
    }

    pub fn is_loaded(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_plugin_is_loaded(self.to_glib_none().0))
        }
    }

    pub fn load(&self) -> Option<Plugin> {
        unsafe {
            from_glib_full(ffi::gst_plugin_load(self.to_glib_none().0))
        }
    }

    pub fn set_cache_data(&self, cache_data: &mut Structure) {
        unsafe {
            ffi::gst_plugin_set_cache_data(self.to_glib_none().0, cache_data.to_glib_full());
        }
    }

    pub fn list_free(list: &[Plugin]) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_plugin_list_free(list.to_glib_full());
        }
    }

    pub fn load_by_name(name: &str) -> Option<Plugin> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_plugin_load_by_name(name.to_glib_none().0))
        }
    }

    pub fn load_file(filename: &str) -> Result<Plugin, Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gst_plugin_load_file(filename.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //pub fn register_static(major_version: i32, minor_version: i32, name: &str, description: &str, init_func: /*Unknown conversion*//*Unimplemented*/PluginInitFunc, version: &str, license: &str, source: &str, package: &str, origin: &str) -> bool {
    //    unsafe { TODO: call ffi::gst_plugin_register_static() }
    //}

    //pub fn register_static_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(major_version: i32, minor_version: i32, name: &str, description: &str, init_full_func: /*Unknown conversion*//*Unimplemented*/PluginInitFullFunc, version: &str, license: &str, source: &str, package: &str, origin: &str, user_data: P) -> bool {
    //    unsafe { TODO: call ffi::gst_plugin_register_static_full() }
    //}
}

unsafe impl Send for Plugin {}
unsafe impl Sync for Plugin {}
