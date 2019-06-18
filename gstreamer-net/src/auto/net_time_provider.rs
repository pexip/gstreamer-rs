// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gst;
use gst_net_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct NetTimeProvider(Object<gst_net_sys::GstNetTimeProvider, gst_net_sys::GstNetTimeProviderClass, NetTimeProviderClass>) @extends gst::Object;

    match fn {
        get_type => || gst_net_sys::gst_net_time_provider_get_type(),
    }
}

impl NetTimeProvider {
    pub fn get_property_active(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"active\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    pub fn set_property_active(&self, active: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"active\0".as_ptr() as *const _,
                Value::from(&active).to_glib_none().0,
            );
        }
    }

    pub fn get_property_address(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"address\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    pub fn get_property_clock(&self) -> Option<gst::Clock> {
        unsafe {
            let mut value = Value::from_type(<gst::Clock as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"clock\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    pub fn get_property_port(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"port\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    pub fn get_property_qos_dscp(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"qos-dscp\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    pub fn set_property_qos_dscp(&self, qos_dscp: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"qos-dscp\0".as_ptr() as *const _,
                Value::from(&qos_dscp).to_glib_none().0,
            );
        }
    }

    pub fn connect_property_active_notify<F: Fn(&NetTimeProvider) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<
            F: Fn(&NetTimeProvider) + Send + Sync + 'static,
        >(
            this: *mut gst_net_sys::GstNetTimeProvider,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active\0".as_ptr() as *const _,
                Some(transmute(notify_active_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_qos_dscp_notify<F: Fn(&NetTimeProvider) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_qos_dscp_trampoline<
            F: Fn(&NetTimeProvider) + Send + Sync + 'static,
        >(
            this: *mut gst_net_sys::GstNetTimeProvider,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::qos-dscp\0".as_ptr() as *const _,
                Some(transmute(notify_qos_dscp_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for NetTimeProvider {}
unsafe impl Sync for NetTimeProvider {}
