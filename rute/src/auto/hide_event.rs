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
/// This event is sent just before QWidget::hide() returns, and also
/// when a top-level window has been hidden (iconified) by the user.
///
/// If spontaneous() is true, the event originated outside the
/// application. In this case, the user hid the window using the
/// window manager controls, either by iconifying the window or by
/// switching to another virtual desktop where the window is not
/// visible. The window will become hidden but not withdrawn. If the
/// window was iconified, QWidget::isMinimized() returns `true.`
///
/// **See also:** [`ShowEvent`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct HideEvent<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUHideEventAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> HideEvent<'a> {
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUHideEvent) -> HideEvent<'a> {
        HideEvent {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUHideEvent) -> HideEvent<'a> {
        HideEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUHideEvent) -> HideEvent<'a> {
        HideEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
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
pub trait HideEventTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_hide_event_obj_funcs(&self) -> (*const RUBase, *const RUHideEventFuncs);
}

impl<'a> EventTrait<'a> for HideEvent<'a> {
    #[doc(hidden)]
    fn get_event_obj_funcs(&self) -> (*const RUBase, *const RUEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).event_funcs) }
    }
}

impl<'a> HideEventTrait<'a> for HideEvent<'a> {
    #[doc(hidden)]
    fn get_hide_event_obj_funcs(&self) -> (*const RUBase, *const RUHideEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).hide_event_funcs) }
    }
}
