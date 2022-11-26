// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::error::ErrorDomain;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::Quark;
use glib::StaticType;
use glib::Type;
use std::ffi::CStr;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstPlayerColorBalanceType")]
pub enum PlayerColorBalanceType {
    #[doc(alias = "GST_PLAYER_COLOR_BALANCE_HUE")]
    Hue,
    #[doc(alias = "GST_PLAYER_COLOR_BALANCE_BRIGHTNESS")]
    Brightness,
    #[doc(alias = "GST_PLAYER_COLOR_BALANCE_SATURATION")]
    Saturation,
    #[doc(alias = "GST_PLAYER_COLOR_BALANCE_CONTRAST")]
    Contrast,
    #[doc(hidden)]
    __Unknown(i32),
}

impl PlayerColorBalanceType {
    pub fn name<'a>(self) -> &'a str {
        unsafe {
            CStr::from_ptr(
                ffi::gst_player_color_balance_type_get_name(self.into_glib())
                    .as_ref()
                    .expect("gst_player_color_balance_type_get_name returned NULL"),
            )
            .to_str()
            .expect("gst_player_color_balance_type_get_name returned an invalid string")
        }
    }
}

impl fmt::Display for PlayerColorBalanceType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}

#[doc(hidden)]
impl IntoGlib for PlayerColorBalanceType {
    type GlibType = ffi::GstPlayerColorBalanceType;

    fn into_glib(self) -> ffi::GstPlayerColorBalanceType {
        match self {
            Self::Hue => ffi::GST_PLAYER_COLOR_BALANCE_HUE,
            Self::Brightness => ffi::GST_PLAYER_COLOR_BALANCE_BRIGHTNESS,
            Self::Saturation => ffi::GST_PLAYER_COLOR_BALANCE_SATURATION,
            Self::Contrast => ffi::GST_PLAYER_COLOR_BALANCE_CONTRAST,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPlayerColorBalanceType> for PlayerColorBalanceType {
    unsafe fn from_glib(value: ffi::GstPlayerColorBalanceType) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_PLAYER_COLOR_BALANCE_HUE => Self::Hue,
            ffi::GST_PLAYER_COLOR_BALANCE_BRIGHTNESS => Self::Brightness,
            ffi::GST_PLAYER_COLOR_BALANCE_SATURATION => Self::Saturation,
            ffi::GST_PLAYER_COLOR_BALANCE_CONTRAST => Self::Contrast,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for PlayerColorBalanceType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_player_color_balance_type_get_type()) }
    }
}

impl glib::value::ValueType for PlayerColorBalanceType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PlayerColorBalanceType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for PlayerColorBalanceType {
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

impl From<PlayerColorBalanceType> for glib::Value {
    #[inline]
    fn from(v: PlayerColorBalanceType) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstPlayerError")]
pub enum PlayerError {
    #[doc(alias = "GST_PLAYER_ERROR_FAILED")]
    Failed,
    #[doc(hidden)]
    __Unknown(i32),
}

impl PlayerError {
    pub fn name<'a>(self) -> &'a str {
        unsafe {
            CStr::from_ptr(
                ffi::gst_player_error_get_name(self.into_glib())
                    .as_ref()
                    .expect("gst_player_error_get_name returned NULL"),
            )
            .to_str()
            .expect("gst_player_error_get_name returned an invalid string")
        }
    }
}

impl fmt::Display for PlayerError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}

#[doc(hidden)]
impl IntoGlib for PlayerError {
    type GlibType = ffi::GstPlayerError;

    fn into_glib(self) -> ffi::GstPlayerError {
        match self {
            Self::Failed => ffi::GST_PLAYER_ERROR_FAILED,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPlayerError> for PlayerError {
    unsafe fn from_glib(value: ffi::GstPlayerError) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_PLAYER_ERROR_FAILED => Self::Failed,
            value => Self::__Unknown(value),
        }
    }
}

impl ErrorDomain for PlayerError {
    fn domain() -> Quark {
        skip_assert_initialized!();

        unsafe { from_glib(ffi::gst_player_error_quark()) }
    }

    fn code(self) -> i32 {
        self.into_glib()
    }

    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match code {
            ffi::GST_PLAYER_ERROR_FAILED => Some(Self::Failed),
            _ => Some(Self::Failed),
        }
    }
}

impl StaticType for PlayerError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_player_error_get_type()) }
    }
}

impl glib::value::ValueType for PlayerError {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PlayerError {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for PlayerError {
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

impl From<PlayerError> for glib::Value {
    #[inline]
    fn from(v: PlayerError) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstPlayerSnapshotFormat")]
pub enum PlayerSnapshotFormat {
    #[doc(alias = "GST_PLAYER_THUMBNAIL_RAW_NATIVE")]
    RawNative,
    #[doc(alias = "GST_PLAYER_THUMBNAIL_RAW_xRGB")]
    RawXrgb,
    #[doc(alias = "GST_PLAYER_THUMBNAIL_RAW_BGRx")]
    RawBgrx,
    #[doc(alias = "GST_PLAYER_THUMBNAIL_JPG")]
    Jpg,
    #[doc(alias = "GST_PLAYER_THUMBNAIL_PNG")]
    Png,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for PlayerSnapshotFormat {
    type GlibType = ffi::GstPlayerSnapshotFormat;

    fn into_glib(self) -> ffi::GstPlayerSnapshotFormat {
        match self {
            Self::RawNative => ffi::GST_PLAYER_THUMBNAIL_RAW_NATIVE,
            Self::RawXrgb => ffi::GST_PLAYER_THUMBNAIL_RAW_xRGB,
            Self::RawBgrx => ffi::GST_PLAYER_THUMBNAIL_RAW_BGRx,
            Self::Jpg => ffi::GST_PLAYER_THUMBNAIL_JPG,
            Self::Png => ffi::GST_PLAYER_THUMBNAIL_PNG,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPlayerSnapshotFormat> for PlayerSnapshotFormat {
    unsafe fn from_glib(value: ffi::GstPlayerSnapshotFormat) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_PLAYER_THUMBNAIL_RAW_NATIVE => Self::RawNative,
            ffi::GST_PLAYER_THUMBNAIL_RAW_xRGB => Self::RawXrgb,
            ffi::GST_PLAYER_THUMBNAIL_RAW_BGRx => Self::RawBgrx,
            ffi::GST_PLAYER_THUMBNAIL_JPG => Self::Jpg,
            ffi::GST_PLAYER_THUMBNAIL_PNG => Self::Png,
            value => Self::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstPlayerState")]
pub enum PlayerState {
    #[doc(alias = "GST_PLAYER_STATE_STOPPED")]
    Stopped,
    #[doc(alias = "GST_PLAYER_STATE_BUFFERING")]
    Buffering,
    #[doc(alias = "GST_PLAYER_STATE_PAUSED")]
    Paused,
    #[doc(alias = "GST_PLAYER_STATE_PLAYING")]
    Playing,
    #[doc(hidden)]
    __Unknown(i32),
}

impl PlayerState {
    pub fn name<'a>(self) -> &'a str {
        unsafe {
            CStr::from_ptr(
                ffi::gst_player_state_get_name(self.into_glib())
                    .as_ref()
                    .expect("gst_player_state_get_name returned NULL"),
            )
            .to_str()
            .expect("gst_player_state_get_name returned an invalid string")
        }
    }
}

impl fmt::Display for PlayerState {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}

#[doc(hidden)]
impl IntoGlib for PlayerState {
    type GlibType = ffi::GstPlayerState;

    fn into_glib(self) -> ffi::GstPlayerState {
        match self {
            Self::Stopped => ffi::GST_PLAYER_STATE_STOPPED,
            Self::Buffering => ffi::GST_PLAYER_STATE_BUFFERING,
            Self::Paused => ffi::GST_PLAYER_STATE_PAUSED,
            Self::Playing => ffi::GST_PLAYER_STATE_PLAYING,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPlayerState> for PlayerState {
    unsafe fn from_glib(value: ffi::GstPlayerState) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_PLAYER_STATE_STOPPED => Self::Stopped,
            ffi::GST_PLAYER_STATE_BUFFERING => Self::Buffering,
            ffi::GST_PLAYER_STATE_PAUSED => Self::Paused,
            ffi::GST_PLAYER_STATE_PLAYING => Self::Playing,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for PlayerState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_player_state_get_type()) }
    }
}

impl glib::value::ValueType for PlayerState {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PlayerState {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for PlayerState {
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

impl From<PlayerState> for glib::Value {
    #[inline]
    fn from(v: PlayerState) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
