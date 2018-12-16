// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::image_ffi::RUImage;
#[allow(unused_imports)]
use auto::mime_data_ffi::RUMimeData;
#[allow(unused_imports)]
use auto::object_ffi::*;
#[allow(unused_imports)]
use auto::pixmap_ffi::RUPixmap;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUClipboardFuncs {
    pub clear: extern "C" fn(self_c: *const RUBase, mode: i32),
    pub supports_selection: extern "C" fn(self_c: *const RUBase) -> bool,
    pub supports_find_buffer: extern "C" fn(self_c: *const RUBase) -> bool,
    pub owns_selection: extern "C" fn(self_c: *const RUBase) -> bool,
    pub owns_clipboard: extern "C" fn(self_c: *const RUBase) -> bool,
    pub owns_find_buffer: extern "C" fn(self_c: *const RUBase) -> bool,
    pub text: extern "C" fn(self_c: *const RUBase, mode: i32) -> *const ::std::os::raw::c_char,
    pub set_text:
        extern "C" fn(self_c: *const RUBase, arg0: *const ::std::os::raw::c_char, mode: i32),
    pub mime_data: extern "C" fn(self_c: *const RUBase, mode: i32) -> RUMimeData,
    pub set_mime_data: extern "C" fn(self_c: *const RUBase, data: *const RUBase, mode: i32),
    pub image: extern "C" fn(self_c: *const RUBase, mode: i32) -> RUImage,
    pub pixmap: extern "C" fn(self_c: *const RUBase, mode: i32) -> RUPixmap,
    pub set_image: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase, mode: i32),
    pub set_pixmap: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase, mode: i32),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUClipboardAllFuncs {
    pub object_funcs: *const RUObjectFuncs,
    pub clipboard_funcs: *const RUClipboardFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUClipboard {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUClipboardAllFuncs,
}