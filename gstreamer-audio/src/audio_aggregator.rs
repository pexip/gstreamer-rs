use crate::auto::AudioAggregator;
use crate::auto::AudioAggregatorPad;
#[cfg(any(feature = "v1_18", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v1_18", feature = "dox"))]
use glib::signal::{connect_raw, SignalHandlerId};
use glib::translate::*;

#[cfg(any(feature = "v1_18", feature = "dox"))]
use std::mem::transmute;

pub trait AudioAggregatorExtManual: 'static {
    #[doc(alias = "gst_audio_aggregator_set_sink_caps")]
    fn set_sink_caps(&self, pad: &impl IsA<AudioAggregatorPad>, caps: &gst::CapsRef);

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "output-buffer-duration-fraction")]
    fn output_buffer_duration_fraction(&self) -> gst::Fraction;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "output-buffer-duration-fraction")]
    fn set_output_buffer_duration_fraction(&self, output_buffer_duration_fraction: gst::Fraction);

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "output-buffer-duration-fraction")]
    fn connect_output_buffer_duration_fraction_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn current_caps(&self) -> Option<gst::Caps>;
    fn current_audio_info(&self) -> Option<crate::AudioInfo>;
}

impl<O: IsA<AudioAggregator>> AudioAggregatorExtManual for O {
    fn set_sink_caps(&self, pad: &impl IsA<AudioAggregatorPad>, caps: &gst::CapsRef) {
        unsafe {
            ffi::gst_audio_aggregator_set_sink_caps(
                self.as_ref().to_glib_none().0,
                pad.as_ref().to_glib_none().0,
                caps.as_mut_ptr(),
            );
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn output_buffer_duration_fraction(&self) -> gst::Fraction {
        glib::ObjectExt::property(self.as_ref(), "output-buffer-duration-fraction")
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_output_buffer_duration_fraction(&self, output_buffer_duration_fraction: gst::Fraction) {
        glib::ObjectExt::set_property(
            self.as_ref(),
            "output-buffer-duration-fraction",
            output_buffer_duration_fraction,
        )
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_output_buffer_duration_fraction_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_output_buffer_duration_fraction_trampoline<
            P: IsA<AudioAggregator>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioAggregator,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AudioAggregator::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::output-buffer-duration-fraction\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_output_buffer_duration_fraction_trampoline::<Self, F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    fn current_caps(&self) -> Option<gst::Caps> {
        unsafe {
            let ptr = self.as_ptr() as *mut ffi::GstAudioAggregator;
            let _guard = crate::utils::MutexGuard::lock(&(*(ptr as *mut gst::ffi::GstObject)).lock);
            from_glib_none((*ptr).current_caps)
        }
    }

    fn current_audio_info(&self) -> Option<crate::AudioInfo> {
        self.current_caps()
            .and_then(|caps| crate::AudioInfo::from_caps(&caps).ok())
    }
}
