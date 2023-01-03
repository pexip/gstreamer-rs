// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ClockTime, Object};
use glib::{prelude::*, translate::*};
use std::mem;

glib::wrapper! {
    #[doc(alias = "GstControlSource")]
    pub struct ControlSource(Object<ffi::GstControlSource, ffi::GstControlSourceClass>) @extends Object;

    match fn {
        type_ => || ffi::gst_control_source_get_type(),
    }
}

impl ControlSource {
    pub const NONE: Option<&'static ControlSource> = None;
}

unsafe impl Send for ControlSource {}
unsafe impl Sync for ControlSource {}

pub trait ControlSourceExt: 'static {
    #[doc(alias = "gst_control_source_get_value")]
    #[doc(alias = "control_source_get_value")]
    fn value(&self, timestamp: ClockTime) -> Option<f64>;
}

impl<O: IsA<ControlSource>> ControlSourceExt for O {
    fn value(&self, timestamp: ClockTime) -> Option<f64> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_control_source_get_value(
                self.as_ref().to_glib_none().0,
                timestamp.into_glib(),
                value.as_mut_ptr(),
            ));
            if ret {
                Some(value.assume_init())
            } else {
                None
            }
        }
    }
}
