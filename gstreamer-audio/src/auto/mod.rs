// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod stream_volume;
pub use self::stream_volume::StreamVolumeExt;
pub use self::stream_volume::{StreamVolume, NONE_STREAM_VOLUME};

#[cfg(any(feature = "v1_14", feature = "dox"))]
mod audio_stream_align;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::audio_stream_align::AudioStreamAlign;

mod enums;
pub use self::enums::AudioChannelPosition;
pub use self::enums::AudioFormat;
pub use self::enums::AudioLayout;
pub use self::enums::StreamVolumeFormat;

mod flags;
pub use self::flags::AudioFlags;
pub use self::flags::AudioFormatFlags;
pub use self::flags::AudioPackFlags;

#[doc(hidden)]
pub mod traits {
    pub use super::StreamVolumeExt;
}
