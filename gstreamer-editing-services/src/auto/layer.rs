// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Asset;
use crate::Clip;
use crate::Extractable;
use crate::MetaContainer;
use crate::Timeline;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use crate::Track;
use crate::TrackType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GESLayer")]
    pub struct Layer(Object<ffi::GESLayer, ffi::GESLayerClass>) @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_layer_get_type(),
    }
}

impl Layer {
    pub const NONE: Option<&'static Layer> = None;

    #[doc(alias = "ges_layer_new")]
    pub fn new() -> Layer {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_layer_new()) }
    }
}

impl Default for Layer {
    fn default() -> Self {
        Self::new()
    }
}

pub trait LayerExt: 'static {
    #[doc(alias = "ges_layer_add_asset")]
    fn add_asset(
        &self,
        asset: &impl IsA<Asset>,
        start: impl Into<Option<gst::ClockTime>>,
        inpoint: impl Into<Option<gst::ClockTime>>,
        duration: impl Into<Option<gst::ClockTime>>,
        track_types: TrackType,
    ) -> Result<Clip, glib::BoolError>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_layer_add_asset_full")]
    fn add_asset_full(
        &self,
        asset: &impl IsA<Asset>,
        start: impl Into<Option<gst::ClockTime>>,
        inpoint: impl Into<Option<gst::ClockTime>>,
        duration: impl Into<Option<gst::ClockTime>>,
        track_types: TrackType,
    ) -> Result<Clip, glib::Error>;

    #[doc(alias = "ges_layer_add_clip")]
    fn add_clip(&self, clip: &impl IsA<Clip>) -> Result<(), glib::error::BoolError>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_layer_add_clip_full")]
    fn add_clip_full(&self, clip: &impl IsA<Clip>) -> Result<(), glib::Error>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_layer_get_active_for_track")]
    #[doc(alias = "get_active_for_track")]
    fn is_active_for_track(&self, track: &impl IsA<Track>) -> bool;

    #[doc(alias = "ges_layer_get_auto_transition")]
    #[doc(alias = "get_auto_transition")]
    fn is_auto_transition(&self) -> bool;

    #[doc(alias = "ges_layer_get_clips")]
    #[doc(alias = "get_clips")]
    fn clips(&self) -> Vec<Clip>;

    #[doc(alias = "ges_layer_get_clips_in_interval")]
    #[doc(alias = "get_clips_in_interval")]
    fn clips_in_interval(
        &self,
        start: impl Into<Option<gst::ClockTime>>,
        end: impl Into<Option<gst::ClockTime>>,
    ) -> Vec<Clip>;

    #[doc(alias = "ges_layer_get_duration")]
    #[doc(alias = "get_duration")]
    fn duration(&self) -> gst::ClockTime;

    #[doc(alias = "ges_layer_get_priority")]
    #[doc(alias = "get_priority")]
    fn priority(&self) -> u32;

    #[doc(alias = "ges_layer_get_timeline")]
    #[doc(alias = "get_timeline")]
    fn timeline(&self) -> Option<Timeline>;

    #[doc(alias = "ges_layer_is_empty")]
    fn is_empty(&self) -> bool;

    #[doc(alias = "ges_layer_remove_clip")]
    fn remove_clip(&self, clip: &impl IsA<Clip>) -> Result<(), glib::error::BoolError>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_layer_set_active_for_tracks")]
    fn set_active_for_tracks(&self, active: bool, tracks: &[Track]) -> bool;

    #[doc(alias = "ges_layer_set_auto_transition")]
    fn set_auto_transition(&self, auto_transition: bool);

    #[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
    #[doc(alias = "ges_layer_set_priority")]
    fn set_priority(&self, priority: u32);

    #[doc(alias = "ges_layer_set_timeline")]
    fn set_timeline(&self, timeline: &impl IsA<Timeline>);

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    //#[doc(alias = "active-changed")]
    //fn connect_active_changed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "clip-added")]
    fn connect_clip_added<F: Fn(&Self, &Clip) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "clip-removed")]
    fn connect_clip_removed<F: Fn(&Self, &Clip) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "auto-transition")]
    fn connect_auto_transition_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
    #[doc(alias = "priority")]
    fn connect_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Layer>> LayerExt for O {
    fn add_asset(
        &self,
        asset: &impl IsA<Asset>,
        start: impl Into<Option<gst::ClockTime>>,
        inpoint: impl Into<Option<gst::ClockTime>>,
        duration: impl Into<Option<gst::ClockTime>>,
        track_types: TrackType,
    ) -> Result<Clip, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_none(ffi::ges_layer_add_asset(
                self.as_ref().to_glib_none().0,
                asset.as_ref().to_glib_none().0,
                start.into().into_glib(),
                inpoint.into().into_glib(),
                duration.into().into_glib(),
                track_types.into_glib(),
            ))
            .ok_or_else(|| glib::bool_error!("Failed to add asset"))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn add_asset_full(
        &self,
        asset: &impl IsA<Asset>,
        start: impl Into<Option<gst::ClockTime>>,
        inpoint: impl Into<Option<gst::ClockTime>>,
        duration: impl Into<Option<gst::ClockTime>>,
        track_types: TrackType,
    ) -> Result<Clip, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ges_layer_add_asset_full(
                self.as_ref().to_glib_none().0,
                asset.as_ref().to_glib_none().0,
                start.into().into_glib(),
                inpoint.into().into_glib(),
                duration.into().into_glib(),
                track_types.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn add_clip(&self, clip: &impl IsA<Clip>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_layer_add_clip(
                    self.as_ref().to_glib_none().0,
                    clip.as_ref().to_glib_none().0
                ),
                "Failed to add clip"
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn add_clip_full(&self, clip: &impl IsA<Clip>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::ges_layer_add_clip_full(
                self.as_ref().to_glib_none().0,
                clip.as_ref().to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == 0, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn is_active_for_track(&self, track: &impl IsA<Track>) -> bool {
        unsafe {
            from_glib(ffi::ges_layer_get_active_for_track(
                self.as_ref().to_glib_none().0,
                track.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_auto_transition(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_layer_get_auto_transition(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn clips(&self) -> Vec<Clip> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_layer_get_clips(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn clips_in_interval(
        &self,
        start: impl Into<Option<gst::ClockTime>>,
        end: impl Into<Option<gst::ClockTime>>,
    ) -> Vec<Clip> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_layer_get_clips_in_interval(
                self.as_ref().to_glib_none().0,
                start.into().into_glib(),
                end.into().into_glib(),
            ))
        }
    }

    fn duration(&self) -> gst::ClockTime {
        unsafe {
            try_from_glib(ffi::ges_layer_get_duration(self.as_ref().to_glib_none().0))
                .expect("mandatory glib value is None")
        }
    }

    fn priority(&self) -> u32 {
        unsafe { ffi::ges_layer_get_priority(self.as_ref().to_glib_none().0) }
    }

    fn timeline(&self) -> Option<Timeline> {
        unsafe { from_glib_none(ffi::ges_layer_get_timeline(self.as_ref().to_glib_none().0)) }
    }

    fn is_empty(&self) -> bool {
        unsafe { from_glib(ffi::ges_layer_is_empty(self.as_ref().to_glib_none().0)) }
    }

    fn remove_clip(&self, clip: &impl IsA<Clip>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_layer_remove_clip(
                    self.as_ref().to_glib_none().0,
                    clip.as_ref().to_glib_none().0
                ),
                "Failed to remove clip"
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_active_for_tracks(&self, active: bool, tracks: &[Track]) -> bool {
        unsafe {
            from_glib(ffi::ges_layer_set_active_for_tracks(
                self.as_ref().to_glib_none().0,
                active.into_glib(),
                tracks.to_glib_none().0,
            ))
        }
    }

    fn set_auto_transition(&self, auto_transition: bool) {
        unsafe {
            ffi::ges_layer_set_auto_transition(
                self.as_ref().to_glib_none().0,
                auto_transition.into_glib(),
            );
        }
    }

    fn set_priority(&self, priority: u32) {
        unsafe {
            ffi::ges_layer_set_priority(self.as_ref().to_glib_none().0, priority);
        }
    }

    fn set_timeline(&self, timeline: &impl IsA<Timeline>) {
        unsafe {
            ffi::ges_layer_set_timeline(
                self.as_ref().to_glib_none().0,
                timeline.as_ref().to_glib_none().0,
            );
        }
    }

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    //fn connect_active_changed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype tracks: *.PtrArray TypeId { ns_id: 1, id: 17 }
    //}

    fn connect_clip_added<F: Fn(&Self, &Clip) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn clip_added_trampoline<P: IsA<Layer>, F: Fn(&P, &Clip) + 'static>(
            this: *mut ffi::GESLayer,
            clip: *mut ffi::GESClip,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Layer::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(clip),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"clip-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    clip_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_clip_removed<F: Fn(&Self, &Clip) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn clip_removed_trampoline<P: IsA<Layer>, F: Fn(&P, &Clip) + 'static>(
            this: *mut ffi::GESLayer,
            clip: *mut ffi::GESClip,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Layer::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(clip),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"clip-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    clip_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_auto_transition_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_auto_transition_trampoline<
            P: IsA<Layer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESLayer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Layer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::auto-transition\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_auto_transition_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_priority_trampoline<P: IsA<Layer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESLayer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Layer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::priority\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_priority_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
