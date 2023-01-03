// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{Object, Pad};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstProxyPad")]
    pub struct ProxyPad(Object<ffi::GstProxyPad, ffi::GstProxyPadClass>) @extends Pad, Object;

    match fn {
        type_ => || ffi::gst_proxy_pad_get_type(),
    }
}

impl ProxyPad {
    pub const NONE: Option<&'static ProxyPad> = None;

    //#[doc(alias = "gst_proxy_pad_iterate_internal_links_default")]
    //pub fn iterate_internal_links_default(pad: &impl IsA<Pad>, parent: Option<&impl IsA<Object>>) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi:gst_proxy_pad_iterate_internal_links_default() }
    //}
}

unsafe impl Send for ProxyPad {}
unsafe impl Sync for ProxyPad {}

pub trait ProxyPadExt: 'static {
    #[doc(alias = "gst_proxy_pad_get_internal")]
    #[doc(alias = "get_internal")]
    #[must_use]
    fn internal(&self) -> Option<ProxyPad>;
}

impl<O: IsA<ProxyPad>> ProxyPadExt for O {
    fn internal(&self) -> Option<ProxyPad> {
        unsafe {
            from_glib_full(ffi::gst_proxy_pad_get_internal(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}
