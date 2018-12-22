// This file is auto-generated by rute_gen. DO NOT EDIT.
use std::cell::Cell;
use std::rc::Rc;

#[allow(unused_imports)]
use std::marker::PhantomData;

#[allow(unused_imports)]
use std::os::raw::c_void;

#[allow(unused_imports)]
use std::mem::transmute;

#[allow(unused_imports)]
use std::ffi::{CStr, CString};

use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::*;

/// **Notice these docs are heavy WIP and not very relevent yet**
///
/// A widget must accept this event in order to receive the [drag move events](QDragMoveEvent)
/// that are sent while the drag
/// and drop action is in progress. The drag enter event is always
/// immediately followed by a drag move event.
///
/// QDragEnterEvent inherits most of its functionality from
/// QDragMoveEvent, which in turn inherits most of its functionality
/// from QDropEvent.
///
/// **See also:** [`DragLeaveEvent`]
/// [`DragMoveEvent`]
/// [`DropEvent`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct DragEnterEvent<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUDragEnterEventAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> DragEnterEvent<'a> {
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUDragEnterEvent) -> DragEnterEvent<'a> {
        DragEnterEvent {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUDragEnterEvent) -> DragEnterEvent<'a> {
        DragEnterEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUDragEnterEvent) -> DragEnterEvent<'a> {
        DragEnterEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    #[doc(hidden)]
    pub fn answer_rect(&self) -> Rect {
        let (obj_data, funcs) = self.get_drag_move_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).answer_rect)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Rect::new_from_rc(t);
            } else {
                ret_val = Rect::new_from_owned(t);
            }
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn accept(&self) -> &Self {
        let (obj_data, funcs) = self.get_drag_move_event_obj_funcs();
        unsafe {
            ((*funcs).accept)(obj_data);
        }
        self
    }
    #[doc(hidden)]
    pub fn ignore(&self) -> &Self {
        let (obj_data, funcs) = self.get_drag_move_event_obj_funcs();
        unsafe {
            ((*funcs).ignore)(obj_data);
        }
        self
    }
    #[doc(hidden)]
    pub fn accept_2<R: RectTrait<'a>>(&self, r: &R) -> &Self {
        let (obj_r_1, _funcs) = r.get_rect_obj_funcs();

        let (obj_data, funcs) = self.get_drag_move_event_obj_funcs();
        unsafe {
            ((*funcs).accept_2)(obj_data, obj_r_1);
        }
        self
    }
    #[doc(hidden)]
    pub fn ignore_2<R: RectTrait<'a>>(&self, r: &R) -> &Self {
        let (obj_r_1, _funcs) = r.get_rect_obj_funcs();

        let (obj_data, funcs) = self.get_drag_move_event_obj_funcs();
        unsafe {
            ((*funcs).ignore_2)(obj_data, obj_r_1);
        }
        self
    }
    #[doc(hidden)]
    pub fn pos(&self) -> Point {
        let (obj_data, funcs) = self.get_drop_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).pos)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Point::new_from_rc(t);
            } else {
                ret_val = Point::new_from_owned(t);
            }
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn pos_f(&self) -> Option<PointF> {
        let (obj_data, funcs) = self.get_drop_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).pos_f)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = PointF::new_from_rc(t);
            } else {
                ret_val = PointF::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    #[doc(hidden)]
    pub fn mouse_buttons(&self) -> MouseButtons {
        let (obj_data, funcs) = self.get_drop_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).mouse_buttons)(obj_data);
            let ret_val = MouseButtons::from_bits_truncate(ret_val);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn keyboard_modifiers(&self) -> KeyboardModifiers {
        let (obj_data, funcs) = self.get_drop_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).keyboard_modifiers)(obj_data);
            let ret_val = KeyboardModifiers::from_bits_truncate(ret_val);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn possible_actions(&self) -> DropActions {
        let (obj_data, funcs) = self.get_drop_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).possible_actions)(obj_data);
            let ret_val = DropActions::from_bits_truncate(ret_val);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn proposed_action(&self) -> DropAction {
        let (obj_data, funcs) = self.get_drop_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).proposed_action)(obj_data);
            let ret_val = { transmute::<u32, DropAction>(ret_val) };
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn accept_proposed_action(&self) -> &Self {
        let (obj_data, funcs) = self.get_drop_event_obj_funcs();
        unsafe {
            ((*funcs).accept_proposed_action)(obj_data);
        }
        self
    }
    #[doc(hidden)]
    pub fn drop_action(&self) -> DropAction {
        let (obj_data, funcs) = self.get_drop_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).drop_action)(obj_data);
            let ret_val = { transmute::<u32, DropAction>(ret_val) };
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn set_drop_action(&self, action: DropAction) -> &Self {
        let enum_action_1 = action as u32;

        let (obj_data, funcs) = self.get_drop_event_obj_funcs();
        unsafe {
            ((*funcs).set_drop_action)(obj_data, enum_action_1);
        }
        self
    }
    #[doc(hidden)]
    pub fn source(&self) -> Option<Object> {
        let (obj_data, funcs) = self.get_drop_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).source)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Object::new_from_rc(t);
            } else {
                ret_val = Object::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    #[doc(hidden)]
    pub fn mime_data(&self) -> Option<MimeData> {
        let (obj_data, funcs) = self.get_drop_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).mime_data)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = MimeData::new_from_rc(t);
            } else {
                ret_val = MimeData::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    #[doc(hidden)]
    pub fn spontaneous(&self) -> bool {
        let (obj_data, funcs) = self.get_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).spontaneous)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn set_accepted(&self, accepted: bool) -> &Self {
        let (obj_data, funcs) = self.get_event_obj_funcs();
        unsafe {
            ((*funcs).set_accepted)(obj_data, accepted);
        }
        self
    }
    #[doc(hidden)]
    pub fn is_accepted(&self) -> bool {
        let (obj_data, funcs) = self.get_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_accepted)(obj_data);
            ret_val
        }
    }
}
pub trait DragEnterEventTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_drag_enter_event_obj_funcs(&self) -> (*const RUBase, *const RUDragEnterEventFuncs);
}

impl<'a> EventTrait<'a> for DragEnterEvent<'a> {
    #[doc(hidden)]
    fn get_event_obj_funcs(&self) -> (*const RUBase, *const RUEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).event_funcs) }
    }
}

impl<'a> DropEventTrait<'a> for DragEnterEvent<'a> {
    #[doc(hidden)]
    fn get_drop_event_obj_funcs(&self) -> (*const RUBase, *const RUDropEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).drop_event_funcs) }
    }
}

impl<'a> DragMoveEventTrait<'a> for DragEnterEvent<'a> {
    #[doc(hidden)]
    fn get_drag_move_event_obj_funcs(&self) -> (*const RUBase, *const RUDragMoveEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).drag_move_event_funcs) }
    }
}

impl<'a> DragEnterEventTrait<'a> for DragEnterEvent<'a> {
    #[doc(hidden)]
    fn get_drag_enter_event_obj_funcs(&self) -> (*const RUBase, *const RUDragEnterEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).drag_enter_event_funcs) }
    }
}
