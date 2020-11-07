// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ges_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::value::SetValueOptional;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gst;
use gst_pbutils;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::ptr;
use PipelineFlags;
use Timeline;

glib_wrapper! {
    pub struct Pipeline(Object<ges_sys::GESPipeline, ges_sys::GESPipelineClass>) @extends gst::Pipeline, gst::Element, gst::Object;

    match fn {
        get_type => || ges_sys::ges_pipeline_get_type(),
    }
}

impl Pipeline {
    pub fn new() -> Pipeline {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ges_sys::ges_pipeline_new()) }
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_PIPELINE: Option<&Pipeline> = None;

pub trait GESPipelineExt: 'static {
    fn get_mode(&self) -> PipelineFlags;

    fn get_thumbnail(&self, caps: &gst::Caps) -> Option<gst::Sample>;

    fn get_thumbnail_rgb24(&self, width: i32, height: i32) -> Option<gst::Sample>;

    fn preview_get_audio_sink(&self) -> Option<gst::Element>;

    fn preview_get_video_sink(&self) -> Option<gst::Element>;

    fn preview_set_audio_sink<P: IsA<gst::Element>>(&self, sink: &P);

    fn preview_set_video_sink<P: IsA<gst::Element>>(&self, sink: &P);

    fn save_thumbnail(
        &self,
        width: i32,
        height: i32,
        format: &str,
        location: &str,
    ) -> Result<(), glib::Error>;

    fn set_mode(&self, mode: PipelineFlags) -> Result<(), glib::error::BoolError>;

    fn set_render_settings<P: IsA<gst_pbutils::EncodingProfile>>(
        &self,
        output_uri: &str,
        profile: &P,
    ) -> Result<(), glib::error::BoolError>;

    fn set_timeline<P: IsA<Timeline>>(&self, timeline: &P) -> Result<(), glib::error::BoolError>;

    fn get_property_audio_filter(&self) -> Option<gst::Element>;

    fn set_property_audio_filter<P: IsA<gst::Element> + SetValueOptional>(
        &self,
        audio_filter: Option<&P>,
    );

    fn get_property_audio_sink(&self) -> Option<gst::Element>;

    fn set_property_audio_sink<P: IsA<gst::Element> + SetValueOptional>(
        &self,
        audio_sink: Option<&P>,
    );

    fn get_property_timeline(&self) -> Option<Timeline>;

    fn get_property_video_filter(&self) -> Option<gst::Element>;

    fn set_property_video_filter<P: IsA<gst::Element> + SetValueOptional>(
        &self,
        video_filter: Option<&P>,
    );

    fn get_property_video_sink(&self) -> Option<gst::Element>;

    fn set_property_video_sink<P: IsA<gst::Element> + SetValueOptional>(
        &self,
        video_sink: Option<&P>,
    );

    fn connect_property_audio_filter_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_audio_sink_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_timeline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_video_filter_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_video_sink_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Pipeline>> GESPipelineExt for O {
    fn get_mode(&self) -> PipelineFlags {
        unsafe {
            from_glib(ges_sys::ges_pipeline_get_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_thumbnail(&self, caps: &gst::Caps) -> Option<gst::Sample> {
        unsafe {
            from_glib_full(ges_sys::ges_pipeline_get_thumbnail(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
            ))
        }
    }

    fn get_thumbnail_rgb24(&self, width: i32, height: i32) -> Option<gst::Sample> {
        unsafe {
            from_glib_full(ges_sys::ges_pipeline_get_thumbnail_rgb24(
                self.as_ref().to_glib_none().0,
                width,
                height,
            ))
        }
    }

    fn preview_get_audio_sink(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ges_sys::ges_pipeline_preview_get_audio_sink(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn preview_get_video_sink(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ges_sys::ges_pipeline_preview_get_video_sink(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn preview_set_audio_sink<P: IsA<gst::Element>>(&self, sink: &P) {
        unsafe {
            ges_sys::ges_pipeline_preview_set_audio_sink(
                self.as_ref().to_glib_none().0,
                sink.as_ref().to_glib_none().0,
            );
        }
    }

    fn preview_set_video_sink<P: IsA<gst::Element>>(&self, sink: &P) {
        unsafe {
            ges_sys::ges_pipeline_preview_set_video_sink(
                self.as_ref().to_glib_none().0,
                sink.as_ref().to_glib_none().0,
            );
        }
    }

    fn save_thumbnail(
        &self,
        width: i32,
        height: i32,
        format: &str,
        location: &str,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ges_sys::ges_pipeline_save_thumbnail(
                self.as_ref().to_glib_none().0,
                width,
                height,
                format.to_glib_none().0,
                location.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_mode(&self, mode: PipelineFlags) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                ges_sys::ges_pipeline_set_mode(self.as_ref().to_glib_none().0, mode.to_glib()),
                "Failed to set mode"
            )
        }
    }

    fn set_render_settings<P: IsA<gst_pbutils::EncodingProfile>>(
        &self,
        output_uri: &str,
        profile: &P,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                ges_sys::ges_pipeline_set_render_settings(
                    self.as_ref().to_glib_none().0,
                    output_uri.to_glib_none().0,
                    profile.as_ref().to_glib_none().0
                ),
                "Failed to set render settings"
            )
        }
    }

    fn set_timeline<P: IsA<Timeline>>(&self, timeline: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                ges_sys::ges_pipeline_set_timeline(
                    self.as_ref().to_glib_none().0,
                    timeline.as_ref().to_glib_full()
                ),
                "Failed to set timeline"
            )
        }
    }

    fn get_property_audio_filter(&self) -> Option<gst::Element> {
        unsafe {
            let mut value = Value::from_type(<gst::Element as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"audio-filter\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `audio-filter` getter")
        }
    }

    fn set_property_audio_filter<P: IsA<gst::Element> + SetValueOptional>(
        &self,
        audio_filter: Option<&P>,
    ) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"audio-filter\0".as_ptr() as *const _,
                Value::from(audio_filter).to_glib_none().0,
            );
        }
    }

    fn get_property_audio_sink(&self) -> Option<gst::Element> {
        unsafe {
            let mut value = Value::from_type(<gst::Element as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"audio-sink\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `audio-sink` getter")
        }
    }

    fn set_property_audio_sink<P: IsA<gst::Element> + SetValueOptional>(
        &self,
        audio_sink: Option<&P>,
    ) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"audio-sink\0".as_ptr() as *const _,
                Value::from(audio_sink).to_glib_none().0,
            );
        }
    }

    fn get_property_timeline(&self) -> Option<Timeline> {
        unsafe {
            let mut value = Value::from_type(<Timeline as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"timeline\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `timeline` getter")
        }
    }

    fn get_property_video_filter(&self) -> Option<gst::Element> {
        unsafe {
            let mut value = Value::from_type(<gst::Element as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"video-filter\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `video-filter` getter")
        }
    }

    fn set_property_video_filter<P: IsA<gst::Element> + SetValueOptional>(
        &self,
        video_filter: Option<&P>,
    ) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"video-filter\0".as_ptr() as *const _,
                Value::from(video_filter).to_glib_none().0,
            );
        }
    }

    fn get_property_video_sink(&self) -> Option<gst::Element> {
        unsafe {
            let mut value = Value::from_type(<gst::Element as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"video-sink\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `video-sink` getter")
        }
    }

    fn set_property_video_sink<P: IsA<gst::Element> + SetValueOptional>(
        &self,
        video_sink: Option<&P>,
    ) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"video-sink\0".as_ptr() as *const _,
                Value::from(video_sink).to_glib_none().0,
            );
        }
    }

    fn connect_property_audio_filter_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_audio_filter_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESPipeline,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Pipeline>,
        {
            let f: &F = &*(f as *const F);
            f(&Pipeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::audio-filter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_audio_filter_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_audio_sink_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_audio_sink_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESPipeline,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Pipeline>,
        {
            let f: &F = &*(f as *const F);
            f(&Pipeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::audio-sink\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_audio_sink_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESPipeline,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Pipeline>,
        {
            let f: &F = &*(f as *const F);
            f(&Pipeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_timeline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeline_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESPipeline,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Pipeline>,
        {
            let f: &F = &*(f as *const F);
            f(&Pipeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeline\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_timeline_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_video_filter_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_video_filter_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESPipeline,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Pipeline>,
        {
            let f: &F = &*(f as *const F);
            f(&Pipeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::video-filter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_video_filter_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_video_sink_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_video_sink_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESPipeline,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Pipeline>,
        {
            let f: &F = &*(f as *const F);
            f(&Pipeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::video-sink\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_video_sink_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
