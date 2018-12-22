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
/// Focus events are sent to widgets when the keyboard input focus
/// changes. Focus events occur due to mouse actions, key presses
/// (such as **{Tab}** or **{Backtab}),** the window system, popup
/// menus, keyboard shortcuts, or other application-specific reasons.
/// The reason for a particular focus event is returned by reason()
/// in the appropriate event handler.
///
/// The event handlers QWidget::focusInEvent(),
/// QWidget::focusOutEvent(), QGraphicsItem::focusInEvent and
/// QGraphicsItem::focusOutEvent() receive focus events.
///
/// **See also:** [`Widget::set_focus`]
/// [`Widget::set_focus_policy`]
/// {Keyboard Focus in Widgets}
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct FocusEvent<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUFocusEventAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> FocusEvent<'a> {
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUFocusEvent) -> FocusEvent<'a> {
        FocusEvent {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUFocusEvent) -> FocusEvent<'a> {
        FocusEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUFocusEvent) -> FocusEvent<'a> {
        FocusEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Returns `true` if type() is QEvent::FocusIn; otherwise returns
    /// false.
    pub fn got_focus(&self) -> bool {
        let (obj_data, funcs) = self.get_focus_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).got_focus)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns `true` if type() is QEvent::FocusOut; otherwise returns
    /// false.
    pub fn lost_focus(&self) -> bool {
        let (obj_data, funcs) = self.get_focus_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).lost_focus)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the reason for this focus event.
    pub fn reason(&self) -> FocusReason {
        let (obj_data, funcs) = self.get_focus_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).reason)(obj_data);
            let ret_val = { transmute::<u32, FocusReason>(ret_val) };
            ret_val
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
    #[doc(hidden)]
    pub fn accept(&self) -> &Self {
        let (obj_data, funcs) = self.get_event_obj_funcs();
        unsafe {
            ((*funcs).accept)(obj_data);
        }
        self
    }
    #[doc(hidden)]
    pub fn ignore(&self) -> &Self {
        let (obj_data, funcs) = self.get_event_obj_funcs();
        unsafe {
            ((*funcs).ignore)(obj_data);
        }
        self
    }
}
pub trait FocusEventTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_focus_event_obj_funcs(&self) -> (*const RUBase, *const RUFocusEventFuncs);
}

impl<'a> EventTrait<'a> for FocusEvent<'a> {
    #[doc(hidden)]
    fn get_event_obj_funcs(&self) -> (*const RUBase, *const RUEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).event_funcs) }
    }
}

impl<'a> FocusEventTrait<'a> for FocusEvent<'a> {
    #[doc(hidden)]
    fn get_focus_event_obj_funcs(&self) -> (*const RUBase, *const RUFocusEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).focus_event_funcs) }
    }
}
