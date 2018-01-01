use std::os::raw::c_void;

#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct PUPaintDevice {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct PUWidgetType {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct PURect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[repr(C)]
pub struct PUWidget {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub show: extern "C" fn(self_c: *const ::std::os::raw::c_void),
    pub resize: extern "C" fn(self_c: *const ::std::os::raw::c_void, width: i32, height: i32),
    pub set_paint_event_event: extern "C" fn(object: *const c_void, user_data: *const c_void,
                                        callback: extern "C" fn(self_c: *const ::std::os::raw::c_void, event: *const PUPaintEvent)),
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUPushButton {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub show: extern "C" fn(self_c: *const ::std::os::raw::c_void),
    pub resize: extern "C" fn(self_c: *const ::std::os::raw::c_void, width: i32, height: i32),
    pub set_released_event: extern "C" fn(object: *const c_void, user_data: *const c_void,
                                        callback: extern "C" fn(self_c: *const ::std::os::raw::c_void)),
    pub set_text: extern "C" fn(self_c: *const ::std::os::raw::c_void, text: *const ::std::os::raw::c_char),
    pub set_flat: extern "C" fn(self_c: *const ::std::os::raw::c_void, flat: bool),
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUPainter {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub draw_line: extern "C" fn(self_c: *const ::std::os::raw::c_void, x1: i32, y1: i32, x2: i32, y2: i32),
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUListWidgetItem {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub set_text: extern "C" fn(self_c: *const ::std::os::raw::c_void, text: *const ::std::os::raw::c_char),
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUListWidget {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub show: extern "C" fn(self_c: *const ::std::os::raw::c_void),
    pub resize: extern "C" fn(self_c: *const ::std::os::raw::c_void, width: i32, height: i32),
    pub add_item: extern "C" fn(self_c: *const ::std::os::raw::c_void, text: *const ::std::os::raw::c_char),
    pub add_widget_item: extern "C" fn(self_c: *const ::std::os::raw::c_void, item: *const PUListWidgetItem),
    pub set_current_row_changed_event: extern "C" fn(object: *const c_void, user_data: *const c_void,
                                        callback: extern "C" fn(self_c: *const ::std::os::raw::c_void, row: i32)),
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUSlider {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub show: extern "C" fn(self_c: *const ::std::os::raw::c_void),
    pub resize: extern "C" fn(self_c: *const ::std::os::raw::c_void, width: i32, height: i32),
    pub set_value_changed_event: extern "C" fn(object: *const c_void, user_data: *const c_void,
                                        callback: extern "C" fn(self_c: *const ::std::os::raw::c_void, value: i32)),
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUMainWindow {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub show: extern "C" fn(self_c: *const ::std::os::raw::c_void),
    pub resize: extern "C" fn(self_c: *const ::std::os::raw::c_void, width: i32, height: i32),
    pub is_animated: extern "C" fn(self_c: *const ::std::os::raw::c_void) -> bool,
    pub set_central_widget: extern "C" fn(self_c: *const ::std::os::raw::c_void, widget: *const PUWidgetType),
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUApplication {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub set_style: extern "C" fn(self_c: *const ::std::os::raw::c_void, style: *const ::std::os::raw::c_char),
    pub exec: extern "C" fn(self_c: *const ::std::os::raw::c_void),
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUPaintEvent {
    pub rect: extern "C" fn(self_c: *const ::std::os::raw::c_void) ->  PURect,
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PU {
    pub create_widget: extern "C" fn(priv_data: *const c_void) -> *const PUWidget,
    pub create_push_button: extern "C" fn(priv_data: *const c_void) -> *const PUPushButton,
    pub create_painter: extern "C" fn(priv_data: *const c_void) -> *const PUPainter,
    pub create_list_widget_item: extern "C" fn(priv_data: *const c_void) -> *const PUListWidgetItem,
    pub create_list_widget: extern "C" fn(priv_data: *const c_void) -> *const PUListWidget,
    pub create_slider: extern "C" fn(priv_data: *const c_void) -> *const PUSlider,
    pub create_main_window: extern "C" fn(priv_data: *const c_void) -> *const PUMainWindow,
    pub create_application: extern "C" fn(priv_data: *const c_void) -> *const PUApplication,
    pub privd: *const c_void,
}

