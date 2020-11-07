// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gst_sys;
use std;
use std::boxed::Box as Box_;
use std::mem::transmute;
use Object;
use Plugin;
use PluginFeature;

glib_wrapper! {
    pub struct Registry(Object<gst_sys::GstRegistry, gst_sys::GstRegistryClass>) @extends Object;

    match fn {
        get_type => || gst_sys::gst_registry_get_type(),
    }
}

impl Registry {
    pub fn add_feature<P: IsA<PluginFeature>>(
        &self,
        feature: &P,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_registry_add_feature(
                    self.to_glib_none().0,
                    feature.as_ref().to_glib_none().0
                ),
                "Failed to add feature"
            )
        }
    }

    pub fn add_plugin(&self, plugin: &Plugin) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_registry_add_plugin(self.to_glib_none().0, plugin.to_glib_none().0),
                "Failed to add plugin"
            )
        }
    }

    pub fn check_feature_version(
        &self,
        feature_name: &str,
        min_major: u32,
        min_minor: u32,
        min_micro: u32,
    ) -> bool {
        unsafe {
            from_glib(gst_sys::gst_registry_check_feature_version(
                self.to_glib_none().0,
                feature_name.to_glib_none().0,
                min_major,
                min_minor,
                min_micro,
            ))
        }
    }

    pub fn feature_filter<P: FnMut(&PluginFeature) -> bool>(
        &self,
        filter: P,
        first: bool,
    ) -> Vec<PluginFeature> {
        let filter_data: P = filter;
        unsafe extern "C" fn filter_func<P: FnMut(&PluginFeature) -> bool>(
            feature: *mut gst_sys::GstPluginFeature,
            user_data: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let feature = from_glib_borrow(feature);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            let res = (*callback)(&feature);
            res.to_glib()
        }
        let filter = Some(filter_func::<P> as _);
        let super_callback0: &P = &filter_data;
        unsafe {
            FromGlibPtrContainer::from_glib_full(gst_sys::gst_registry_feature_filter(
                self.to_glib_none().0,
                filter,
                first.to_glib(),
                super_callback0 as *const _ as usize as *mut _,
            ))
        }
    }

    pub fn find_feature(&self, name: &str, type_: glib::types::Type) -> Option<PluginFeature> {
        unsafe {
            from_glib_full(gst_sys::gst_registry_find_feature(
                self.to_glib_none().0,
                name.to_glib_none().0,
                type_.to_glib(),
            ))
        }
    }

    pub fn find_plugin(&self, name: &str) -> Option<Plugin> {
        unsafe {
            from_glib_full(gst_sys::gst_registry_find_plugin(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    pub fn get_feature_list(&self, type_: glib::types::Type) -> Vec<PluginFeature> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gst_sys::gst_registry_get_feature_list(
                self.to_glib_none().0,
                type_.to_glib(),
            ))
        }
    }

    pub fn get_feature_list_by_plugin(&self, name: &str) -> Vec<PluginFeature> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gst_sys::gst_registry_get_feature_list_by_plugin(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    pub fn get_feature_list_cookie(&self) -> u32 {
        unsafe { gst_sys::gst_registry_get_feature_list_cookie(self.to_glib_none().0) }
    }

    pub fn get_plugin_list(&self) -> Vec<Plugin> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gst_sys::gst_registry_get_plugin_list(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn lookup(&self, filename: &str) -> Option<Plugin> {
        unsafe {
            from_glib_full(gst_sys::gst_registry_lookup(
                self.to_glib_none().0,
                filename.to_glib_none().0,
            ))
        }
    }

    pub fn lookup_feature(&self, name: &str) -> Option<PluginFeature> {
        unsafe {
            from_glib_full(gst_sys::gst_registry_lookup_feature(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    pub fn plugin_filter<P: FnMut(&Plugin) -> bool>(&self, filter: P, first: bool) -> Vec<Plugin> {
        let filter_data: P = filter;
        unsafe extern "C" fn filter_func<P: FnMut(&Plugin) -> bool>(
            plugin: *mut gst_sys::GstPlugin,
            user_data: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let plugin = from_glib_borrow(plugin);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            let res = (*callback)(&plugin);
            res.to_glib()
        }
        let filter = Some(filter_func::<P> as _);
        let super_callback0: &P = &filter_data;
        unsafe {
            FromGlibPtrContainer::from_glib_full(gst_sys::gst_registry_plugin_filter(
                self.to_glib_none().0,
                filter,
                first.to_glib(),
                super_callback0 as *const _ as usize as *mut _,
            ))
        }
    }

    pub fn remove_feature<P: IsA<PluginFeature>>(&self, feature: &P) {
        unsafe {
            gst_sys::gst_registry_remove_feature(
                self.to_glib_none().0,
                feature.as_ref().to_glib_none().0,
            );
        }
    }

    pub fn remove_plugin(&self, plugin: &Plugin) {
        unsafe {
            gst_sys::gst_registry_remove_plugin(self.to_glib_none().0, plugin.to_glib_none().0);
        }
    }

    pub fn scan_path<P: AsRef<std::path::Path>>(&self, path: P) -> bool {
        unsafe {
            from_glib(gst_sys::gst_registry_scan_path(
                self.to_glib_none().0,
                path.as_ref().to_glib_none().0,
            ))
        }
    }

    pub fn get() -> Registry {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(gst_sys::gst_registry_get()) }
    }

    pub fn connect_feature_added<F: Fn(&Registry, &PluginFeature) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn feature_added_trampoline<
            F: Fn(&Registry, &PluginFeature) + Send + Sync + 'static,
        >(
            this: *mut gst_sys::GstRegistry,
            feature: *mut gst_sys::GstPluginFeature,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(feature))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"feature-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    feature_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_plugin_added<F: Fn(&Registry, &Plugin) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn plugin_added_trampoline<
            F: Fn(&Registry, &Plugin) + Send + Sync + 'static,
        >(
            this: *mut gst_sys::GstRegistry,
            plugin: *mut gst_sys::GstPlugin,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(plugin))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"plugin-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    plugin_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for Registry {}
unsafe impl Sync for Registry {}
