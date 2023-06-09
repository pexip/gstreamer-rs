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
    #[doc(alias = "GstRTSPEvent")]
    pub struct RTSPEvent: u32 {
        #[doc(alias = "GST_RTSP_EV_READ")]
        const READ = ffi::GST_RTSP_EV_READ as _;
        #[doc(alias = "GST_RTSP_EV_WRITE")]
        const WRITE = ffi::GST_RTSP_EV_WRITE as _;
    }
}

#[doc(hidden)]
impl IntoGlib for RTSPEvent {
    type GlibType = ffi::GstRTSPEvent;

    fn into_glib(self) -> ffi::GstRTSPEvent {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPEvent> for RTSPEvent {
    unsafe fn from_glib(value: ffi::GstRTSPEvent) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for RTSPEvent {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtsp_event_get_type()) }
    }
}

impl glib::value::ValueType for RTSPEvent {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for RTSPEvent {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for RTSPEvent {
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

bitflags! {
    #[doc(alias = "GstRTSPLowerTrans")]
    pub struct RTSPLowerTrans: u32 {
        #[doc(alias = "GST_RTSP_LOWER_TRANS_UDP")]
        const UDP = ffi::GST_RTSP_LOWER_TRANS_UDP as _;
        #[doc(alias = "GST_RTSP_LOWER_TRANS_UDP_MCAST")]
        const UDP_MCAST = ffi::GST_RTSP_LOWER_TRANS_UDP_MCAST as _;
        #[doc(alias = "GST_RTSP_LOWER_TRANS_TCP")]
        const TCP = ffi::GST_RTSP_LOWER_TRANS_TCP as _;
        #[doc(alias = "GST_RTSP_LOWER_TRANS_HTTP")]
        const HTTP = ffi::GST_RTSP_LOWER_TRANS_HTTP as _;
        #[doc(alias = "GST_RTSP_LOWER_TRANS_TLS")]
        const TLS = ffi::GST_RTSP_LOWER_TRANS_TLS as _;
    }
}

#[doc(hidden)]
impl IntoGlib for RTSPLowerTrans {
    type GlibType = ffi::GstRTSPLowerTrans;

    fn into_glib(self) -> ffi::GstRTSPLowerTrans {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPLowerTrans> for RTSPLowerTrans {
    unsafe fn from_glib(value: ffi::GstRTSPLowerTrans) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for RTSPLowerTrans {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtsp_lower_trans_get_type()) }
    }
}

impl glib::value::ValueType for RTSPLowerTrans {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for RTSPLowerTrans {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for RTSPLowerTrans {
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

bitflags! {
    #[doc(alias = "GstRTSPMethod")]
    pub struct RTSPMethod: u32 {
        #[doc(alias = "GST_RTSP_DESCRIBE")]
        const DESCRIBE = ffi::GST_RTSP_DESCRIBE as _;
        #[doc(alias = "GST_RTSP_ANNOUNCE")]
        const ANNOUNCE = ffi::GST_RTSP_ANNOUNCE as _;
        #[doc(alias = "GST_RTSP_GET_PARAMETER")]
        const GET_PARAMETER = ffi::GST_RTSP_GET_PARAMETER as _;
        #[doc(alias = "GST_RTSP_OPTIONS")]
        const OPTIONS = ffi::GST_RTSP_OPTIONS as _;
        #[doc(alias = "GST_RTSP_PAUSE")]
        const PAUSE = ffi::GST_RTSP_PAUSE as _;
        #[doc(alias = "GST_RTSP_PLAY")]
        const PLAY = ffi::GST_RTSP_PLAY as _;
        #[doc(alias = "GST_RTSP_RECORD")]
        const RECORD = ffi::GST_RTSP_RECORD as _;
        #[doc(alias = "GST_RTSP_REDIRECT")]
        const REDIRECT = ffi::GST_RTSP_REDIRECT as _;
        #[doc(alias = "GST_RTSP_SETUP")]
        const SETUP = ffi::GST_RTSP_SETUP as _;
        #[doc(alias = "GST_RTSP_SET_PARAMETER")]
        const SET_PARAMETER = ffi::GST_RTSP_SET_PARAMETER as _;
        #[doc(alias = "GST_RTSP_TEARDOWN")]
        const TEARDOWN = ffi::GST_RTSP_TEARDOWN as _;
        #[doc(alias = "GST_RTSP_GET")]
        const GET = ffi::GST_RTSP_GET as _;
        #[doc(alias = "GST_RTSP_POST")]
        const POST = ffi::GST_RTSP_POST as _;
    }
}

impl RTSPMethod {
    #[doc(alias = "gst_rtsp_method_as_text")]
    pub fn as_text(self) -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gst_rtsp_method_as_text(self.into_glib())) }
    }
}

#[doc(hidden)]
impl IntoGlib for RTSPMethod {
    type GlibType = ffi::GstRTSPMethod;

    fn into_glib(self) -> ffi::GstRTSPMethod {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPMethod> for RTSPMethod {
    unsafe fn from_glib(value: ffi::GstRTSPMethod) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for RTSPMethod {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtsp_method_get_type()) }
    }
}

impl glib::value::ValueType for RTSPMethod {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for RTSPMethod {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for RTSPMethod {
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

bitflags! {
    #[doc(alias = "GstRTSPProfile")]
    pub struct RTSPProfile: u32 {
        #[doc(alias = "GST_RTSP_PROFILE_AVP")]
        const AVP = ffi::GST_RTSP_PROFILE_AVP as _;
        #[doc(alias = "GST_RTSP_PROFILE_SAVP")]
        const SAVP = ffi::GST_RTSP_PROFILE_SAVP as _;
        #[doc(alias = "GST_RTSP_PROFILE_AVPF")]
        const AVPF = ffi::GST_RTSP_PROFILE_AVPF as _;
        #[doc(alias = "GST_RTSP_PROFILE_SAVPF")]
        const SAVPF = ffi::GST_RTSP_PROFILE_SAVPF as _;
    }
}

#[doc(hidden)]
impl IntoGlib for RTSPProfile {
    type GlibType = ffi::GstRTSPProfile;

    fn into_glib(self) -> ffi::GstRTSPProfile {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPProfile> for RTSPProfile {
    unsafe fn from_glib(value: ffi::GstRTSPProfile) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for RTSPProfile {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtsp_profile_get_type()) }
    }
}

impl glib::value::ValueType for RTSPProfile {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for RTSPProfile {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for RTSPProfile {
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

bitflags! {
    #[doc(alias = "GstRTSPTransMode")]
    pub struct RTSPTransMode: u32 {
        #[doc(alias = "GST_RTSP_TRANS_RTP")]
        const RTP = ffi::GST_RTSP_TRANS_RTP as _;
        #[doc(alias = "GST_RTSP_TRANS_RDT")]
        const RDT = ffi::GST_RTSP_TRANS_RDT as _;
    }
}

#[doc(hidden)]
impl IntoGlib for RTSPTransMode {
    type GlibType = ffi::GstRTSPTransMode;

    fn into_glib(self) -> ffi::GstRTSPTransMode {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPTransMode> for RTSPTransMode {
    unsafe fn from_glib(value: ffi::GstRTSPTransMode) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for RTSPTransMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtsp_trans_mode_get_type()) }
    }
}

impl glib::value::ValueType for RTSPTransMode {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for RTSPTransMode {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for RTSPTransMode {
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
