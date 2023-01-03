// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use crate::FrameNumber;
use crate::{Asset, MetaContainer, TrackType};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use std::mem;
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GESClipAsset")]
    pub struct ClipAsset(Object<ffi::GESClipAsset, ffi::GESClipAssetClass>) @extends Asset, @implements MetaContainer;

    match fn {
        type_ => || ffi::ges_clip_asset_get_type(),
    }
}

impl ClipAsset {
    pub const NONE: Option<&'static ClipAsset> = None;
}

pub trait ClipAssetExt: 'static {
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_asset_get_frame_time")]
    #[doc(alias = "get_frame_time")]
    fn frame_time(&self, frame_number: FrameNumber) -> Option<gst::ClockTime>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_asset_get_natural_framerate")]
    #[doc(alias = "get_natural_framerate")]
    fn natural_framerate(&self) -> Option<(i32, i32)>;

    #[doc(alias = "ges_clip_asset_get_supported_formats")]
    #[doc(alias = "get_supported_formats")]
    fn supported_formats(&self) -> TrackType;

    #[doc(alias = "ges_clip_asset_set_supported_formats")]
    fn set_supported_formats(&self, supportedformats: TrackType);

    #[doc(alias = "supported-formats")]
    fn connect_supported_formats_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ClipAsset>> ClipAssetExt for O {
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn frame_time(&self, frame_number: FrameNumber) -> Option<gst::ClockTime> {
        unsafe {
            from_glib(ffi::ges_clip_asset_get_frame_time(
                self.as_ref().to_glib_none().0,
                frame_number,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn natural_framerate(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut framerate_n = mem::MaybeUninit::uninit();
            let mut framerate_d = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::ges_clip_asset_get_natural_framerate(
                self.as_ref().to_glib_none().0,
                framerate_n.as_mut_ptr(),
                framerate_d.as_mut_ptr(),
            ));
            if ret {
                Some((framerate_n.assume_init(), framerate_d.assume_init()))
            } else {
                None
            }
        }
    }

    fn supported_formats(&self) -> TrackType {
        unsafe {
            from_glib(ffi::ges_clip_asset_get_supported_formats(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_supported_formats(&self, supportedformats: TrackType) {
        unsafe {
            ffi::ges_clip_asset_set_supported_formats(
                self.as_ref().to_glib_none().0,
                supportedformats.into_glib(),
            );
        }
    }

    fn connect_supported_formats_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_supported_formats_trampoline<
            P: IsA<ClipAsset>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESClipAsset,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ClipAsset::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::supported-formats\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_supported_formats_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
