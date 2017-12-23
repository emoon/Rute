use std::os::raw::{c_void, c_char};

#[repr(C)]
pub struct PURect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[repr(C)]
pub struct PUWidget {
    pub show: extern "C" fn(self_c: *const c_void),
    pub resize: extern "C" fn(self_c: *const c_void, width: i32, height: i32),
    pub set_paint_event_event: extern "C" fn(object: *const c_void, user_data: *const c_void,
                                        callback: extern "C" fn(self_c: *const c_void, event: *const PUPaintEvent)),
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUPushButton {
    pub show: extern "C" fn(self_c: *const c_void),
    pub resize: extern "C" fn(self_c: *const c_void, width: i32, height: i32),
    pub set_released_event: extern "C" fn(object: *const c_void, user_data: *const c_void,
                                        callback: extern "C" fn(self_c: *const c_void)),
    pub set_text: extern "C" fn(self_c: *const c_void, text: *const c_char),
    pub set_flat: extern "C" fn(self_c: *const c_void, flat: bool),
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUSlider {
    pub show: extern "C" fn(self_c: *const c_void),
    pub resize: extern "C" fn(self_c: *const c_void, width: i32, height: i32),
    pub set_value_changed_event: extern "C" fn(object: *const c_void, user_data: *const c_void,
                                        callback: extern "C" fn(self_c: *const c_void, value: i32)),
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUApplication {
    pub set_style: extern "C" fn(self_c: *const c_void, style: *const c_char),
    pub run: extern "C" fn(self_c: *const c_void),
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUPaintEvent {
    pub rect: extern "C" fn(self_c: *const c_void -> *const PURect),
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUPainter {
    pub draw_line: extern "C" fn(self_c: *const c_void, x1: i32, y1: i32, x2: i32, y2: i32),
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PU {
    pub create_widget: extern "C" fn(priv_data: *const c_void) -> *const PUWidget,
    pub create_push_button: extern "C" fn(priv_data: *const c_void) -> *const PUPushButton,
    pub create_slider: extern "C" fn(priv_data: *const c_void) -> *const PUSlider,
    pub create_application: extern "C" fn(priv_data: *const c_void) -> *const PUApplication,
    pub create_paint_event: extern "C" fn(priv_data: *const c_void) -> *const PUPaintEvent,
    pub create_painter: extern "C" fn(priv_data: *const c_void) -> *const PUPainter,
    pub privd: *const c_void,
}

