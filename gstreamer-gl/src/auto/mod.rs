// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_16", feature = "dox"))]
mod gl_base_filter;
#[cfg(any(feature = "v1_16", feature = "dox"))]
pub use self::gl_base_filter::GLBaseFilterExt;
#[cfg(any(feature = "v1_16", feature = "dox"))]
pub use self::gl_base_filter::{GLBaseFilter, GLBaseFilterClass, NONE_GL_BASE_FILTER};

mod gl_color_convert;
pub use self::gl_color_convert::{GLColorConvert, GLColorConvertClass};

mod gl_context;
pub use self::gl_context::GLContextExt;
pub use self::gl_context::{GLContext, GLContextClass, NONE_GL_CONTEXT};

mod gl_display;
pub use self::gl_display::GLDisplayExt;
pub use self::gl_display::{GLDisplay, GLDisplayClass, NONE_GL_DISPLAY};

#[cfg(any(feature = "egl", feature = "dox"))]
mod gl_display_egl;
#[cfg(any(feature = "egl", feature = "dox"))]
pub use self::gl_display_egl::{GLDisplayEGL, GLDisplayEGLClass};

#[cfg(any(feature = "wayland", feature = "dox"))]
mod gl_display_wayland;
#[cfg(any(feature = "wayland", feature = "dox"))]
pub use self::gl_display_wayland::{GLDisplayWayland, GLDisplayWaylandClass};

#[cfg(any(feature = "x11", feature = "dox"))]
mod gl_display_x11;
#[cfg(any(feature = "x11", feature = "dox"))]
pub use self::gl_display_x11::{GLDisplayX11, GLDisplayX11Class};

mod gl_framebuffer;
pub use self::gl_framebuffer::GLFramebufferExt;
pub use self::gl_framebuffer::{GLFramebuffer, GLFramebufferClass, NONE_GL_FRAMEBUFFER};

mod gl_overlay_compositor;
pub use self::gl_overlay_compositor::{GLOverlayCompositor, GLOverlayCompositorClass};

mod glsl_stage;
pub use self::glsl_stage::{GLSLStage, GLSLStageClass};

mod gl_shader;
pub use self::gl_shader::{GLShader, GLShaderClass};

mod gl_upload;
pub use self::gl_upload::{GLUpload, GLUploadClass};

mod gl_view_convert;
pub use self::gl_view_convert::{GLViewConvert, GLViewConvertClass};

mod gl_window;
pub use self::gl_window::GLWindowExt;
pub use self::gl_window::{GLWindow, GLWindowClass, NONE_GL_WINDOW};

mod enums;
pub use self::enums::GLContextError;
pub use self::enums::GLFormat;
pub use self::enums::GLQueryType;
pub use self::enums::GLSLError;
pub use self::enums::GLSLVersion;
pub use self::enums::GLStereoDownmix;
pub use self::enums::GLTextureTarget;
pub use self::enums::GLUploadReturn;
pub use self::enums::GLWindowError;

mod flags;
pub use self::flags::GLDisplayType;
pub use self::flags::GLPlatform;
pub use self::flags::GLSLProfile;
pub use self::flags::GLAPI;

#[doc(hidden)]
pub mod traits {
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub use super::GLBaseFilterExt;
    pub use super::GLContextExt;
    pub use super::GLDisplayExt;
    pub use super::GLFramebufferExt;
    pub use super::GLWindowExt;
}
