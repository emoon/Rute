// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::brush_ffi::RUBrush;
#[allow(unused_imports)]
use auto::color_ffi::RUColor;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUPaletteFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub swap: extern "C" fn(self_c: *const RUBase, other: *const RUBase),
    pub current_color_group: extern "C" fn(self_c: *const RUBase) -> u32,
    pub set_current_color_group: extern "C" fn(self_c: *const RUBase, cg: u32),
    pub color: extern "C" fn(self_c: *const RUBase, cg: u32, cr: u32) -> RUColor,
    pub brush: extern "C" fn(self_c: *const RUBase, cg: u32, cr: u32) -> RUBrush,
    pub set_color: extern "C" fn(self_c: *const RUBase, cg: u32, cr: u32, color: *const RUBase),
    pub set_color_2: extern "C" fn(self_c: *const RUBase, cr: u32, color: *const RUBase),
    pub set_brush: extern "C" fn(self_c: *const RUBase, cr: u32, brush: *const RUBase),
    pub is_brush_set: extern "C" fn(self_c: *const RUBase, cg: u32, cr: u32) -> bool,
    pub set_brush_2: extern "C" fn(self_c: *const RUBase, cg: u32, cr: u32, brush: *const RUBase),
    pub set_color_group: extern "C" fn(
        self_c: *const RUBase,
        cr: u32,
        window_text: *const RUBase,
        button: *const RUBase,
        light: *const RUBase,
        dark: *const RUBase,
        mid: *const RUBase,
        text: *const RUBase,
        bright_text: *const RUBase,
        base: *const RUBase,
        window: *const RUBase,
    ),
    pub is_equal: extern "C" fn(self_c: *const RUBase, cr1: u32, cr2: u32) -> bool,
    pub color_2: extern "C" fn(self_c: *const RUBase, cr: u32) -> RUColor,
    pub brush_2: extern "C" fn(self_c: *const RUBase, cr: u32) -> RUBrush,
    pub foreground: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub window_text: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub button: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub light: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub dark: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub mid: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub text: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub base: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub alternate_base: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub tool_tip_base: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub tool_tip_text: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub background: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub window: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub midlight: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub bright_text: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub button_text: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub shadow: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub highlight: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub highlighted_text: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub link: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub link_visited: extern "C" fn(self_c: *const RUBase) -> RUBrush,
    pub is_copy_of: extern "C" fn(self_c: *const RUBase, p: *const RUBase) -> bool,
    pub cache_key: extern "C" fn(self_c: *const RUBase) -> i64,
    pub resolve: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase) -> RUPalette,
    pub resolve_2: extern "C" fn(self_c: *const RUBase) -> u32,
    pub resolve_3: extern "C" fn(self_c: *const RUBase, mask: u32),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUPaletteAllFuncs {
    pub palette_funcs: *const RUPaletteFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUPalette {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUPaletteAllFuncs,
}
