// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;

bitflags! {
    #[doc(alias = "GstAudioFlags")]
    pub struct AudioFlags: u32 {
        #[doc(alias = "GST_AUDIO_FLAG_UNPOSITIONED")]
        const UNPOSITIONED = ffi::GST_AUDIO_FLAG_UNPOSITIONED as _;
    }
}

#[doc(hidden)]
impl IntoGlib for AudioFlags {
    type GlibType = ffi::GstAudioFlags;

    fn into_glib(self) -> ffi::GstAudioFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioFlags> for AudioFlags {
    unsafe fn from_glib(value: ffi::GstAudioFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for AudioFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_flags_get_type()) }
    }
}

impl glib::value::ValueType for AudioFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AudioFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for AudioFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<AudioFlags> for glib::Value {
    #[inline]
    fn from(v: AudioFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "GstAudioFormatFlags")]
    pub struct AudioFormatFlags: u32 {
        #[doc(alias = "GST_AUDIO_FORMAT_FLAG_INTEGER")]
        const INTEGER = ffi::GST_AUDIO_FORMAT_FLAG_INTEGER as _;
        #[doc(alias = "GST_AUDIO_FORMAT_FLAG_FLOAT")]
        const FLOAT = ffi::GST_AUDIO_FORMAT_FLAG_FLOAT as _;
        #[doc(alias = "GST_AUDIO_FORMAT_FLAG_SIGNED")]
        const SIGNED = ffi::GST_AUDIO_FORMAT_FLAG_SIGNED as _;
        #[doc(alias = "GST_AUDIO_FORMAT_FLAG_COMPLEX")]
        const COMPLEX = ffi::GST_AUDIO_FORMAT_FLAG_COMPLEX as _;
        #[doc(alias = "GST_AUDIO_FORMAT_FLAG_UNPACK")]
        const UNPACK = ffi::GST_AUDIO_FORMAT_FLAG_UNPACK as _;
    }
}

#[doc(hidden)]
impl IntoGlib for AudioFormatFlags {
    type GlibType = ffi::GstAudioFormatFlags;

    fn into_glib(self) -> ffi::GstAudioFormatFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioFormatFlags> for AudioFormatFlags {
    unsafe fn from_glib(value: ffi::GstAudioFormatFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for AudioFormatFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_format_flags_get_type()) }
    }
}

impl glib::value::ValueType for AudioFormatFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AudioFormatFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for AudioFormatFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<AudioFormatFlags> for glib::Value {
    #[inline]
    fn from(v: AudioFormatFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "GstAudioPackFlags")]
    pub struct AudioPackFlags: u32 {
        #[doc(alias = "GST_AUDIO_PACK_FLAG_TRUNCATE_RANGE")]
        const TRUNCATE_RANGE = ffi::GST_AUDIO_PACK_FLAG_TRUNCATE_RANGE as _;
    }
}

#[doc(hidden)]
impl IntoGlib for AudioPackFlags {
    type GlibType = ffi::GstAudioPackFlags;

    fn into_glib(self) -> ffi::GstAudioPackFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioPackFlags> for AudioPackFlags {
    unsafe fn from_glib(value: ffi::GstAudioPackFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for AudioPackFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_pack_flags_get_type()) }
    }
}

impl glib::value::ValueType for AudioPackFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AudioPackFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for AudioPackFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<AudioPackFlags> for glib::Value {
    #[inline]
    fn from(v: AudioPackFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
