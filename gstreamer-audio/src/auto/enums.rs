// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstAudioChannelPosition")]
pub enum AudioChannelPosition {
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_NONE")]
    None,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_MONO")]
    Mono,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_INVALID")]
    Invalid,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_FRONT_LEFT")]
    FrontLeft,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_FRONT_RIGHT")]
    FrontRight,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_FRONT_CENTER")]
    FrontCenter,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_LFE1")]
    Lfe1,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_REAR_LEFT")]
    RearLeft,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_REAR_RIGHT")]
    RearRight,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_FRONT_LEFT_OF_CENTER")]
    FrontLeftOfCenter,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_FRONT_RIGHT_OF_CENTER")]
    FrontRightOfCenter,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_REAR_CENTER")]
    RearCenter,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_LFE2")]
    Lfe2,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_SIDE_LEFT")]
    SideLeft,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_SIDE_RIGHT")]
    SideRight,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_TOP_FRONT_LEFT")]
    TopFrontLeft,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_TOP_FRONT_RIGHT")]
    TopFrontRight,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_TOP_FRONT_CENTER")]
    TopFrontCenter,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_TOP_CENTER")]
    TopCenter,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_TOP_REAR_LEFT")]
    TopRearLeft,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_TOP_REAR_RIGHT")]
    TopRearRight,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_TOP_SIDE_LEFT")]
    TopSideLeft,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_TOP_SIDE_RIGHT")]
    TopSideRight,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_TOP_REAR_CENTER")]
    TopRearCenter,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_BOTTOM_FRONT_CENTER")]
    BottomFrontCenter,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_BOTTOM_FRONT_LEFT")]
    BottomFrontLeft,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_BOTTOM_FRONT_RIGHT")]
    BottomFrontRight,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_WIDE_LEFT")]
    WideLeft,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_WIDE_RIGHT")]
    WideRight,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_SURROUND_LEFT")]
    SurroundLeft,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_SURROUND_RIGHT")]
    SurroundRight,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for AudioChannelPosition {
    type GlibType = ffi::GstAudioChannelPosition;

    fn into_glib(self) -> ffi::GstAudioChannelPosition {
        match self {
            Self::None => ffi::GST_AUDIO_CHANNEL_POSITION_NONE,
            Self::Mono => ffi::GST_AUDIO_CHANNEL_POSITION_MONO,
            Self::Invalid => ffi::GST_AUDIO_CHANNEL_POSITION_INVALID,
            Self::FrontLeft => ffi::GST_AUDIO_CHANNEL_POSITION_FRONT_LEFT,
            Self::FrontRight => ffi::GST_AUDIO_CHANNEL_POSITION_FRONT_RIGHT,
            Self::FrontCenter => ffi::GST_AUDIO_CHANNEL_POSITION_FRONT_CENTER,
            Self::Lfe1 => ffi::GST_AUDIO_CHANNEL_POSITION_LFE1,
            Self::RearLeft => ffi::GST_AUDIO_CHANNEL_POSITION_REAR_LEFT,
            Self::RearRight => ffi::GST_AUDIO_CHANNEL_POSITION_REAR_RIGHT,
            Self::FrontLeftOfCenter => ffi::GST_AUDIO_CHANNEL_POSITION_FRONT_LEFT_OF_CENTER,
            Self::FrontRightOfCenter => ffi::GST_AUDIO_CHANNEL_POSITION_FRONT_RIGHT_OF_CENTER,
            Self::RearCenter => ffi::GST_AUDIO_CHANNEL_POSITION_REAR_CENTER,
            Self::Lfe2 => ffi::GST_AUDIO_CHANNEL_POSITION_LFE2,
            Self::SideLeft => ffi::GST_AUDIO_CHANNEL_POSITION_SIDE_LEFT,
            Self::SideRight => ffi::GST_AUDIO_CHANNEL_POSITION_SIDE_RIGHT,
            Self::TopFrontLeft => ffi::GST_AUDIO_CHANNEL_POSITION_TOP_FRONT_LEFT,
            Self::TopFrontRight => ffi::GST_AUDIO_CHANNEL_POSITION_TOP_FRONT_RIGHT,
            Self::TopFrontCenter => ffi::GST_AUDIO_CHANNEL_POSITION_TOP_FRONT_CENTER,
            Self::TopCenter => ffi::GST_AUDIO_CHANNEL_POSITION_TOP_CENTER,
            Self::TopRearLeft => ffi::GST_AUDIO_CHANNEL_POSITION_TOP_REAR_LEFT,
            Self::TopRearRight => ffi::GST_AUDIO_CHANNEL_POSITION_TOP_REAR_RIGHT,
            Self::TopSideLeft => ffi::GST_AUDIO_CHANNEL_POSITION_TOP_SIDE_LEFT,
            Self::TopSideRight => ffi::GST_AUDIO_CHANNEL_POSITION_TOP_SIDE_RIGHT,
            Self::TopRearCenter => ffi::GST_AUDIO_CHANNEL_POSITION_TOP_REAR_CENTER,
            Self::BottomFrontCenter => ffi::GST_AUDIO_CHANNEL_POSITION_BOTTOM_FRONT_CENTER,
            Self::BottomFrontLeft => ffi::GST_AUDIO_CHANNEL_POSITION_BOTTOM_FRONT_LEFT,
            Self::BottomFrontRight => ffi::GST_AUDIO_CHANNEL_POSITION_BOTTOM_FRONT_RIGHT,
            Self::WideLeft => ffi::GST_AUDIO_CHANNEL_POSITION_WIDE_LEFT,
            Self::WideRight => ffi::GST_AUDIO_CHANNEL_POSITION_WIDE_RIGHT,
            Self::SurroundLeft => ffi::GST_AUDIO_CHANNEL_POSITION_SURROUND_LEFT,
            Self::SurroundRight => ffi::GST_AUDIO_CHANNEL_POSITION_SURROUND_RIGHT,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioChannelPosition> for AudioChannelPosition {
    unsafe fn from_glib(value: ffi::GstAudioChannelPosition) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_AUDIO_CHANNEL_POSITION_NONE => Self::None,
            ffi::GST_AUDIO_CHANNEL_POSITION_MONO => Self::Mono,
            ffi::GST_AUDIO_CHANNEL_POSITION_INVALID => Self::Invalid,
            ffi::GST_AUDIO_CHANNEL_POSITION_FRONT_LEFT => Self::FrontLeft,
            ffi::GST_AUDIO_CHANNEL_POSITION_FRONT_RIGHT => Self::FrontRight,
            ffi::GST_AUDIO_CHANNEL_POSITION_FRONT_CENTER => Self::FrontCenter,
            ffi::GST_AUDIO_CHANNEL_POSITION_LFE1 => Self::Lfe1,
            ffi::GST_AUDIO_CHANNEL_POSITION_REAR_LEFT => Self::RearLeft,
            ffi::GST_AUDIO_CHANNEL_POSITION_REAR_RIGHT => Self::RearRight,
            ffi::GST_AUDIO_CHANNEL_POSITION_FRONT_LEFT_OF_CENTER => Self::FrontLeftOfCenter,
            ffi::GST_AUDIO_CHANNEL_POSITION_FRONT_RIGHT_OF_CENTER => Self::FrontRightOfCenter,
            ffi::GST_AUDIO_CHANNEL_POSITION_REAR_CENTER => Self::RearCenter,
            ffi::GST_AUDIO_CHANNEL_POSITION_LFE2 => Self::Lfe2,
            ffi::GST_AUDIO_CHANNEL_POSITION_SIDE_LEFT => Self::SideLeft,
            ffi::GST_AUDIO_CHANNEL_POSITION_SIDE_RIGHT => Self::SideRight,
            ffi::GST_AUDIO_CHANNEL_POSITION_TOP_FRONT_LEFT => Self::TopFrontLeft,
            ffi::GST_AUDIO_CHANNEL_POSITION_TOP_FRONT_RIGHT => Self::TopFrontRight,
            ffi::GST_AUDIO_CHANNEL_POSITION_TOP_FRONT_CENTER => Self::TopFrontCenter,
            ffi::GST_AUDIO_CHANNEL_POSITION_TOP_CENTER => Self::TopCenter,
            ffi::GST_AUDIO_CHANNEL_POSITION_TOP_REAR_LEFT => Self::TopRearLeft,
            ffi::GST_AUDIO_CHANNEL_POSITION_TOP_REAR_RIGHT => Self::TopRearRight,
            ffi::GST_AUDIO_CHANNEL_POSITION_TOP_SIDE_LEFT => Self::TopSideLeft,
            ffi::GST_AUDIO_CHANNEL_POSITION_TOP_SIDE_RIGHT => Self::TopSideRight,
            ffi::GST_AUDIO_CHANNEL_POSITION_TOP_REAR_CENTER => Self::TopRearCenter,
            ffi::GST_AUDIO_CHANNEL_POSITION_BOTTOM_FRONT_CENTER => Self::BottomFrontCenter,
            ffi::GST_AUDIO_CHANNEL_POSITION_BOTTOM_FRONT_LEFT => Self::BottomFrontLeft,
            ffi::GST_AUDIO_CHANNEL_POSITION_BOTTOM_FRONT_RIGHT => Self::BottomFrontRight,
            ffi::GST_AUDIO_CHANNEL_POSITION_WIDE_LEFT => Self::WideLeft,
            ffi::GST_AUDIO_CHANNEL_POSITION_WIDE_RIGHT => Self::WideRight,
            ffi::GST_AUDIO_CHANNEL_POSITION_SURROUND_LEFT => Self::SurroundLeft,
            ffi::GST_AUDIO_CHANNEL_POSITION_SURROUND_RIGHT => Self::SurroundRight,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for AudioChannelPosition {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_channel_position_get_type()) }
    }
}

impl glib::value::ValueType for AudioChannelPosition {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AudioChannelPosition {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for AudioChannelPosition {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstAudioDitherMethod")]
pub enum AudioDitherMethod {
    #[doc(alias = "GST_AUDIO_DITHER_NONE")]
    None,
    #[doc(alias = "GST_AUDIO_DITHER_RPDF")]
    Rpdf,
    #[doc(alias = "GST_AUDIO_DITHER_TPDF")]
    Tpdf,
    #[doc(alias = "GST_AUDIO_DITHER_TPDF_HF")]
    TpdfHf,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for AudioDitherMethod {
    type GlibType = ffi::GstAudioDitherMethod;

    fn into_glib(self) -> ffi::GstAudioDitherMethod {
        match self {
            Self::None => ffi::GST_AUDIO_DITHER_NONE,
            Self::Rpdf => ffi::GST_AUDIO_DITHER_RPDF,
            Self::Tpdf => ffi::GST_AUDIO_DITHER_TPDF,
            Self::TpdfHf => ffi::GST_AUDIO_DITHER_TPDF_HF,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioDitherMethod> for AudioDitherMethod {
    unsafe fn from_glib(value: ffi::GstAudioDitherMethod) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_AUDIO_DITHER_NONE => Self::None,
            ffi::GST_AUDIO_DITHER_RPDF => Self::Rpdf,
            ffi::GST_AUDIO_DITHER_TPDF => Self::Tpdf,
            ffi::GST_AUDIO_DITHER_TPDF_HF => Self::TpdfHf,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for AudioDitherMethod {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_dither_method_get_type()) }
    }
}

impl glib::value::ValueType for AudioDitherMethod {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AudioDitherMethod {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for AudioDitherMethod {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstAudioFormat")]
pub enum AudioFormat {
    #[doc(alias = "GST_AUDIO_FORMAT_UNKNOWN")]
    Unknown,
    #[doc(alias = "GST_AUDIO_FORMAT_ENCODED")]
    Encoded,
    #[doc(alias = "GST_AUDIO_FORMAT_S8")]
    S8,
    #[doc(alias = "GST_AUDIO_FORMAT_U8")]
    U8,
    #[doc(alias = "GST_AUDIO_FORMAT_S16LE")]
    S16le,
    #[doc(alias = "GST_AUDIO_FORMAT_S16BE")]
    S16be,
    #[doc(alias = "GST_AUDIO_FORMAT_U16LE")]
    U16le,
    #[doc(alias = "GST_AUDIO_FORMAT_U16BE")]
    U16be,
    #[doc(alias = "GST_AUDIO_FORMAT_S24_32LE")]
    S2432le,
    #[doc(alias = "GST_AUDIO_FORMAT_S24_32BE")]
    S2432be,
    #[doc(alias = "GST_AUDIO_FORMAT_U24_32LE")]
    U2432le,
    #[doc(alias = "GST_AUDIO_FORMAT_U24_32BE")]
    U2432be,
    #[doc(alias = "GST_AUDIO_FORMAT_S32LE")]
    S32le,
    #[doc(alias = "GST_AUDIO_FORMAT_S32BE")]
    S32be,
    #[doc(alias = "GST_AUDIO_FORMAT_U32LE")]
    U32le,
    #[doc(alias = "GST_AUDIO_FORMAT_U32BE")]
    U32be,
    #[doc(alias = "GST_AUDIO_FORMAT_S24LE")]
    S24le,
    #[doc(alias = "GST_AUDIO_FORMAT_S24BE")]
    S24be,
    #[doc(alias = "GST_AUDIO_FORMAT_U24LE")]
    U24le,
    #[doc(alias = "GST_AUDIO_FORMAT_U24BE")]
    U24be,
    #[doc(alias = "GST_AUDIO_FORMAT_S20LE")]
    S20le,
    #[doc(alias = "GST_AUDIO_FORMAT_S20BE")]
    S20be,
    #[doc(alias = "GST_AUDIO_FORMAT_U20LE")]
    U20le,
    #[doc(alias = "GST_AUDIO_FORMAT_U20BE")]
    U20be,
    #[doc(alias = "GST_AUDIO_FORMAT_S18LE")]
    S18le,
    #[doc(alias = "GST_AUDIO_FORMAT_S18BE")]
    S18be,
    #[doc(alias = "GST_AUDIO_FORMAT_U18LE")]
    U18le,
    #[doc(alias = "GST_AUDIO_FORMAT_U18BE")]
    U18be,
    #[doc(alias = "GST_AUDIO_FORMAT_F32LE")]
    F32le,
    #[doc(alias = "GST_AUDIO_FORMAT_F32BE")]
    F32be,
    #[doc(alias = "GST_AUDIO_FORMAT_F64LE")]
    F64le,
    #[doc(alias = "GST_AUDIO_FORMAT_F64BE")]
    F64be,
    #[doc(hidden)]
    __Unknown(i32),
}

impl AudioFormat {
    #[doc(alias = "gst_audio_format_from_string")]
    pub fn from_string(format: &str) -> AudioFormat {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::gst_audio_format_from_string(format.to_glib_none().0)) }
    }
}

impl fmt::Display for AudioFormat {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}

#[doc(hidden)]
impl IntoGlib for AudioFormat {
    type GlibType = ffi::GstAudioFormat;

    fn into_glib(self) -> ffi::GstAudioFormat {
        match self {
            Self::Unknown => ffi::GST_AUDIO_FORMAT_UNKNOWN,
            Self::Encoded => ffi::GST_AUDIO_FORMAT_ENCODED,
            Self::S8 => ffi::GST_AUDIO_FORMAT_S8,
            Self::U8 => ffi::GST_AUDIO_FORMAT_U8,
            Self::S16le => ffi::GST_AUDIO_FORMAT_S16LE,
            Self::S16be => ffi::GST_AUDIO_FORMAT_S16BE,
            Self::U16le => ffi::GST_AUDIO_FORMAT_U16LE,
            Self::U16be => ffi::GST_AUDIO_FORMAT_U16BE,
            Self::S2432le => ffi::GST_AUDIO_FORMAT_S24_32LE,
            Self::S2432be => ffi::GST_AUDIO_FORMAT_S24_32BE,
            Self::U2432le => ffi::GST_AUDIO_FORMAT_U24_32LE,
            Self::U2432be => ffi::GST_AUDIO_FORMAT_U24_32BE,
            Self::S32le => ffi::GST_AUDIO_FORMAT_S32LE,
            Self::S32be => ffi::GST_AUDIO_FORMAT_S32BE,
            Self::U32le => ffi::GST_AUDIO_FORMAT_U32LE,
            Self::U32be => ffi::GST_AUDIO_FORMAT_U32BE,
            Self::S24le => ffi::GST_AUDIO_FORMAT_S24LE,
            Self::S24be => ffi::GST_AUDIO_FORMAT_S24BE,
            Self::U24le => ffi::GST_AUDIO_FORMAT_U24LE,
            Self::U24be => ffi::GST_AUDIO_FORMAT_U24BE,
            Self::S20le => ffi::GST_AUDIO_FORMAT_S20LE,
            Self::S20be => ffi::GST_AUDIO_FORMAT_S20BE,
            Self::U20le => ffi::GST_AUDIO_FORMAT_U20LE,
            Self::U20be => ffi::GST_AUDIO_FORMAT_U20BE,
            Self::S18le => ffi::GST_AUDIO_FORMAT_S18LE,
            Self::S18be => ffi::GST_AUDIO_FORMAT_S18BE,
            Self::U18le => ffi::GST_AUDIO_FORMAT_U18LE,
            Self::U18be => ffi::GST_AUDIO_FORMAT_U18BE,
            Self::F32le => ffi::GST_AUDIO_FORMAT_F32LE,
            Self::F32be => ffi::GST_AUDIO_FORMAT_F32BE,
            Self::F64le => ffi::GST_AUDIO_FORMAT_F64LE,
            Self::F64be => ffi::GST_AUDIO_FORMAT_F64BE,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioFormat> for AudioFormat {
    unsafe fn from_glib(value: ffi::GstAudioFormat) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_AUDIO_FORMAT_UNKNOWN => Self::Unknown,
            ffi::GST_AUDIO_FORMAT_ENCODED => Self::Encoded,
            ffi::GST_AUDIO_FORMAT_S8 => Self::S8,
            ffi::GST_AUDIO_FORMAT_U8 => Self::U8,
            ffi::GST_AUDIO_FORMAT_S16LE => Self::S16le,
            ffi::GST_AUDIO_FORMAT_S16BE => Self::S16be,
            ffi::GST_AUDIO_FORMAT_U16LE => Self::U16le,
            ffi::GST_AUDIO_FORMAT_U16BE => Self::U16be,
            ffi::GST_AUDIO_FORMAT_S24_32LE => Self::S2432le,
            ffi::GST_AUDIO_FORMAT_S24_32BE => Self::S2432be,
            ffi::GST_AUDIO_FORMAT_U24_32LE => Self::U2432le,
            ffi::GST_AUDIO_FORMAT_U24_32BE => Self::U2432be,
            ffi::GST_AUDIO_FORMAT_S32LE => Self::S32le,
            ffi::GST_AUDIO_FORMAT_S32BE => Self::S32be,
            ffi::GST_AUDIO_FORMAT_U32LE => Self::U32le,
            ffi::GST_AUDIO_FORMAT_U32BE => Self::U32be,
            ffi::GST_AUDIO_FORMAT_S24LE => Self::S24le,
            ffi::GST_AUDIO_FORMAT_S24BE => Self::S24be,
            ffi::GST_AUDIO_FORMAT_U24LE => Self::U24le,
            ffi::GST_AUDIO_FORMAT_U24BE => Self::U24be,
            ffi::GST_AUDIO_FORMAT_S20LE => Self::S20le,
            ffi::GST_AUDIO_FORMAT_S20BE => Self::S20be,
            ffi::GST_AUDIO_FORMAT_U20LE => Self::U20le,
            ffi::GST_AUDIO_FORMAT_U20BE => Self::U20be,
            ffi::GST_AUDIO_FORMAT_S18LE => Self::S18le,
            ffi::GST_AUDIO_FORMAT_S18BE => Self::S18be,
            ffi::GST_AUDIO_FORMAT_U18LE => Self::U18le,
            ffi::GST_AUDIO_FORMAT_U18BE => Self::U18be,
            ffi::GST_AUDIO_FORMAT_F32LE => Self::F32le,
            ffi::GST_AUDIO_FORMAT_F32BE => Self::F32be,
            ffi::GST_AUDIO_FORMAT_F64LE => Self::F64le,
            ffi::GST_AUDIO_FORMAT_F64BE => Self::F64be,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for AudioFormat {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_format_get_type()) }
    }
}

impl glib::value::ValueType for AudioFormat {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AudioFormat {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for AudioFormat {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstAudioLayout")]
pub enum AudioLayout {
    #[doc(alias = "GST_AUDIO_LAYOUT_INTERLEAVED")]
    Interleaved,
    #[doc(alias = "GST_AUDIO_LAYOUT_NON_INTERLEAVED")]
    NonInterleaved,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for AudioLayout {
    type GlibType = ffi::GstAudioLayout;

    fn into_glib(self) -> ffi::GstAudioLayout {
        match self {
            Self::Interleaved => ffi::GST_AUDIO_LAYOUT_INTERLEAVED,
            Self::NonInterleaved => ffi::GST_AUDIO_LAYOUT_NON_INTERLEAVED,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioLayout> for AudioLayout {
    unsafe fn from_glib(value: ffi::GstAudioLayout) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_AUDIO_LAYOUT_INTERLEAVED => Self::Interleaved,
            ffi::GST_AUDIO_LAYOUT_NON_INTERLEAVED => Self::NonInterleaved,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for AudioLayout {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_layout_get_type()) }
    }
}

impl glib::value::ValueType for AudioLayout {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AudioLayout {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for AudioLayout {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstAudioNoiseShapingMethod")]
pub enum AudioNoiseShapingMethod {
    #[doc(alias = "GST_AUDIO_NOISE_SHAPING_NONE")]
    None,
    #[doc(alias = "GST_AUDIO_NOISE_SHAPING_ERROR_FEEDBACK")]
    ErrorFeedback,
    #[doc(alias = "GST_AUDIO_NOISE_SHAPING_SIMPLE")]
    Simple,
    #[doc(alias = "GST_AUDIO_NOISE_SHAPING_MEDIUM")]
    Medium,
    #[doc(alias = "GST_AUDIO_NOISE_SHAPING_HIGH")]
    High,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for AudioNoiseShapingMethod {
    type GlibType = ffi::GstAudioNoiseShapingMethod;

    fn into_glib(self) -> ffi::GstAudioNoiseShapingMethod {
        match self {
            Self::None => ffi::GST_AUDIO_NOISE_SHAPING_NONE,
            Self::ErrorFeedback => ffi::GST_AUDIO_NOISE_SHAPING_ERROR_FEEDBACK,
            Self::Simple => ffi::GST_AUDIO_NOISE_SHAPING_SIMPLE,
            Self::Medium => ffi::GST_AUDIO_NOISE_SHAPING_MEDIUM,
            Self::High => ffi::GST_AUDIO_NOISE_SHAPING_HIGH,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioNoiseShapingMethod> for AudioNoiseShapingMethod {
    unsafe fn from_glib(value: ffi::GstAudioNoiseShapingMethod) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_AUDIO_NOISE_SHAPING_NONE => Self::None,
            ffi::GST_AUDIO_NOISE_SHAPING_ERROR_FEEDBACK => Self::ErrorFeedback,
            ffi::GST_AUDIO_NOISE_SHAPING_SIMPLE => Self::Simple,
            ffi::GST_AUDIO_NOISE_SHAPING_MEDIUM => Self::Medium,
            ffi::GST_AUDIO_NOISE_SHAPING_HIGH => Self::High,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for AudioNoiseShapingMethod {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_noise_shaping_method_get_type()) }
    }
}

impl glib::value::ValueType for AudioNoiseShapingMethod {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AudioNoiseShapingMethod {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for AudioNoiseShapingMethod {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstAudioResamplerMethod")]
pub enum AudioResamplerMethod {
    #[doc(alias = "GST_AUDIO_RESAMPLER_METHOD_NEAREST")]
    Nearest,
    #[doc(alias = "GST_AUDIO_RESAMPLER_METHOD_LINEAR")]
    Linear,
    #[doc(alias = "GST_AUDIO_RESAMPLER_METHOD_CUBIC")]
    Cubic,
    #[doc(alias = "GST_AUDIO_RESAMPLER_METHOD_BLACKMAN_NUTTALL")]
    BlackmanNuttall,
    #[doc(alias = "GST_AUDIO_RESAMPLER_METHOD_KAISER")]
    Kaiser,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for AudioResamplerMethod {
    type GlibType = ffi::GstAudioResamplerMethod;

    fn into_glib(self) -> ffi::GstAudioResamplerMethod {
        match self {
            Self::Nearest => ffi::GST_AUDIO_RESAMPLER_METHOD_NEAREST,
            Self::Linear => ffi::GST_AUDIO_RESAMPLER_METHOD_LINEAR,
            Self::Cubic => ffi::GST_AUDIO_RESAMPLER_METHOD_CUBIC,
            Self::BlackmanNuttall => ffi::GST_AUDIO_RESAMPLER_METHOD_BLACKMAN_NUTTALL,
            Self::Kaiser => ffi::GST_AUDIO_RESAMPLER_METHOD_KAISER,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioResamplerMethod> for AudioResamplerMethod {
    unsafe fn from_glib(value: ffi::GstAudioResamplerMethod) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_AUDIO_RESAMPLER_METHOD_NEAREST => Self::Nearest,
            ffi::GST_AUDIO_RESAMPLER_METHOD_LINEAR => Self::Linear,
            ffi::GST_AUDIO_RESAMPLER_METHOD_CUBIC => Self::Cubic,
            ffi::GST_AUDIO_RESAMPLER_METHOD_BLACKMAN_NUTTALL => Self::BlackmanNuttall,
            ffi::GST_AUDIO_RESAMPLER_METHOD_KAISER => Self::Kaiser,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for AudioResamplerMethod {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_resampler_method_get_type()) }
    }
}

impl glib::value::ValueType for AudioResamplerMethod {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AudioResamplerMethod {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for AudioResamplerMethod {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstAudioRingBufferFormatType")]
pub enum AudioRingBufferFormatType {
    #[doc(alias = "GST_AUDIO_RING_BUFFER_FORMAT_TYPE_RAW")]
    Raw,
    #[doc(alias = "GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MU_LAW")]
    MuLaw,
    #[doc(alias = "GST_AUDIO_RING_BUFFER_FORMAT_TYPE_A_LAW")]
    ALaw,
    #[doc(alias = "GST_AUDIO_RING_BUFFER_FORMAT_TYPE_IMA_ADPCM")]
    ImaAdpcm,
    #[doc(alias = "GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG")]
    Mpeg,
    #[doc(alias = "GST_AUDIO_RING_BUFFER_FORMAT_TYPE_GSM")]
    Gsm,
    #[doc(alias = "GST_AUDIO_RING_BUFFER_FORMAT_TYPE_IEC958")]
    Iec958,
    #[doc(alias = "GST_AUDIO_RING_BUFFER_FORMAT_TYPE_AC3")]
    Ac3,
    #[doc(alias = "GST_AUDIO_RING_BUFFER_FORMAT_TYPE_EAC3")]
    Eac3,
    #[doc(alias = "GST_AUDIO_RING_BUFFER_FORMAT_TYPE_DTS")]
    Dts,
    #[doc(alias = "GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG2_AAC")]
    Mpeg2Aac,
    #[doc(alias = "GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG4_AAC")]
    Mpeg4Aac,
    #[doc(alias = "GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG2_AAC_RAW")]
    Mpeg2AacRaw,
    #[doc(alias = "GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG4_AAC_RAW")]
    Mpeg4AacRaw,
    #[doc(alias = "GST_AUDIO_RING_BUFFER_FORMAT_TYPE_FLAC")]
    Flac,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for AudioRingBufferFormatType {
    type GlibType = ffi::GstAudioRingBufferFormatType;

    fn into_glib(self) -> ffi::GstAudioRingBufferFormatType {
        match self {
            Self::Raw => ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_RAW,
            Self::MuLaw => ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MU_LAW,
            Self::ALaw => ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_A_LAW,
            Self::ImaAdpcm => ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_IMA_ADPCM,
            Self::Mpeg => ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG,
            Self::Gsm => ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_GSM,
            Self::Iec958 => ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_IEC958,
            Self::Ac3 => ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_AC3,
            Self::Eac3 => ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_EAC3,
            Self::Dts => ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_DTS,
            Self::Mpeg2Aac => ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG2_AAC,
            Self::Mpeg4Aac => ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG4_AAC,
            Self::Mpeg2AacRaw => ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG2_AAC_RAW,
            Self::Mpeg4AacRaw => ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG4_AAC_RAW,
            Self::Flac => ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_FLAC,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioRingBufferFormatType> for AudioRingBufferFormatType {
    unsafe fn from_glib(value: ffi::GstAudioRingBufferFormatType) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_RAW => Self::Raw,
            ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MU_LAW => Self::MuLaw,
            ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_A_LAW => Self::ALaw,
            ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_IMA_ADPCM => Self::ImaAdpcm,
            ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG => Self::Mpeg,
            ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_GSM => Self::Gsm,
            ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_IEC958 => Self::Iec958,
            ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_AC3 => Self::Ac3,
            ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_EAC3 => Self::Eac3,
            ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_DTS => Self::Dts,
            ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG2_AAC => Self::Mpeg2Aac,
            ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG4_AAC => Self::Mpeg4Aac,
            ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG2_AAC_RAW => Self::Mpeg2AacRaw,
            ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_MPEG4_AAC_RAW => Self::Mpeg4AacRaw,
            ffi::GST_AUDIO_RING_BUFFER_FORMAT_TYPE_FLAC => Self::Flac,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for AudioRingBufferFormatType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_ring_buffer_format_type_get_type()) }
    }
}

impl glib::value::ValueType for AudioRingBufferFormatType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AudioRingBufferFormatType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for AudioRingBufferFormatType {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstStreamVolumeFormat")]
pub enum StreamVolumeFormat {
    #[doc(alias = "GST_STREAM_VOLUME_FORMAT_LINEAR")]
    Linear,
    #[doc(alias = "GST_STREAM_VOLUME_FORMAT_CUBIC")]
    Cubic,
    #[doc(alias = "GST_STREAM_VOLUME_FORMAT_DB")]
    Db,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for StreamVolumeFormat {
    type GlibType = ffi::GstStreamVolumeFormat;

    fn into_glib(self) -> ffi::GstStreamVolumeFormat {
        match self {
            Self::Linear => ffi::GST_STREAM_VOLUME_FORMAT_LINEAR,
            Self::Cubic => ffi::GST_STREAM_VOLUME_FORMAT_CUBIC,
            Self::Db => ffi::GST_STREAM_VOLUME_FORMAT_DB,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstStreamVolumeFormat> for StreamVolumeFormat {
    unsafe fn from_glib(value: ffi::GstStreamVolumeFormat) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_STREAM_VOLUME_FORMAT_LINEAR => Self::Linear,
            ffi::GST_STREAM_VOLUME_FORMAT_CUBIC => Self::Cubic,
            ffi::GST_STREAM_VOLUME_FORMAT_DB => Self::Db,
            value => Self::__Unknown(value),
        }
    }
}
