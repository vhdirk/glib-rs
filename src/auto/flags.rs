// This file was generated by gir (ac9a8da) from gir-files (0bcaef9)
// DO NOT EDIT

use ffi;
use translate::*;

bitflags! {
    pub struct FormatSizeFlags: u32 {
        const FORMAT_SIZE_DEFAULT = 0;
        const FORMAT_SIZE_LONG_FORMAT = 1;
        const FORMAT_SIZE_IEC_UNITS = 2;
    }
}

#[doc(hidden)]
impl ToGlib for FormatSizeFlags {
    type GlibType = ffi::GFormatSizeFlags;

    fn to_glib(&self) -> ffi::GFormatSizeFlags {
        ffi::GFormatSizeFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GFormatSizeFlags> for FormatSizeFlags {
    fn from_glib(value: ffi::GFormatSizeFlags) -> FormatSizeFlags {
        FormatSizeFlags::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub struct KeyFileFlags: u32 {
        const KEY_FILE_NONE = 0;
        const KEY_FILE_KEEP_COMMENTS = 1;
        const KEY_FILE_KEEP_TRANSLATIONS = 2;
    }
}

#[doc(hidden)]
impl ToGlib for KeyFileFlags {
    type GlibType = ffi::GKeyFileFlags;

    fn to_glib(&self) -> ffi::GKeyFileFlags {
        ffi::GKeyFileFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GKeyFileFlags> for KeyFileFlags {
    fn from_glib(value: ffi::GKeyFileFlags) -> KeyFileFlags {
        KeyFileFlags::from_bits_truncate(value.bits())
    }
}
