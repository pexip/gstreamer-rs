// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use bitflags::bitflags;
use glib::{translate::*, value::FromValue, value::ToValue, StaticType, Type};
use std::fmt;

bitflags! {
    #[doc(alias = "GstGLAPI")]
    pub struct GLAPI: u32 {
        #[doc(alias = "GST_GL_API_OPENGL")]
        const OPENGL = ffi::GST_GL_API_OPENGL as _;
        #[doc(alias = "GST_GL_API_OPENGL3")]
        const OPENGL3 = ffi::GST_GL_API_OPENGL3 as _;
        #[doc(alias = "GST_GL_API_GLES1")]
        const GLES1 = ffi::GST_GL_API_GLES1 as _;
        #[doc(alias = "GST_GL_API_GLES2")]
        const GLES2 = ffi::GST_GL_API_GLES2 as _;
    }
}

impl GLAPI {
    #[doc(alias = "gst_gl_api_from_string")]
    pub fn from_string(api_s: &str) -> GLAPI {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::gst_gl_api_from_string(api_s.to_glib_none().0)) }
    }

    #[doc(alias = "gst_gl_api_to_string")]
    #[doc(alias = "to_string")]
    pub fn to_str(self) -> glib::GString {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_gl_api_to_string(self.into_glib())) }
    }
}

impl fmt::Display for GLAPI {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}

#[doc(hidden)]
impl IntoGlib for GLAPI {
    type GlibType = ffi::GstGLAPI;

    #[inline]
    fn into_glib(self) -> ffi::GstGLAPI {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLAPI> for GLAPI {
    #[inline]
    unsafe fn from_glib(value: ffi::GstGLAPI) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for GLAPI {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_gl_api_get_type()) }
    }
}

impl glib::value::ValueType for GLAPI {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for GLAPI {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for GLAPI {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<GLAPI> for glib::Value {
    #[inline]
    fn from(v: GLAPI) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
bitflags! {
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "GstGLConfigSurfaceType")]
    pub struct GLConfigSurfaceType: u32 {
        #[doc(alias = "GST_GL_CONFIG_SURFACE_TYPE_NONE")]
        const NONE = ffi::GST_GL_CONFIG_SURFACE_TYPE_NONE as _;
        #[doc(alias = "GST_GL_CONFIG_SURFACE_TYPE_WINDOW")]
        const WINDOW = ffi::GST_GL_CONFIG_SURFACE_TYPE_WINDOW as _;
        #[doc(alias = "GST_GL_CONFIG_SURFACE_TYPE_PBUFFER")]
        const PBUFFER = ffi::GST_GL_CONFIG_SURFACE_TYPE_PBUFFER as _;
        #[doc(alias = "GST_GL_CONFIG_SURFACE_TYPE_PIXMAP")]
        const PIXMAP = ffi::GST_GL_CONFIG_SURFACE_TYPE_PIXMAP as _;
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl GLConfigSurfaceType {
    #[doc(alias = "gst_gl_config_surface_type_to_string")]
    #[doc(alias = "to_string")]
    pub fn to_str(self) -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gst_gl_config_surface_type_to_string(self.into_glib())) }
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl IntoGlib for GLConfigSurfaceType {
    type GlibType = ffi::GstGLConfigSurfaceType;

    #[inline]
    fn into_glib(self) -> ffi::GstGLConfigSurfaceType {
        self.bits()
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl FromGlib<ffi::GstGLConfigSurfaceType> for GLConfigSurfaceType {
    #[inline]
    unsafe fn from_glib(value: ffi::GstGLConfigSurfaceType) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl StaticType for GLConfigSurfaceType {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_gl_config_surface_type_get_type()) }
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl glib::value::ValueType for GLConfigSurfaceType {
    type Type = Self;
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
unsafe impl<'a> FromValue<'a> for GLConfigSurfaceType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl ToValue for GLConfigSurfaceType {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl From<GLConfigSurfaceType> for glib::Value {
    #[inline]
    fn from(v: GLConfigSurfaceType) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "GstGLDisplayType")]
    pub struct GLDisplayType: u32 {
        #[doc(alias = "GST_GL_DISPLAY_TYPE_X11")]
        const X11 = ffi::GST_GL_DISPLAY_TYPE_X11 as _;
        #[doc(alias = "GST_GL_DISPLAY_TYPE_WAYLAND")]
        const WAYLAND = ffi::GST_GL_DISPLAY_TYPE_WAYLAND as _;
        #[doc(alias = "GST_GL_DISPLAY_TYPE_COCOA")]
        const COCOA = ffi::GST_GL_DISPLAY_TYPE_COCOA as _;
        #[doc(alias = "GST_GL_DISPLAY_TYPE_WIN32")]
        const WIN32 = ffi::GST_GL_DISPLAY_TYPE_WIN32 as _;
        #[doc(alias = "GST_GL_DISPLAY_TYPE_DISPMANX")]
        const DISPMANX = ffi::GST_GL_DISPLAY_TYPE_DISPMANX as _;
        #[doc(alias = "GST_GL_DISPLAY_TYPE_EGL")]
        const EGL = ffi::GST_GL_DISPLAY_TYPE_EGL as _;
        #[doc(alias = "GST_GL_DISPLAY_TYPE_VIV_FB")]
        const VIV_FB = ffi::GST_GL_DISPLAY_TYPE_VIV_FB as _;
        #[doc(alias = "GST_GL_DISPLAY_TYPE_GBM")]
        const GBM = ffi::GST_GL_DISPLAY_TYPE_GBM as _;
        #[cfg(any(feature = "v1_18", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
        #[doc(alias = "GST_GL_DISPLAY_TYPE_EGL_DEVICE")]
        const EGL_DEVICE = ffi::GST_GL_DISPLAY_TYPE_EGL_DEVICE as _;
        #[cfg(any(feature = "v1_20", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
        #[doc(alias = "GST_GL_DISPLAY_TYPE_EAGL")]
        const EAGL = ffi::GST_GL_DISPLAY_TYPE_EAGL as _;
        #[cfg(any(feature = "v1_20", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
        #[doc(alias = "GST_GL_DISPLAY_TYPE_WINRT")]
        const WINRT = ffi::GST_GL_DISPLAY_TYPE_WINRT as _;
        #[cfg(any(feature = "v1_20", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
        #[doc(alias = "GST_GL_DISPLAY_TYPE_ANDROID")]
        const ANDROID = ffi::GST_GL_DISPLAY_TYPE_ANDROID as _;
    }
}

#[doc(hidden)]
impl IntoGlib for GLDisplayType {
    type GlibType = ffi::GstGLDisplayType;

    #[inline]
    fn into_glib(self) -> ffi::GstGLDisplayType {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLDisplayType> for GLDisplayType {
    #[inline]
    unsafe fn from_glib(value: ffi::GstGLDisplayType) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for GLDisplayType {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_gl_display_type_get_type()) }
    }
}

impl glib::value::ValueType for GLDisplayType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for GLDisplayType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for GLDisplayType {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<GLDisplayType> for glib::Value {
    #[inline]
    fn from(v: GLDisplayType) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "GstGLPlatform")]
    pub struct GLPlatform: u32 {
        #[doc(alias = "GST_GL_PLATFORM_EGL")]
        const EGL = ffi::GST_GL_PLATFORM_EGL as _;
        #[doc(alias = "GST_GL_PLATFORM_GLX")]
        const GLX = ffi::GST_GL_PLATFORM_GLX as _;
        #[doc(alias = "GST_GL_PLATFORM_WGL")]
        const WGL = ffi::GST_GL_PLATFORM_WGL as _;
        #[doc(alias = "GST_GL_PLATFORM_CGL")]
        const CGL = ffi::GST_GL_PLATFORM_CGL as _;
        #[doc(alias = "GST_GL_PLATFORM_EAGL")]
        const EAGL = ffi::GST_GL_PLATFORM_EAGL as _;
    }
}

impl GLPlatform {
    #[doc(alias = "gst_gl_platform_from_string")]
    pub fn from_string(platform_s: &str) -> GLPlatform {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::gst_gl_platform_from_string(
                platform_s.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_gl_platform_to_string")]
    #[doc(alias = "to_string")]
    pub fn to_str(self) -> glib::GString {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_gl_platform_to_string(self.into_glib())) }
    }
}

impl fmt::Display for GLPlatform {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}

#[doc(hidden)]
impl IntoGlib for GLPlatform {
    type GlibType = ffi::GstGLPlatform;

    #[inline]
    fn into_glib(self) -> ffi::GstGLPlatform {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLPlatform> for GLPlatform {
    #[inline]
    unsafe fn from_glib(value: ffi::GstGLPlatform) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for GLPlatform {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_gl_platform_get_type()) }
    }
}

impl glib::value::ValueType for GLPlatform {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for GLPlatform {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for GLPlatform {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<GLPlatform> for glib::Value {
    #[inline]
    fn from(v: GLPlatform) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "GstGLSLProfile")]
    pub struct GLSLProfile: u32 {
        #[doc(alias = "GST_GLSL_PROFILE_ES")]
        const ES = ffi::GST_GLSL_PROFILE_ES as _;
        #[doc(alias = "GST_GLSL_PROFILE_CORE")]
        const CORE = ffi::GST_GLSL_PROFILE_CORE as _;
        #[doc(alias = "GST_GLSL_PROFILE_COMPATIBILITY")]
        const COMPATIBILITY = ffi::GST_GLSL_PROFILE_COMPATIBILITY as _;
    }
}

impl GLSLProfile {
    #[doc(alias = "gst_glsl_profile_from_string")]
    pub fn from_string(string: &str) -> GLSLProfile {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::gst_glsl_profile_from_string(string.to_glib_none().0)) }
    }

    #[doc(alias = "gst_glsl_profile_to_string")]
    #[doc(alias = "to_string")]
    pub fn to_str(self) -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gst_glsl_profile_to_string(self.into_glib())) }
    }
}

#[doc(hidden)]
impl IntoGlib for GLSLProfile {
    type GlibType = ffi::GstGLSLProfile;

    #[inline]
    fn into_glib(self) -> ffi::GstGLSLProfile {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLSLProfile> for GLSLProfile {
    #[inline]
    unsafe fn from_glib(value: ffi::GstGLSLProfile) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for GLSLProfile {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_glsl_profile_get_type()) }
    }
}

impl glib::value::ValueType for GLSLProfile {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for GLSLProfile {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for GLSLProfile {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<GLSLProfile> for glib::Value {
    #[inline]
    fn from(v: GLSLProfile) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
