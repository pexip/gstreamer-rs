// Generated by gir (https://github.com/gtk-rs/gir @ b5e4e17d87b0)
// from gir-files (https://github.com/gtk-rs/gir-files @ 05ae6b134dda)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ 4903e817c5dd)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type GstAppLeakyType = c_int;
pub const GST_APP_LEAKY_TYPE_NONE: GstAppLeakyType = 0;
pub const GST_APP_LEAKY_TYPE_UPSTREAM: GstAppLeakyType = 1;
pub const GST_APP_LEAKY_TYPE_DOWNSTREAM: GstAppLeakyType = 2;

pub type GstAppStreamType = c_int;
pub const GST_APP_STREAM_TYPE_STREAM: GstAppStreamType = 0;
pub const GST_APP_STREAM_TYPE_SEEKABLE: GstAppStreamType = 1;
pub const GST_APP_STREAM_TYPE_RANDOM_ACCESS: GstAppStreamType = 2;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstAppSinkCallbacks {
    pub eos: Option<unsafe extern "C" fn(*mut GstAppSink, gpointer)>,
    pub new_preroll: Option<unsafe extern "C" fn(*mut GstAppSink, gpointer) -> gst::GstFlowReturn>,
    pub new_sample: Option<unsafe extern "C" fn(*mut GstAppSink, gpointer) -> gst::GstFlowReturn>,
    pub new_event: Option<unsafe extern "C" fn(*mut GstAppSink, gpointer) -> gboolean>,
    pub _gst_reserved: [gpointer; 3],
}

impl ::std::fmt::Debug for GstAppSinkCallbacks {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstAppSinkCallbacks @ {:p}", self))
            .field("eos", &self.eos)
            .field("new_preroll", &self.new_preroll)
            .field("new_sample", &self.new_sample)
            .field("new_event", &self.new_event)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstAppSinkClass {
    pub basesink_class: gst_base::GstBaseSinkClass,
    pub eos: Option<unsafe extern "C" fn(*mut GstAppSink)>,
    pub new_preroll: Option<unsafe extern "C" fn(*mut GstAppSink) -> gst::GstFlowReturn>,
    pub new_sample: Option<unsafe extern "C" fn(*mut GstAppSink) -> gst::GstFlowReturn>,
    pub pull_preroll: Option<unsafe extern "C" fn(*mut GstAppSink) -> *mut gst::GstSample>,
    pub pull_sample: Option<unsafe extern "C" fn(*mut GstAppSink) -> *mut gst::GstSample>,
    pub try_pull_preroll:
        Option<unsafe extern "C" fn(*mut GstAppSink, gst::GstClockTime) -> *mut gst::GstSample>,
    pub try_pull_sample:
        Option<unsafe extern "C" fn(*mut GstAppSink, gst::GstClockTime) -> *mut gst::GstSample>,
    pub try_pull_object:
        Option<unsafe extern "C" fn(*mut GstAppSink, gst::GstClockTime) -> *mut gst::GstMiniObject>,
    pub _gst_reserved: [gpointer; 1],
}

impl ::std::fmt::Debug for GstAppSinkClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstAppSinkClass @ {:p}", self))
            .field("basesink_class", &self.basesink_class)
            .field("eos", &self.eos)
            .field("new_preroll", &self.new_preroll)
            .field("new_sample", &self.new_sample)
            .field("pull_preroll", &self.pull_preroll)
            .field("pull_sample", &self.pull_sample)
            .field("try_pull_preroll", &self.try_pull_preroll)
            .field("try_pull_sample", &self.try_pull_sample)
            .field("try_pull_object", &self.try_pull_object)
            .finish()
    }
}

#[repr(C)]
pub struct _GstAppSinkPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstAppSinkPrivate = *mut _GstAppSinkPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstAppSrcCallbacks {
    pub need_data: Option<unsafe extern "C" fn(*mut GstAppSrc, c_uint, gpointer)>,
    pub enough_data: Option<unsafe extern "C" fn(*mut GstAppSrc, gpointer)>,
    pub seek_data: Option<unsafe extern "C" fn(*mut GstAppSrc, u64, gpointer) -> gboolean>,
    pub _gst_reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstAppSrcCallbacks {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstAppSrcCallbacks @ {:p}", self))
            .field("need_data", &self.need_data)
            .field("enough_data", &self.enough_data)
            .field("seek_data", &self.seek_data)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstAppSrcClass {
    pub basesrc_class: gst_base::GstBaseSrcClass,
    pub need_data: Option<unsafe extern "C" fn(*mut GstAppSrc, c_uint)>,
    pub enough_data: Option<unsafe extern "C" fn(*mut GstAppSrc)>,
    pub seek_data: Option<unsafe extern "C" fn(*mut GstAppSrc, u64) -> gboolean>,
    pub push_buffer:
        Option<unsafe extern "C" fn(*mut GstAppSrc, *mut gst::GstBuffer) -> gst::GstFlowReturn>,
    pub end_of_stream: Option<unsafe extern "C" fn(*mut GstAppSrc) -> gst::GstFlowReturn>,
    pub push_sample:
        Option<unsafe extern "C" fn(*mut GstAppSrc, *mut gst::GstSample) -> gst::GstFlowReturn>,
    pub push_buffer_list:
        Option<unsafe extern "C" fn(*mut GstAppSrc, *mut gst::GstBufferList) -> gst::GstFlowReturn>,
    pub _gst_reserved: [gpointer; 2],
}

impl ::std::fmt::Debug for GstAppSrcClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstAppSrcClass @ {:p}", self))
            .field("basesrc_class", &self.basesrc_class)
            .field("need_data", &self.need_data)
            .field("enough_data", &self.enough_data)
            .field("seek_data", &self.seek_data)
            .field("push_buffer", &self.push_buffer)
            .field("end_of_stream", &self.end_of_stream)
            .field("push_sample", &self.push_sample)
            .field("push_buffer_list", &self.push_buffer_list)
            .finish()
    }
}

#[repr(C)]
pub struct _GstAppSrcPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstAppSrcPrivate = *mut _GstAppSrcPrivate;

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstAppSink {
    pub basesink: gst_base::GstBaseSink,
    pub priv_: *mut GstAppSinkPrivate,
    pub _gst_reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstAppSink {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstAppSink @ {:p}", self))
            .field("basesink", &self.basesink)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstAppSrc {
    pub basesrc: gst_base::GstBaseSrc,
    pub priv_: *mut GstAppSrcPrivate,
    pub _gst_reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstAppSrc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstAppSrc @ {:p}", self))
            .field("basesrc", &self.basesrc)
            .finish()
    }
}

#[link(name = "gstapp-1.0")]
extern "C" {

    //=========================================================================
    // GstAppLeakyType
    //=========================================================================
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn gst_app_leaky_type_get_type() -> GType;

    //=========================================================================
    // GstAppStreamType
    //=========================================================================
    pub fn gst_app_stream_type_get_type() -> GType;

    //=========================================================================
    // GstAppSink
    //=========================================================================
    pub fn gst_app_sink_get_type() -> GType;
    pub fn gst_app_sink_get_buffer_list_support(appsink: *mut GstAppSink) -> gboolean;
    pub fn gst_app_sink_get_caps(appsink: *mut GstAppSink) -> *mut gst::GstCaps;
    pub fn gst_app_sink_get_drop(appsink: *mut GstAppSink) -> gboolean;
    pub fn gst_app_sink_get_emit_signals(appsink: *mut GstAppSink) -> gboolean;
    pub fn gst_app_sink_get_max_buffers(appsink: *mut GstAppSink) -> c_uint;
    pub fn gst_app_sink_get_wait_on_eos(appsink: *mut GstAppSink) -> gboolean;
    pub fn gst_app_sink_is_eos(appsink: *mut GstAppSink) -> gboolean;
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn gst_app_sink_pull_object(appsink: *mut GstAppSink) -> *mut gst::GstMiniObject;
    pub fn gst_app_sink_pull_preroll(appsink: *mut GstAppSink) -> *mut gst::GstSample;
    pub fn gst_app_sink_pull_sample(appsink: *mut GstAppSink) -> *mut gst::GstSample;
    pub fn gst_app_sink_set_buffer_list_support(appsink: *mut GstAppSink, enable_lists: gboolean);
    pub fn gst_app_sink_set_callbacks(
        appsink: *mut GstAppSink,
        callbacks: *mut GstAppSinkCallbacks,
        user_data: gpointer,
        notify: glib::GDestroyNotify,
    );
    pub fn gst_app_sink_set_caps(appsink: *mut GstAppSink, caps: *const gst::GstCaps);
    pub fn gst_app_sink_set_drop(appsink: *mut GstAppSink, drop: gboolean);
    pub fn gst_app_sink_set_emit_signals(appsink: *mut GstAppSink, emit: gboolean);
    pub fn gst_app_sink_set_max_buffers(appsink: *mut GstAppSink, max: c_uint);
    pub fn gst_app_sink_set_wait_on_eos(appsink: *mut GstAppSink, wait: gboolean);
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn gst_app_sink_try_pull_object(
        appsink: *mut GstAppSink,
        timeout: gst::GstClockTime,
    ) -> *mut gst::GstMiniObject;
    pub fn gst_app_sink_try_pull_preroll(
        appsink: *mut GstAppSink,
        timeout: gst::GstClockTime,
    ) -> *mut gst::GstSample;
    pub fn gst_app_sink_try_pull_sample(
        appsink: *mut GstAppSink,
        timeout: gst::GstClockTime,
    ) -> *mut gst::GstSample;

    //=========================================================================
    // GstAppSrc
    //=========================================================================
    pub fn gst_app_src_get_type() -> GType;
    pub fn gst_app_src_end_of_stream(appsrc: *mut GstAppSrc) -> gst::GstFlowReturn;
    pub fn gst_app_src_get_caps(appsrc: *mut GstAppSrc) -> *mut gst::GstCaps;
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn gst_app_src_get_current_level_buffers(appsrc: *mut GstAppSrc) -> u64;
    pub fn gst_app_src_get_current_level_bytes(appsrc: *mut GstAppSrc) -> u64;
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn gst_app_src_get_current_level_time(appsrc: *mut GstAppSrc) -> gst::GstClockTime;
    pub fn gst_app_src_get_duration(appsrc: *mut GstAppSrc) -> gst::GstClockTime;
    pub fn gst_app_src_get_emit_signals(appsrc: *mut GstAppSrc) -> gboolean;
    pub fn gst_app_src_get_latency(appsrc: *mut GstAppSrc, min: *mut u64, max: *mut u64);
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn gst_app_src_get_leaky_type(appsrc: *mut GstAppSrc) -> GstAppLeakyType;
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn gst_app_src_get_max_buffers(appsrc: *mut GstAppSrc) -> u64;
    pub fn gst_app_src_get_max_bytes(appsrc: *mut GstAppSrc) -> u64;
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn gst_app_src_get_max_time(appsrc: *mut GstAppSrc) -> gst::GstClockTime;
    pub fn gst_app_src_get_size(appsrc: *mut GstAppSrc) -> i64;
    pub fn gst_app_src_get_stream_type(appsrc: *mut GstAppSrc) -> GstAppStreamType;
    pub fn gst_app_src_push_buffer(
        appsrc: *mut GstAppSrc,
        buffer: *mut gst::GstBuffer,
    ) -> gst::GstFlowReturn;
    pub fn gst_app_src_push_buffer_list(
        appsrc: *mut GstAppSrc,
        buffer_list: *mut gst::GstBufferList,
    ) -> gst::GstFlowReturn;
    pub fn gst_app_src_push_sample(
        appsrc: *mut GstAppSrc,
        sample: *mut gst::GstSample,
    ) -> gst::GstFlowReturn;
    pub fn gst_app_src_set_callbacks(
        appsrc: *mut GstAppSrc,
        callbacks: *mut GstAppSrcCallbacks,
        user_data: gpointer,
        notify: glib::GDestroyNotify,
    );
    pub fn gst_app_src_set_caps(appsrc: *mut GstAppSrc, caps: *const gst::GstCaps);
    pub fn gst_app_src_set_duration(appsrc: *mut GstAppSrc, duration: gst::GstClockTime);
    pub fn gst_app_src_set_emit_signals(appsrc: *mut GstAppSrc, emit: gboolean);
    pub fn gst_app_src_set_latency(appsrc: *mut GstAppSrc, min: u64, max: u64);
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn gst_app_src_set_leaky_type(appsrc: *mut GstAppSrc, leaky: GstAppLeakyType);
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn gst_app_src_set_max_buffers(appsrc: *mut GstAppSrc, max: u64);
    pub fn gst_app_src_set_max_bytes(appsrc: *mut GstAppSrc, max: u64);
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn gst_app_src_set_max_time(appsrc: *mut GstAppSrc, max: gst::GstClockTime);
    pub fn gst_app_src_set_size(appsrc: *mut GstAppSrc, size: i64);
    pub fn gst_app_src_set_stream_type(appsrc: *mut GstAppSrc, type_: GstAppStreamType);

}
