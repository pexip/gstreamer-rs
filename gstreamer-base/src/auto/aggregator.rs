// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use crate::{AggregatorPad, AggregatorStartTimeSelection};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GstAggregator")]
    pub struct Aggregator(Object<ffi::GstAggregator, ffi::GstAggregatorClass>) @extends gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_aggregator_get_type(),
    }
}

impl Aggregator {
    pub const NONE: Option<&'static Aggregator> = None;
}

unsafe impl Send for Aggregator {}
unsafe impl Sync for Aggregator {}

pub trait AggregatorExt: 'static {
    #[doc(alias = "gst_aggregator_get_buffer_pool")]
    #[doc(alias = "get_buffer_pool")]
    fn buffer_pool(&self) -> Option<gst::BufferPool>;

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_aggregator_get_force_live")]
    #[doc(alias = "get_force_live")]
    fn is_force_live(&self) -> bool;

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_aggregator_get_ignore_inactive_pads")]
    #[doc(alias = "get_ignore_inactive_pads")]
    fn ignores_inactive_pads(&self) -> bool;

    #[doc(alias = "gst_aggregator_get_latency")]
    #[doc(alias = "get_latency")]
    fn latency(&self) -> Option<gst::ClockTime>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_aggregator_negotiate")]
    fn negotiate(&self) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_aggregator_peek_next_sample")]
    fn peek_next_sample(&self, pad: &impl IsA<AggregatorPad>) -> Option<gst::Sample>;

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_aggregator_set_force_live")]
    fn set_force_live(&self, force_live: bool);

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_aggregator_set_ignore_inactive_pads")]
    fn set_ignore_inactive_pads(&self, ignore: bool);

    #[doc(alias = "gst_aggregator_set_latency")]
    fn set_latency(
        &self,
        min_latency: gst::ClockTime,
        max_latency: impl Into<Option<gst::ClockTime>>,
    );

    #[doc(alias = "gst_aggregator_set_src_caps")]
    fn set_src_caps(&self, caps: &gst::Caps);

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_aggregator_simple_get_next_time")]
    fn simple_get_next_time(&self) -> Option<gst::ClockTime>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "emit-signals")]
    fn emits_signals(&self) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "emit-signals")]
    fn set_emit_signals(&self, emit_signals: bool);

    #[doc(alias = "start-time")]
    fn start_time(&self) -> u64;

    #[doc(alias = "start-time")]
    fn set_start_time(&self, start_time: u64);

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "start-time-selection")]
    fn start_time_selection(&self) -> AggregatorStartTimeSelection;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "start-time-selection")]
    fn set_start_time_selection(&self, start_time_selection: AggregatorStartTimeSelection);

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "emit-signals")]
    fn connect_emit_signals_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "latency")]
    fn connect_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F)
        -> SignalHandlerId;

    #[doc(alias = "start-time")]
    fn connect_start_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "start-time-selection")]
    fn connect_start_time_selection_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Aggregator>> AggregatorExt for O {
    fn buffer_pool(&self) -> Option<gst::BufferPool> {
        unsafe {
            from_glib_full(ffi::gst_aggregator_get_buffer_pool(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn is_force_live(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_aggregator_get_force_live(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn ignores_inactive_pads(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_aggregator_get_ignore_inactive_pads(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn latency(&self) -> Option<gst::ClockTime> {
        unsafe {
            from_glib(ffi::gst_aggregator_get_latency(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn negotiate(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_aggregator_negotiate(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn peek_next_sample(&self, pad: &impl IsA<AggregatorPad>) -> Option<gst::Sample> {
        unsafe {
            from_glib_full(ffi::gst_aggregator_peek_next_sample(
                self.as_ref().to_glib_none().0,
                pad.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn set_force_live(&self, force_live: bool) {
        unsafe {
            ffi::gst_aggregator_set_force_live(
                self.as_ref().to_glib_none().0,
                force_live.into_glib(),
            );
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn set_ignore_inactive_pads(&self, ignore: bool) {
        unsafe {
            ffi::gst_aggregator_set_ignore_inactive_pads(
                self.as_ref().to_glib_none().0,
                ignore.into_glib(),
            );
        }
    }

    fn set_latency(
        &self,
        min_latency: gst::ClockTime,
        max_latency: impl Into<Option<gst::ClockTime>>,
    ) {
        unsafe {
            ffi::gst_aggregator_set_latency(
                self.as_ref().to_glib_none().0,
                min_latency.into_glib(),
                max_latency.into().into_glib(),
            );
        }
    }

    fn set_src_caps(&self, caps: &gst::Caps) {
        unsafe {
            ffi::gst_aggregator_set_src_caps(self.as_ref().to_glib_none().0, caps.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn simple_get_next_time(&self) -> Option<gst::ClockTime> {
        unsafe {
            from_glib(ffi::gst_aggregator_simple_get_next_time(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn emits_signals(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "emit-signals")
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_emit_signals(&self, emit_signals: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "emit-signals", &emit_signals)
    }

    fn start_time(&self) -> u64 {
        glib::ObjectExt::property(self.as_ref(), "start-time")
    }

    fn set_start_time(&self, start_time: u64) {
        glib::ObjectExt::set_property(self.as_ref(), "start-time", &start_time)
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn start_time_selection(&self) -> AggregatorStartTimeSelection {
        glib::ObjectExt::property(self.as_ref(), "start-time-selection")
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_start_time_selection(&self, start_time_selection: AggregatorStartTimeSelection) {
        glib::ObjectExt::set_property(self.as_ref(), "start-time-selection", &start_time_selection)
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_emit_signals_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_emit_signals_trampoline<
            P: IsA<Aggregator>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAggregator,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Aggregator::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::emit-signals\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_emit_signals_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_latency_trampoline<
            P: IsA<Aggregator>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAggregator,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Aggregator::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::latency\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_latency_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_start_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_start_time_trampoline<
            P: IsA<Aggregator>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAggregator,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Aggregator::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::start-time\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_start_time_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_start_time_selection_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_start_time_selection_trampoline<
            P: IsA<Aggregator>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAggregator,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Aggregator::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::start-time-selection\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_start_time_selection_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
