// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::PlayMediaInfo;
use glib::error::ErrorDomain;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::Quark;
use glib::StaticType;
use glib::Type;
use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::ptr;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstPlayColorBalanceType")]
pub enum PlayColorBalanceType {
    #[doc(alias = "GST_PLAY_COLOR_BALANCE_HUE")]
    Hue,
    #[doc(alias = "GST_PLAY_COLOR_BALANCE_BRIGHTNESS")]
    Brightness,
    #[doc(alias = "GST_PLAY_COLOR_BALANCE_SATURATION")]
    Saturation,
    #[doc(alias = "GST_PLAY_COLOR_BALANCE_CONTRAST")]
    Contrast,
    #[doc(hidden)]
    __Unknown(i32),
}

impl PlayColorBalanceType {
    pub fn name<'a>(self) -> &'a str {
        unsafe {
            CStr::from_ptr(
                ffi::gst_play_color_balance_type_get_name(self.into_glib())
                    .as_ref()
                    .expect("gst_play_color_balance_type_get_name returned NULL"),
            )
            .to_str()
            .expect("gst_play_color_balance_type_get_name returned an invalid string")
        }
    }
}

impl fmt::Display for PlayColorBalanceType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}

#[doc(hidden)]
impl IntoGlib for PlayColorBalanceType {
    type GlibType = ffi::GstPlayColorBalanceType;

    fn into_glib(self) -> ffi::GstPlayColorBalanceType {
        match self {
            Self::Hue => ffi::GST_PLAY_COLOR_BALANCE_HUE,
            Self::Brightness => ffi::GST_PLAY_COLOR_BALANCE_BRIGHTNESS,
            Self::Saturation => ffi::GST_PLAY_COLOR_BALANCE_SATURATION,
            Self::Contrast => ffi::GST_PLAY_COLOR_BALANCE_CONTRAST,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPlayColorBalanceType> for PlayColorBalanceType {
    unsafe fn from_glib(value: ffi::GstPlayColorBalanceType) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_PLAY_COLOR_BALANCE_HUE => Self::Hue,
            ffi::GST_PLAY_COLOR_BALANCE_BRIGHTNESS => Self::Brightness,
            ffi::GST_PLAY_COLOR_BALANCE_SATURATION => Self::Saturation,
            ffi::GST_PLAY_COLOR_BALANCE_CONTRAST => Self::Contrast,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for PlayColorBalanceType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_play_color_balance_type_get_type()) }
    }
}

impl glib::value::ValueType for PlayColorBalanceType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PlayColorBalanceType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for PlayColorBalanceType {
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
#[doc(alias = "GstPlayError")]
pub enum PlayError {
    #[doc(alias = "GST_PLAY_ERROR_FAILED")]
    Failed,
    #[doc(hidden)]
    __Unknown(i32),
}

impl PlayError {
    pub fn name<'a>(self) -> &'a str {
        unsafe {
            CStr::from_ptr(
                ffi::gst_play_error_get_name(self.into_glib())
                    .as_ref()
                    .expect("gst_play_error_get_name returned NULL"),
            )
            .to_str()
            .expect("gst_play_error_get_name returned an invalid string")
        }
    }
}

impl fmt::Display for PlayError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}

#[doc(hidden)]
impl IntoGlib for PlayError {
    type GlibType = ffi::GstPlayError;

    fn into_glib(self) -> ffi::GstPlayError {
        match self {
            Self::Failed => ffi::GST_PLAY_ERROR_FAILED,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPlayError> for PlayError {
    unsafe fn from_glib(value: ffi::GstPlayError) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_PLAY_ERROR_FAILED => Self::Failed,
            value => Self::__Unknown(value),
        }
    }
}

impl ErrorDomain for PlayError {
    fn domain() -> Quark {
        skip_assert_initialized!();

        unsafe { from_glib(ffi::gst_play_error_quark()) }
    }

    fn code(self) -> i32 {
        self.into_glib()
    }

    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match code {
            ffi::GST_PLAY_ERROR_FAILED => Some(Self::Failed),
            _ => Some(Self::Failed),
        }
    }
}

impl StaticType for PlayError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_play_error_get_type()) }
    }
}

impl glib::value::ValueType for PlayError {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PlayError {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for PlayError {
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
#[doc(alias = "GstPlayMessage")]
pub(crate) enum PlayMessage {
    #[doc(alias = "GST_PLAY_MESSAGE_URI_LOADED")]
    UriLoaded,
    #[doc(alias = "GST_PLAY_MESSAGE_POSITION_UPDATED")]
    PositionUpdated,
    #[doc(alias = "GST_PLAY_MESSAGE_DURATION_CHANGED")]
    DurationChanged,
    #[doc(alias = "GST_PLAY_MESSAGE_STATE_CHANGED")]
    StateChanged,
    #[doc(alias = "GST_PLAY_MESSAGE_BUFFERING")]
    Buffering,
    #[doc(alias = "GST_PLAY_MESSAGE_END_OF_STREAM")]
    EndOfStream,
    #[doc(alias = "GST_PLAY_MESSAGE_ERROR")]
    Error,
    #[doc(alias = "GST_PLAY_MESSAGE_WARNING")]
    Warning,
    #[doc(alias = "GST_PLAY_MESSAGE_VIDEO_DIMENSIONS_CHANGED")]
    VideoDimensionsChanged,
    #[doc(alias = "GST_PLAY_MESSAGE_MEDIA_INFO_UPDATED")]
    MediaInfoUpdated,
    #[doc(alias = "GST_PLAY_MESSAGE_VOLUME_CHANGED")]
    VolumeChanged,
    #[doc(alias = "GST_PLAY_MESSAGE_MUTE_CHANGED")]
    MuteChanged,
    #[doc(alias = "GST_PLAY_MESSAGE_SEEK_DONE")]
    SeekDone,
    #[doc(hidden)]
    __Unknown(i32),
}

impl PlayMessage {
    pub fn name<'a>(self) -> &'a str {
        unsafe {
            CStr::from_ptr(
                ffi::gst_play_message_get_name(self.into_glib())
                    .as_ref()
                    .expect("gst_play_message_get_name returned NULL"),
            )
            .to_str()
            .expect("gst_play_message_get_name returned an invalid string")
        }
    }

    #[doc(alias = "gst_play_message_parse_buffering_percent")]
    pub fn parse_buffering_percent(msg: &gst::Message) -> u32 {
        assert_initialized_main_thread!();
        unsafe {
            let mut percent = mem::MaybeUninit::uninit();
            ffi::gst_play_message_parse_buffering_percent(
                msg.to_glib_none().0,
                percent.as_mut_ptr(),
            );
            percent.assume_init()
        }
    }

    #[doc(alias = "gst_play_message_parse_duration_updated")]
    pub fn parse_duration_updated(msg: &gst::Message) -> Option<gst::ClockTime> {
        assert_initialized_main_thread!();
        unsafe {
            let mut duration = mem::MaybeUninit::uninit();
            ffi::gst_play_message_parse_duration_updated(
                msg.to_glib_none().0,
                duration.as_mut_ptr(),
            );
            from_glib(duration.assume_init())
        }
    }

    #[doc(alias = "gst_play_message_parse_error")]
    pub fn parse_error(msg: &gst::Message) -> (glib::Error, Option<gst::Structure>) {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let mut details = ptr::null_mut();
            ffi::gst_play_message_parse_error(msg.to_glib_none().0, &mut error, &mut details);
            (from_glib_full(error), from_glib_full(details))
        }
    }

    #[doc(alias = "gst_play_message_parse_media_info_updated")]
    pub fn parse_media_info_updated(msg: &gst::Message) -> PlayMediaInfo {
        assert_initialized_main_thread!();
        unsafe {
            let mut info = ptr::null_mut();
            ffi::gst_play_message_parse_media_info_updated(msg.to_glib_none().0, &mut info);
            from_glib_full(info)
        }
    }

    #[doc(alias = "gst_play_message_parse_muted_changed")]
    pub fn parse_muted_changed(msg: &gst::Message) -> bool {
        assert_initialized_main_thread!();
        unsafe {
            let mut muted = mem::MaybeUninit::uninit();
            ffi::gst_play_message_parse_muted_changed(msg.to_glib_none().0, muted.as_mut_ptr());
            from_glib(muted.assume_init())
        }
    }

    #[doc(alias = "gst_play_message_parse_position_updated")]
    pub fn parse_position_updated(msg: &gst::Message) -> Option<gst::ClockTime> {
        assert_initialized_main_thread!();
        unsafe {
            let mut position = mem::MaybeUninit::uninit();
            ffi::gst_play_message_parse_position_updated(
                msg.to_glib_none().0,
                position.as_mut_ptr(),
            );
            from_glib(position.assume_init())
        }
    }

    #[doc(alias = "gst_play_message_parse_state_changed")]
    pub fn parse_state_changed(msg: &gst::Message) -> PlayState {
        assert_initialized_main_thread!();
        unsafe {
            let mut state = mem::MaybeUninit::uninit();
            ffi::gst_play_message_parse_state_changed(msg.to_glib_none().0, state.as_mut_ptr());
            from_glib(state.assume_init())
        }
    }

    #[doc(alias = "gst_play_message_parse_type")]
    pub fn parse_type(msg: &gst::Message) -> PlayMessage {
        assert_initialized_main_thread!();
        unsafe {
            let mut type_ = mem::MaybeUninit::uninit();
            ffi::gst_play_message_parse_type(msg.to_glib_none().0, type_.as_mut_ptr());
            from_glib(type_.assume_init())
        }
    }

    #[doc(alias = "gst_play_message_parse_video_dimensions_changed")]
    pub fn parse_video_dimensions_changed(msg: &gst::Message) -> (u32, u32) {
        assert_initialized_main_thread!();
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::gst_play_message_parse_video_dimensions_changed(
                msg.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (width.assume_init(), height.assume_init())
        }
    }

    #[doc(alias = "gst_play_message_parse_volume_changed")]
    pub fn parse_volume_changed(msg: &gst::Message) -> f64 {
        assert_initialized_main_thread!();
        unsafe {
            let mut volume = mem::MaybeUninit::uninit();
            ffi::gst_play_message_parse_volume_changed(msg.to_glib_none().0, volume.as_mut_ptr());
            volume.assume_init()
        }
    }

    #[doc(alias = "gst_play_message_parse_warning")]
    pub fn parse_warning(msg: &gst::Message) -> (glib::Error, Option<gst::Structure>) {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let mut details = ptr::null_mut();
            ffi::gst_play_message_parse_warning(msg.to_glib_none().0, &mut error, &mut details);
            (from_glib_full(error), from_glib_full(details))
        }
    }
}

impl fmt::Display for PlayMessage {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}

#[doc(hidden)]
impl IntoGlib for PlayMessage {
    type GlibType = ffi::GstPlayMessage;

    fn into_glib(self) -> ffi::GstPlayMessage {
        match self {
            Self::UriLoaded => ffi::GST_PLAY_MESSAGE_URI_LOADED,
            Self::PositionUpdated => ffi::GST_PLAY_MESSAGE_POSITION_UPDATED,
            Self::DurationChanged => ffi::GST_PLAY_MESSAGE_DURATION_CHANGED,
            Self::StateChanged => ffi::GST_PLAY_MESSAGE_STATE_CHANGED,
            Self::Buffering => ffi::GST_PLAY_MESSAGE_BUFFERING,
            Self::EndOfStream => ffi::GST_PLAY_MESSAGE_END_OF_STREAM,
            Self::Error => ffi::GST_PLAY_MESSAGE_ERROR,
            Self::Warning => ffi::GST_PLAY_MESSAGE_WARNING,
            Self::VideoDimensionsChanged => ffi::GST_PLAY_MESSAGE_VIDEO_DIMENSIONS_CHANGED,
            Self::MediaInfoUpdated => ffi::GST_PLAY_MESSAGE_MEDIA_INFO_UPDATED,
            Self::VolumeChanged => ffi::GST_PLAY_MESSAGE_VOLUME_CHANGED,
            Self::MuteChanged => ffi::GST_PLAY_MESSAGE_MUTE_CHANGED,
            Self::SeekDone => ffi::GST_PLAY_MESSAGE_SEEK_DONE,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPlayMessage> for PlayMessage {
    unsafe fn from_glib(value: ffi::GstPlayMessage) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_PLAY_MESSAGE_URI_LOADED => Self::UriLoaded,
            ffi::GST_PLAY_MESSAGE_POSITION_UPDATED => Self::PositionUpdated,
            ffi::GST_PLAY_MESSAGE_DURATION_CHANGED => Self::DurationChanged,
            ffi::GST_PLAY_MESSAGE_STATE_CHANGED => Self::StateChanged,
            ffi::GST_PLAY_MESSAGE_BUFFERING => Self::Buffering,
            ffi::GST_PLAY_MESSAGE_END_OF_STREAM => Self::EndOfStream,
            ffi::GST_PLAY_MESSAGE_ERROR => Self::Error,
            ffi::GST_PLAY_MESSAGE_WARNING => Self::Warning,
            ffi::GST_PLAY_MESSAGE_VIDEO_DIMENSIONS_CHANGED => Self::VideoDimensionsChanged,
            ffi::GST_PLAY_MESSAGE_MEDIA_INFO_UPDATED => Self::MediaInfoUpdated,
            ffi::GST_PLAY_MESSAGE_VOLUME_CHANGED => Self::VolumeChanged,
            ffi::GST_PLAY_MESSAGE_MUTE_CHANGED => Self::MuteChanged,
            ffi::GST_PLAY_MESSAGE_SEEK_DONE => Self::SeekDone,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for PlayMessage {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_play_message_get_type()) }
    }
}

impl glib::value::ValueType for PlayMessage {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PlayMessage {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for PlayMessage {
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
#[doc(alias = "GstPlaySnapshotFormat")]
pub enum PlaySnapshotFormat {
    #[doc(alias = "GST_PLAY_THUMBNAIL_RAW_NATIVE")]
    RawNative,
    #[doc(alias = "GST_PLAY_THUMBNAIL_RAW_xRGB")]
    RawXrgb,
    #[doc(alias = "GST_PLAY_THUMBNAIL_RAW_BGRx")]
    RawBgrx,
    #[doc(alias = "GST_PLAY_THUMBNAIL_JPG")]
    Jpg,
    #[doc(alias = "GST_PLAY_THUMBNAIL_PNG")]
    Png,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for PlaySnapshotFormat {
    type GlibType = ffi::GstPlaySnapshotFormat;

    fn into_glib(self) -> ffi::GstPlaySnapshotFormat {
        match self {
            Self::RawNative => ffi::GST_PLAY_THUMBNAIL_RAW_NATIVE,
            Self::RawXrgb => ffi::GST_PLAY_THUMBNAIL_RAW_xRGB,
            Self::RawBgrx => ffi::GST_PLAY_THUMBNAIL_RAW_BGRx,
            Self::Jpg => ffi::GST_PLAY_THUMBNAIL_JPG,
            Self::Png => ffi::GST_PLAY_THUMBNAIL_PNG,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPlaySnapshotFormat> for PlaySnapshotFormat {
    unsafe fn from_glib(value: ffi::GstPlaySnapshotFormat) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_PLAY_THUMBNAIL_RAW_NATIVE => Self::RawNative,
            ffi::GST_PLAY_THUMBNAIL_RAW_xRGB => Self::RawXrgb,
            ffi::GST_PLAY_THUMBNAIL_RAW_BGRx => Self::RawBgrx,
            ffi::GST_PLAY_THUMBNAIL_JPG => Self::Jpg,
            ffi::GST_PLAY_THUMBNAIL_PNG => Self::Png,
            value => Self::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstPlayState")]
pub enum PlayState {
    #[doc(alias = "GST_PLAY_STATE_STOPPED")]
    Stopped,
    #[doc(alias = "GST_PLAY_STATE_BUFFERING")]
    Buffering,
    #[doc(alias = "GST_PLAY_STATE_PAUSED")]
    Paused,
    #[doc(alias = "GST_PLAY_STATE_PLAYING")]
    Playing,
    #[doc(hidden)]
    __Unknown(i32),
}

impl PlayState {
    pub fn name<'a>(self) -> &'a str {
        unsafe {
            CStr::from_ptr(
                ffi::gst_play_state_get_name(self.into_glib())
                    .as_ref()
                    .expect("gst_play_state_get_name returned NULL"),
            )
            .to_str()
            .expect("gst_play_state_get_name returned an invalid string")
        }
    }
}

impl fmt::Display for PlayState {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}

#[doc(hidden)]
impl IntoGlib for PlayState {
    type GlibType = ffi::GstPlayState;

    fn into_glib(self) -> ffi::GstPlayState {
        match self {
            Self::Stopped => ffi::GST_PLAY_STATE_STOPPED,
            Self::Buffering => ffi::GST_PLAY_STATE_BUFFERING,
            Self::Paused => ffi::GST_PLAY_STATE_PAUSED,
            Self::Playing => ffi::GST_PLAY_STATE_PLAYING,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPlayState> for PlayState {
    unsafe fn from_glib(value: ffi::GstPlayState) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_PLAY_STATE_STOPPED => Self::Stopped,
            ffi::GST_PLAY_STATE_BUFFERING => Self::Buffering,
            ffi::GST_PLAY_STATE_PAUSED => Self::Paused,
            ffi::GST_PLAY_STATE_PLAYING => Self::Playing,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for PlayState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_play_state_get_type()) }
    }
}

impl glib::value::ValueType for PlayState {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PlayState {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for PlayState {
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
