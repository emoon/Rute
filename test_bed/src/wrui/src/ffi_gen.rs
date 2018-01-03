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
pub struct PURect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[repr(C)]
pub struct PUObjectFuncs {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub is_widget_type: extern "C" fn(self_c: *const ::std::os::raw::c_void) -> bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PUObject {
    pub funcs: *const PUObjectFuncs,
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUWidgetFuncs {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub is_widget_type: extern "C" fn(self_c: *const ::std::os::raw::c_void) -> bool,
    pub show: extern "C" fn(self_c: *const ::std::os::raw::c_void),
    pub resize: extern "C" fn(self_c: *const ::std::os::raw::c_void, width: i32, height: i32),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PUWidget {
    pub funcs: *const PUWidgetFuncs,
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUPushButtonFuncs {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub is_widget_type: extern "C" fn(self_c: *const ::std::os::raw::c_void) -> bool,
    pub show: extern "C" fn(self_c: *const ::std::os::raw::c_void),
    pub resize: extern "C" fn(self_c: *const ::std::os::raw::c_void, width: i32, height: i32),
    pub set_released_event: extern "C" fn(object: *const c_void, user_data: *const c_void,
                                        callback: extern "C" fn(self_c: *const ::std::os::raw::c_void)),
    pub set_text: extern "C" fn(self_c: *const ::std::os::raw::c_void, text: *const ::std::os::raw::c_char),
    pub set_flat: extern "C" fn(self_c: *const ::std::os::raw::c_void, flat: bool),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PUPushButton {
    pub funcs: *const PUPushButtonFuncs,
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUPainterFuncs {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub draw_line: extern "C" fn(self_c: *const ::std::os::raw::c_void, x1: i32, y1: i32, x2: i32, y2: i32),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PUPainter {
    pub funcs: *const PUPainterFuncs,
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUListWidgetItemFuncs {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub set_text: extern "C" fn(self_c: *const ::std::os::raw::c_void, text: *const ::std::os::raw::c_char),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PUListWidgetItem {
    pub funcs: *const PUListWidgetItemFuncs,
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUListWidgetFuncs {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub is_widget_type: extern "C" fn(self_c: *const ::std::os::raw::c_void) -> bool,
    pub show: extern "C" fn(self_c: *const ::std::os::raw::c_void),
    pub resize: extern "C" fn(self_c: *const ::std::os::raw::c_void, width: i32, height: i32),
    pub add_item: extern "C" fn(self_c: *const ::std::os::raw::c_void, text: *const ::std::os::raw::c_char),
    pub item: extern "C" fn(self_c: *const ::std::os::raw::c_void, index: i32) ->  PUListWidgetItem,
    pub add_widget_item: extern "C" fn(self_c: *const ::std::os::raw::c_void, item: *const PUListWidgetItem),
    pub set_current_row_changed_event: extern "C" fn(object: *const c_void, user_data: *const c_void,
                                        callback: extern "C" fn(self_c: *const ::std::os::raw::c_void, row: i32)),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PUListWidget {
    pub funcs: *const PUListWidgetFuncs,
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUSliderFuncs {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub is_widget_type: extern "C" fn(self_c: *const ::std::os::raw::c_void) -> bool,
    pub show: extern "C" fn(self_c: *const ::std::os::raw::c_void),
    pub resize: extern "C" fn(self_c: *const ::std::os::raw::c_void, width: i32, height: i32),
    pub set_value_changed_event: extern "C" fn(object: *const c_void, user_data: *const c_void,
                                        callback: extern "C" fn(self_c: *const ::std::os::raw::c_void, value: i32)),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PUSlider {
    pub funcs: *const PUSliderFuncs,
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUMainWindowFuncs {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub is_widget_type: extern "C" fn(self_c: *const ::std::os::raw::c_void) -> bool,
    pub show: extern "C" fn(self_c: *const ::std::os::raw::c_void),
    pub resize: extern "C" fn(self_c: *const ::std::os::raw::c_void, width: i32, height: i32),
    pub is_animated: extern "C" fn(self_c: *const ::std::os::raw::c_void) -> bool,
    pub set_central_widget: extern "C" fn(self_c: *const ::std::os::raw::c_void, widget: *const PUWidgetType),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PUMainWindow {
    pub funcs: *const PUMainWindowFuncs,
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUActionFuncs {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub is_widget_type: extern "C" fn(self_c: *const ::std::os::raw::c_void) -> bool,
    pub is_enabled: extern "C" fn(self_c: *const ::std::os::raw::c_void) -> bool,
    pub set_text: extern "C" fn(self_c: *const ::std::os::raw::c_void, text: *const ::std::os::raw::c_char),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PUAction {
    pub funcs: *const PUActionFuncs,
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUMenuFuncs {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub is_widget_type: extern "C" fn(self_c: *const ::std::os::raw::c_void) -> bool,
    pub show: extern "C" fn(self_c: *const ::std::os::raw::c_void),
    pub resize: extern "C" fn(self_c: *const ::std::os::raw::c_void, width: i32, height: i32),
    pub add_action_text: extern "C" fn(self_c: *const ::std::os::raw::c_void, text: *const ::std::os::raw::c_char),
    pub add_action: extern "C" fn(self_c: *const ::std::os::raw::c_void, action: *const PUAction),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PUMenu {
    pub funcs: *const PUMenuFuncs,
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PUApplicationFuncs {
    pub destroy: extern "C" fn(self_c: *const c_void),
    pub set_style: extern "C" fn(self_c: *const ::std::os::raw::c_void, style: *const ::std::os::raw::c_char),
    pub exec: extern "C" fn(self_c: *const ::std::os::raw::c_void),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PUApplication {
    pub funcs: *const PUApplicationFuncs,
    pub privd: *const c_void,
}

#[repr(C)]
pub struct PU {
    pub create_object: extern "C" fn(priv_data: *const c_void) -> PUObject,
    pub create_widget: extern "C" fn(priv_data: *const c_void) -> PUWidget,
    pub create_push_button: extern "C" fn(priv_data: *const c_void) -> PUPushButton,
    pub create_painter: extern "C" fn(priv_data: *const c_void) -> PUPainter,
    pub create_list_widget_item: extern "C" fn(priv_data: *const c_void) -> PUListWidgetItem,
    pub create_list_widget: extern "C" fn(priv_data: *const c_void) -> PUListWidget,
    pub create_slider: extern "C" fn(priv_data: *const c_void) -> PUSlider,
    pub create_main_window: extern "C" fn(priv_data: *const c_void) -> PUMainWindow,
    pub create_action: extern "C" fn(priv_data: *const c_void) -> PUAction,
    pub create_menu: extern "C" fn(priv_data: *const c_void) -> PUMenu,
    pub create_application: extern "C" fn(priv_data: *const c_void) -> PUApplication,
    pub privd: *const c_void,
}

