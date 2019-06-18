// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gst_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;
use Caps;
use Element;
use Object;
use Structure;

glib_wrapper! {
    pub struct Device(Object<gst_sys::GstDevice, gst_sys::GstDeviceClass, DeviceClass>) @extends Object;

    match fn {
        get_type => || gst_sys::gst_device_get_type(),
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

pub const NONE_DEVICE: Option<&Device> = None;

pub trait DeviceExt: 'static {
    fn create_element(&self, name: Option<&str>) -> Option<Element>;

    fn get_caps(&self) -> Option<Caps>;

    fn get_device_class(&self) -> GString;

    fn get_display_name(&self) -> GString;

    fn get_properties(&self) -> Option<Structure>;

    fn has_classes(&self, classes: &str) -> bool;

    fn has_classesv(&self, classes: &[&str]) -> bool;

    fn reconfigure_element<P: IsA<Element>>(
        &self,
        element: &P,
    ) -> Result<(), glib::error::BoolError>;

    fn connect_removed<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Device>> DeviceExt for O {
    fn create_element(&self, name: Option<&str>) -> Option<Element> {
        unsafe {
            from_glib_full(gst_sys::gst_device_create_element(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn get_caps(&self) -> Option<Caps> {
        unsafe { from_glib_full(gst_sys::gst_device_get_caps(self.as_ref().to_glib_none().0)) }
    }

    fn get_device_class(&self) -> GString {
        unsafe {
            from_glib_full(gst_sys::gst_device_get_device_class(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_display_name(&self) -> GString {
        unsafe {
            from_glib_full(gst_sys::gst_device_get_display_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_properties(&self) -> Option<Structure> {
        unsafe {
            from_glib_full(gst_sys::gst_device_get_properties(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_classes(&self, classes: &str) -> bool {
        unsafe {
            from_glib(gst_sys::gst_device_has_classes(
                self.as_ref().to_glib_none().0,
                classes.to_glib_none().0,
            ))
        }
    }

    fn has_classesv(&self, classes: &[&str]) -> bool {
        unsafe {
            from_glib(gst_sys::gst_device_has_classesv(
                self.as_ref().to_glib_none().0,
                classes.to_glib_none().0,
            ))
        }
    }

    fn reconfigure_element<P: IsA<Element>>(
        &self,
        element: &P,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_device_reconfigure_element(
                    self.as_ref().to_glib_none().0,
                    element.as_ref().to_glib_none().0
                ),
                "Failed to reconfigure the element to use this device"
            )
        }
    }

    fn connect_removed<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn removed_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_sys::GstDevice,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Device>,
        {
            let f: &F = &*(f as *const F);
            f(&Device::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"removed\0".as_ptr() as *const _,
                Some(transmute(removed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}
