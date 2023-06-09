// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Asset;
use crate::MetaContainer;
use crate::TrackType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GESTrackElementAsset")]
    pub struct TrackElementAsset(Object<ffi::GESTrackElementAsset, ffi::GESTrackElementAssetClass>) @extends Asset, @implements MetaContainer;

    match fn {
        type_ => || ffi::ges_track_element_asset_get_type(),
    }
}

impl TrackElementAsset {
    pub const NONE: Option<&'static TrackElementAsset> = None;
}

pub trait TrackElementAssetExt: 'static {
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_element_asset_get_natural_framerate")]
    #[doc(alias = "get_natural_framerate")]
    fn natural_framerate(&self) -> Option<(i32, i32)>;

    #[doc(alias = "ges_track_element_asset_get_track_type")]
    #[doc(alias = "get_track_type")]
    fn track_type(&self) -> TrackType;

    #[doc(alias = "ges_track_element_asset_set_track_type")]
    fn set_track_type(&self, type_: TrackType);

    #[doc(alias = "track-type")]
    fn connect_track_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TrackElementAsset>> TrackElementAssetExt for O {
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn natural_framerate(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut framerate_n = mem::MaybeUninit::uninit();
            let mut framerate_d = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::ges_track_element_asset_get_natural_framerate(
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

    fn track_type(&self) -> TrackType {
        unsafe {
            from_glib(ffi::ges_track_element_asset_get_track_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_track_type(&self, type_: TrackType) {
        unsafe {
            ffi::ges_track_element_asset_set_track_type(
                self.as_ref().to_glib_none().0,
                type_.into_glib(),
            );
        }
    }

    fn connect_track_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_track_type_trampoline<
            P: IsA<TrackElementAsset>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTrackElementAsset,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TrackElementAsset::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::track-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_track_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
