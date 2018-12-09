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

pub(crate) unsafe extern "C" fn button_group_button_clicked_trampoline_ud<T>(
    self_c: *const c_void,
    func: *const c_void,
    arg0: *const RUBase,
) {
    let f: &&(Fn(&T, &AbstractButton) + 'static) = transmute(func);
    let obj_arg0_0 = AbstractButton::new_from_temporary(*(arg0 as *const RUAbstractButton));
    let data = self_c as *const T;
    f(&*data, &obj_arg0_0);
}

#[allow(unused_variables)]
pub(crate) unsafe extern "C" fn button_group_button_clicked_trampoline(
    self_c: *const c_void,
    func: *const c_void,
    arg0: *const RUBase,
) {
    let f: &&(Fn(&AbstractButton) + 'static) = transmute(func);
    let obj_arg0_0 = AbstractButton::new_from_temporary(*(arg0 as *const RUAbstractButton));
    f(&obj_arg0_0);
}

pub(crate) unsafe extern "C" fn button_group_button_clicked_2_trampoline_ud<T>(
    self_c: *const c_void,
    func: *const c_void,
    arg0: i32,
) {
    let f: &&(Fn(&T, i32) + 'static) = transmute(func);

    let data = self_c as *const T;
    f(&*data, arg0);
}

#[allow(unused_variables)]
pub(crate) unsafe extern "C" fn button_group_button_clicked_2_trampoline(
    self_c: *const c_void,
    func: *const c_void,
    arg0: i32,
) {
    let f: &&(Fn(i32) + 'static) = transmute(func);

    f(arg0);
}

pub(crate) unsafe extern "C" fn button_group_button_pressed_trampoline_ud<T>(
    self_c: *const c_void,
    func: *const c_void,
    arg0: *const RUBase,
) {
    let f: &&(Fn(&T, &AbstractButton) + 'static) = transmute(func);
    let obj_arg0_0 = AbstractButton::new_from_temporary(*(arg0 as *const RUAbstractButton));
    let data = self_c as *const T;
    f(&*data, &obj_arg0_0);
}

#[allow(unused_variables)]
pub(crate) unsafe extern "C" fn button_group_button_pressed_trampoline(
    self_c: *const c_void,
    func: *const c_void,
    arg0: *const RUBase,
) {
    let f: &&(Fn(&AbstractButton) + 'static) = transmute(func);
    let obj_arg0_0 = AbstractButton::new_from_temporary(*(arg0 as *const RUAbstractButton));
    f(&obj_arg0_0);
}

pub(crate) unsafe extern "C" fn button_group_button_pressed_2_trampoline_ud<T>(
    self_c: *const c_void,
    func: *const c_void,
    arg0: i32,
) {
    let f: &&(Fn(&T, i32) + 'static) = transmute(func);

    let data = self_c as *const T;
    f(&*data, arg0);
}

#[allow(unused_variables)]
pub(crate) unsafe extern "C" fn button_group_button_pressed_2_trampoline(
    self_c: *const c_void,
    func: *const c_void,
    arg0: i32,
) {
    let f: &&(Fn(i32) + 'static) = transmute(func);

    f(arg0);
}

pub(crate) unsafe extern "C" fn button_group_button_released_trampoline_ud<T>(
    self_c: *const c_void,
    func: *const c_void,
    arg0: *const RUBase,
) {
    let f: &&(Fn(&T, &AbstractButton) + 'static) = transmute(func);
    let obj_arg0_0 = AbstractButton::new_from_temporary(*(arg0 as *const RUAbstractButton));
    let data = self_c as *const T;
    f(&*data, &obj_arg0_0);
}

#[allow(unused_variables)]
pub(crate) unsafe extern "C" fn button_group_button_released_trampoline(
    self_c: *const c_void,
    func: *const c_void,
    arg0: *const RUBase,
) {
    let f: &&(Fn(&AbstractButton) + 'static) = transmute(func);
    let obj_arg0_0 = AbstractButton::new_from_temporary(*(arg0 as *const RUAbstractButton));
    f(&obj_arg0_0);
}

pub(crate) unsafe extern "C" fn button_group_button_released_2_trampoline_ud<T>(
    self_c: *const c_void,
    func: *const c_void,
    arg0: i32,
) {
    let f: &&(Fn(&T, i32) + 'static) = transmute(func);

    let data = self_c as *const T;
    f(&*data, arg0);
}

#[allow(unused_variables)]
pub(crate) unsafe extern "C" fn button_group_button_released_2_trampoline(
    self_c: *const c_void,
    func: *const c_void,
    arg0: i32,
) {
    let f: &&(Fn(i32) + 'static) = transmute(func);

    f(arg0);
}

pub(crate) unsafe extern "C" fn button_group_button_toggled_trampoline_ud<T>(
    self_c: *const c_void,
    func: *const c_void,
    arg0: *const RUBase,
    arg1: bool,
) {
    let f: &&(Fn(&T, &AbstractButton, bool) + 'static) = transmute(func);
    let obj_arg0_0 = AbstractButton::new_from_temporary(*(arg0 as *const RUAbstractButton));
    let data = self_c as *const T;
    f(&*data, &obj_arg0_0, arg1);
}

#[allow(unused_variables)]
pub(crate) unsafe extern "C" fn button_group_button_toggled_trampoline(
    self_c: *const c_void,
    func: *const c_void,
    arg0: *const RUBase,
    arg1: bool,
) {
    let f: &&(Fn(&AbstractButton, bool) + 'static) = transmute(func);
    let obj_arg0_0 = AbstractButton::new_from_temporary(*(arg0 as *const RUAbstractButton));
    f(&obj_arg0_0, arg1);
}

pub(crate) unsafe extern "C" fn button_group_button_toggled_2_trampoline_ud<T>(
    self_c: *const c_void,
    func: *const c_void,
    arg0: i32,
    arg1: bool,
) {
    let f: &&(Fn(&T, i32, bool) + 'static) = transmute(func);

    let data = self_c as *const T;
    f(&*data, arg0, arg1);
}

#[allow(unused_variables)]
pub(crate) unsafe extern "C" fn button_group_button_toggled_2_trampoline(
    self_c: *const c_void,
    func: *const c_void,
    arg0: i32,
    arg1: bool,
) {
    let f: &&(Fn(i32, bool) + 'static) = transmute(func);

    f(arg0, arg1);
}

/// **Notice these docs are heavy WIP and not very relevent yet**
///
/// QButtonGroup provides an abstract container into which button widgets can
/// be placed. It does not provide a visual representation of this container
/// (see QGroupBox for a container widget), but instead manages the states of
/// each of the buttons in the group.
///
/// An [exclusive](QButtonGroup::exclusive)
/// button group switches
/// off all checkable (toggle) buttons except the one that has been
/// clicked. By default, a button group is exclusive. The buttons in a
/// button group are usually checkable [QPushButton](QPushButton)
/// s, [QCheckBox](QCheckBox)
/// es
/// (normally for non-exclusive button groups), or [QRadioButton](QRadioButton)
/// s.
/// If you create an exclusive button group, you should ensure that
/// one of the buttons in the group is initially checked; otherwise,
/// the group will initially be in a state where no buttons are
/// checked.
///
/// A button can be added to the group with addButton() and removed
/// with removeButton(). If the group is exclusive, the
/// currently checked button is available with checkedButton(). If a
/// button is clicked, the buttonClicked() signal is emitted; for a
/// checkable button in an exclusive group this means that the button
/// has been checked. The list of buttons in the group is returned by
/// buttons().
///
/// In addition, QButtonGroup can map between integers and buttons.
/// You can assign an integer id to a button with setId(), and
/// retrieve it with id(). The id of the currently checked button is
/// available with checkedId(), and there is an overloaded signal
/// buttonClicked() which emits the id of the button. The id `-1`
/// is reserved by QButtonGroup to mean . The purpose
/// of the mapping mechanism is to simplify the representation of enum
/// values in a user interface.
///
/// **See also:** [`GroupBox`]
/// [`PushButton`]
/// [`CheckBox`]
/// [`RadioButton`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct ButtonGroup<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUButtonGroupAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> ButtonGroup<'a> {
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUButtonGroup) -> ButtonGroup<'a> {
        ButtonGroup {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUButtonGroup) -> ButtonGroup<'a> {
        ButtonGroup {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUButtonGroup) -> ButtonGroup<'a> {
        ButtonGroup {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    pub fn set_exclusive(&self, arg0: bool) -> &Self {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();
        unsafe {
            ((*funcs).set_exclusive)(obj_data, arg0);
        }
        self
    }
    ///
    /// If this property is `true,` then only one button in the group can be checked
    /// at any given time. The user can click on any button to check it, and that
    /// button will replace the existing one as the checked button in the group.
    ///
    /// In an exclusive group, the user cannot uncheck the currently checked button
    /// by clicking on it; instead, another button in the group must be clicked
    /// to set the new checked button for that group.
    ///
    /// By default, this property is `true.`
    pub fn exclusive(&self) -> bool {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).exclusive)(obj_data);
            ret_val
        }
    }
    ///
    /// Adds the given *button* to the button group. If *id* is -1,
    /// an id will be assigned to the button.
    /// Automatically assigned ids are guaranteed to be negative,
    /// starting with -2. If you are assigning your own ids, use
    /// positive values to avoid conflicts.
    ///
    /// **See also:** [`remove_button()`]
    /// [`buttons()`]
    pub fn add_button<A: AbstractButtonTrait<'a>>(&self, arg0: &A, id: i32) -> &Self {
        let (obj_arg0_1, _funcs) = arg0.get_abstract_button_obj_funcs();

        let (obj_data, funcs) = self.get_button_group_obj_funcs();
        unsafe {
            ((*funcs).add_button)(obj_data, obj_arg0_1, id);
        }
        self
    }
    ///
    /// Removes the given *button* from the button group.
    ///
    /// **See also:** [`add_button()`]
    /// [`buttons()`]
    pub fn remove_button<A: AbstractButtonTrait<'a>>(&self, arg0: &A) -> &Self {
        let (obj_arg0_1, _funcs) = arg0.get_abstract_button_obj_funcs();

        let (obj_data, funcs) = self.get_button_group_obj_funcs();
        unsafe {
            ((*funcs).remove_button)(obj_data, obj_arg0_1);
        }
        self
    }
    ///
    /// Returns the button group's checked button, or 0 if no buttons are
    /// checked.
    ///
    /// **See also:** [`button_clicked()`]
    pub fn checked_button(&self) -> Option<AbstractButton> {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).checked_button)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = AbstractButton::new_from_rc(t);
            } else {
                ret_val = AbstractButton::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// This signal is emitted when the given *button* is clicked. A
    /// button is clicked when it is first pressed and then released, when
    /// its shortcut key is typed, or when QAbstractButton::click()
    /// or QAbstractButton::animateClick() is programmatically called.
    ///
    /// **See also:** [`checked_button()`]
    /// [`AbstractButton::clicked`]
    ///
    /// This signal is emitted when a button with the given *id* is
    /// clicked.
    ///
    /// **See also:** [`checked_button()`]
    /// [`AbstractButton::clicked`]
    ///
    /// This signal is emitted when the given *button* is pressed down.
    ///
    /// **See also:** [`AbstractButton::pressed`]
    ///
    /// This signal is emitted when a button with the given *id* is
    /// pressed down.
    ///
    /// **See also:** [`AbstractButton::pressed`]
    ///
    /// This signal is emitted when the given *button* is released.
    ///
    /// **See also:** [`AbstractButton::released`]
    ///
    /// This signal is emitted when a button with the given *id* is
    /// released.
    ///
    /// **See also:** [`AbstractButton::released`]
    ///
    /// This signal is emitted when the given *button* is toggled.
    /// *checked* is true if the button is checked, or false if the button is unchecked.
    ///
    /// **See also:** [`AbstractButton::toggled`]
    ///
    /// This signal is emitted when a button with the given *id* is toggled.
    /// *checked* is true if the button is checked, or false if the button is unchecked.
    ///
    /// **See also:** [`AbstractButton::toggled`]
    ///
    /// Returns the button group's list of buttons. This may be empty.
    ///
    /// **See also:** [`add_button()`]
    /// [`remove_button()`]
    ///
    /// Returns the button with the specified *id,* or 0 if no such button
    /// exists.
    pub fn button(&self, id: i32) -> Option<AbstractButton> {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).button)(obj_data, id);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = AbstractButton::new_from_rc(t);
            } else {
                ret_val = AbstractButton::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Sets the *id* for the specified *button.* Note that *id* cannot
    /// be -1.
    ///
    /// **See also:** [`id()`]
    pub fn set_id<A: AbstractButtonTrait<'a>>(&self, button: &A, id: i32) -> &Self {
        let (obj_button_1, _funcs) = button.get_abstract_button_obj_funcs();

        let (obj_data, funcs) = self.get_button_group_obj_funcs();
        unsafe {
            ((*funcs).set_id)(obj_data, obj_button_1, id);
        }
        self
    }
    ///
    /// Returns the id for the specified *button,* or -1 if no such button
    /// exists.
    ///
    /// **See also:** [`set_id()`]
    pub fn id<A: AbstractButtonTrait<'a>>(&self, button: &A) -> i32 {
        let (obj_button_1, _funcs) = button.get_abstract_button_obj_funcs();

        let (obj_data, funcs) = self.get_button_group_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).id)(obj_data, obj_button_1);
            ret_val
        }
    }
    ///
    /// Returns the id of the checkedButton(), or -1 if no button is checked.
    ///
    /// **See also:** [`set_id()`]
    pub fn checked_id(&self) -> i32 {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).checked_id)(obj_data);
            ret_val
        }
    }
    ///
    /// This signal is emitted when the given *button* is clicked. A
    /// button is clicked when it is first pressed and then released, when
    /// its shortcut key is typed, or when QAbstractButton::click()
    /// or QAbstractButton::animateClick() is programmatically called.
    ///
    /// **See also:** [`checked_button()`]
    /// [`AbstractButton::clicked`]
    ///
    /// This signal is emitted when a button with the given *id* is
    /// clicked.
    ///
    /// **See also:** [`checked_button()`]
    /// [`AbstractButton::clicked`]
    pub fn set_button_clicked_event_ud<F, T>(&self, data: &'a T, func: F) -> &Self
    where
        F: Fn(&T, &AbstractButton) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();

        let f: Box<Box<Fn(&T, &AbstractButton) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_button_clicked_event)(
                obj_data,
                user_data,
                Box::into_raw(f) as *const _,
                transmute(button_group_button_clicked_trampoline_ud::<T> as usize),
            );
        }

        self
    }

    pub fn set_button_clicked_event<F>(&self, func: F) -> &Self
    where
        F: Fn(&AbstractButton) + 'a,
    {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();
        let f: Box<Box<Fn(&AbstractButton) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_button_clicked_event)(
                obj_data,
                ::std::ptr::null(),
                Box::into_raw(f) as *const _,
                transmute(button_group_button_clicked_trampoline as usize),
            );
        }

        self
    }
    ///
    /// This signal is emitted when the given *button* is clicked. A
    /// button is clicked when it is first pressed and then released, when
    /// its shortcut key is typed, or when QAbstractButton::click()
    /// or QAbstractButton::animateClick() is programmatically called.
    ///
    /// **See also:** [`checked_button()`]
    /// [`AbstractButton::clicked`]
    ///
    /// This signal is emitted when a button with the given *id* is
    /// clicked.
    ///
    /// **See also:** [`checked_button()`]
    /// [`AbstractButton::clicked`]
    pub fn set_button_clicked_2_event_ud<F, T>(&self, data: &'a T, func: F) -> &Self
    where
        F: Fn(&T, i32) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();

        let f: Box<Box<Fn(&T, i32) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_button_clicked_2_event)(
                obj_data,
                user_data,
                Box::into_raw(f) as *const _,
                transmute(button_group_button_clicked_2_trampoline_ud::<T> as usize),
            );
        }

        self
    }

    pub fn set_button_clicked_2_event<F>(&self, func: F) -> &Self
    where
        F: Fn(i32) + 'a,
    {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();
        let f: Box<Box<Fn(i32) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_button_clicked_2_event)(
                obj_data,
                ::std::ptr::null(),
                Box::into_raw(f) as *const _,
                transmute(button_group_button_clicked_2_trampoline as usize),
            );
        }

        self
    }
    ///
    /// This signal is emitted when the given *button* is pressed down.
    ///
    /// **See also:** [`AbstractButton::pressed`]
    ///
    /// This signal is emitted when a button with the given *id* is
    /// pressed down.
    ///
    /// **See also:** [`AbstractButton::pressed`]
    pub fn set_button_pressed_event_ud<F, T>(&self, data: &'a T, func: F) -> &Self
    where
        F: Fn(&T, &AbstractButton) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();

        let f: Box<Box<Fn(&T, &AbstractButton) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_button_pressed_event)(
                obj_data,
                user_data,
                Box::into_raw(f) as *const _,
                transmute(button_group_button_pressed_trampoline_ud::<T> as usize),
            );
        }

        self
    }

    pub fn set_button_pressed_event<F>(&self, func: F) -> &Self
    where
        F: Fn(&AbstractButton) + 'a,
    {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();
        let f: Box<Box<Fn(&AbstractButton) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_button_pressed_event)(
                obj_data,
                ::std::ptr::null(),
                Box::into_raw(f) as *const _,
                transmute(button_group_button_pressed_trampoline as usize),
            );
        }

        self
    }
    ///
    /// This signal is emitted when the given *button* is pressed down.
    ///
    /// **See also:** [`AbstractButton::pressed`]
    ///
    /// This signal is emitted when a button with the given *id* is
    /// pressed down.
    ///
    /// **See also:** [`AbstractButton::pressed`]
    pub fn set_button_pressed_2_event_ud<F, T>(&self, data: &'a T, func: F) -> &Self
    where
        F: Fn(&T, i32) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();

        let f: Box<Box<Fn(&T, i32) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_button_pressed_2_event)(
                obj_data,
                user_data,
                Box::into_raw(f) as *const _,
                transmute(button_group_button_pressed_2_trampoline_ud::<T> as usize),
            );
        }

        self
    }

    pub fn set_button_pressed_2_event<F>(&self, func: F) -> &Self
    where
        F: Fn(i32) + 'a,
    {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();
        let f: Box<Box<Fn(i32) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_button_pressed_2_event)(
                obj_data,
                ::std::ptr::null(),
                Box::into_raw(f) as *const _,
                transmute(button_group_button_pressed_2_trampoline as usize),
            );
        }

        self
    }
    ///
    /// This signal is emitted when the given *button* is released.
    ///
    /// **See also:** [`AbstractButton::released`]
    ///
    /// This signal is emitted when a button with the given *id* is
    /// released.
    ///
    /// **See also:** [`AbstractButton::released`]
    pub fn set_button_released_event_ud<F, T>(&self, data: &'a T, func: F) -> &Self
    where
        F: Fn(&T, &AbstractButton) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();

        let f: Box<Box<Fn(&T, &AbstractButton) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_button_released_event)(
                obj_data,
                user_data,
                Box::into_raw(f) as *const _,
                transmute(button_group_button_released_trampoline_ud::<T> as usize),
            );
        }

        self
    }

    pub fn set_button_released_event<F>(&self, func: F) -> &Self
    where
        F: Fn(&AbstractButton) + 'a,
    {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();
        let f: Box<Box<Fn(&AbstractButton) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_button_released_event)(
                obj_data,
                ::std::ptr::null(),
                Box::into_raw(f) as *const _,
                transmute(button_group_button_released_trampoline as usize),
            );
        }

        self
    }
    ///
    /// This signal is emitted when the given *button* is released.
    ///
    /// **See also:** [`AbstractButton::released`]
    ///
    /// This signal is emitted when a button with the given *id* is
    /// released.
    ///
    /// **See also:** [`AbstractButton::released`]
    pub fn set_button_released_2_event_ud<F, T>(&self, data: &'a T, func: F) -> &Self
    where
        F: Fn(&T, i32) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();

        let f: Box<Box<Fn(&T, i32) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_button_released_2_event)(
                obj_data,
                user_data,
                Box::into_raw(f) as *const _,
                transmute(button_group_button_released_2_trampoline_ud::<T> as usize),
            );
        }

        self
    }

    pub fn set_button_released_2_event<F>(&self, func: F) -> &Self
    where
        F: Fn(i32) + 'a,
    {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();
        let f: Box<Box<Fn(i32) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_button_released_2_event)(
                obj_data,
                ::std::ptr::null(),
                Box::into_raw(f) as *const _,
                transmute(button_group_button_released_2_trampoline as usize),
            );
        }

        self
    }
    ///
    /// This signal is emitted when the given *button* is toggled.
    /// *checked* is true if the button is checked, or false if the button is unchecked.
    ///
    /// **See also:** [`AbstractButton::toggled`]
    ///
    /// This signal is emitted when a button with the given *id* is toggled.
    /// *checked* is true if the button is checked, or false if the button is unchecked.
    ///
    /// **See also:** [`AbstractButton::toggled`]
    pub fn set_button_toggled_event_ud<F, T>(&self, data: &'a T, func: F) -> &Self
    where
        F: Fn(&T, &AbstractButton, bool) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();

        let f: Box<Box<Fn(&T, &AbstractButton, bool) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_button_toggled_event)(
                obj_data,
                user_data,
                Box::into_raw(f) as *const _,
                transmute(button_group_button_toggled_trampoline_ud::<T> as usize),
            );
        }

        self
    }

    pub fn set_button_toggled_event<F>(&self, func: F) -> &Self
    where
        F: Fn(&AbstractButton, bool) + 'a,
    {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();
        let f: Box<Box<Fn(&AbstractButton, bool) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_button_toggled_event)(
                obj_data,
                ::std::ptr::null(),
                Box::into_raw(f) as *const _,
                transmute(button_group_button_toggled_trampoline as usize),
            );
        }

        self
    }
    ///
    /// This signal is emitted when the given *button* is toggled.
    /// *checked* is true if the button is checked, or false if the button is unchecked.
    ///
    /// **See also:** [`AbstractButton::toggled`]
    ///
    /// This signal is emitted when a button with the given *id* is toggled.
    /// *checked* is true if the button is checked, or false if the button is unchecked.
    ///
    /// **See also:** [`AbstractButton::toggled`]
    pub fn set_button_toggled_2_event_ud<F, T>(&self, data: &'a T, func: F) -> &Self
    where
        F: Fn(&T, i32, bool) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();

        let f: Box<Box<Fn(&T, i32, bool) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_button_toggled_2_event)(
                obj_data,
                user_data,
                Box::into_raw(f) as *const _,
                transmute(button_group_button_toggled_2_trampoline_ud::<T> as usize),
            );
        }

        self
    }

    pub fn set_button_toggled_2_event<F>(&self, func: F) -> &Self
    where
        F: Fn(i32, bool) + 'a,
    {
        let (obj_data, funcs) = self.get_button_group_obj_funcs();
        let f: Box<Box<Fn(i32, bool) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_button_toggled_2_event)(
                obj_data,
                ::std::ptr::null(),
                Box::into_raw(f) as *const _,
                transmute(button_group_button_toggled_2_trampoline as usize),
            );
        }

        self
    }
    #[doc(hidden)]
    pub fn object_name(&self) -> String {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).object_name)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn set_object_name(&self, name: &str) -> &Self {
        let str_in_name_1 = CString::new(name).unwrap();

        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).set_object_name)(obj_data, str_in_name_1.as_ptr());
        }
        self
    }
    #[doc(hidden)]
    pub fn is_widget_type(&self) -> bool {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_widget_type)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn is_window_type(&self) -> bool {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_window_type)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn signals_blocked(&self) -> bool {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).signals_blocked)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn block_signals(&self, b: bool) -> bool {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).block_signals)(obj_data, b);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn start_timer(&self, interval: i32, timer_type: TimerType) -> i32 {
        let enum_timer_type_2 = timer_type as i32;

        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).start_timer)(obj_data, interval, enum_timer_type_2);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn start_timer_2(&self, time: u32, timer_type: TimerType) -> i32 {
        let enum_timer_type_2 = timer_type as i32;

        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).start_timer_2)(obj_data, time, enum_timer_type_2);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn kill_timer(&self, id: i32) -> &Self {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).kill_timer)(obj_data, id);
        }
        self
    }
    #[doc(hidden)]
    pub fn set_parent<O: ObjectTrait<'a>>(&self, parent: &O) -> &Self {
        let (obj_parent_1, _funcs) = parent.get_object_obj_funcs();

        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).set_parent)(obj_data, obj_parent_1);
        }
        self
    }
    #[doc(hidden)]
    pub fn install_event_filter<O: ObjectTrait<'a>>(&self, filter_obj: &O) -> &Self {
        let (obj_filter_obj_1, _funcs) = filter_obj.get_object_obj_funcs();

        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).install_event_filter)(obj_data, obj_filter_obj_1);
        }
        self
    }
    #[doc(hidden)]
    pub fn dump_object_tree(&self) -> &Self {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).dump_object_tree)(obj_data);
        }
        self
    }
    #[doc(hidden)]
    pub fn dump_object_info(&self) -> &Self {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).dump_object_info)(obj_data);
        }
        self
    }
    #[doc(hidden)]
    pub fn dump_object_tree_2(&self) -> &Self {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).dump_object_tree_2)(obj_data);
        }
        self
    }
    #[doc(hidden)]
    pub fn dump_object_info_2(&self) -> &Self {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).dump_object_info_2)(obj_data);
        }
        self
    }
    #[doc(hidden)]
    pub fn parent(&self) -> Option<Object> {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).parent)(obj_data);
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
    pub fn delete_later(&self) -> &Self {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).delete_later)(obj_data);
        }
        self
    }
    #[doc(hidden)]
    pub fn set_custom_event_ud<F, T>(&self, data: &'a T, func: F) -> &Self
    where
        F: Fn(&T, &Event) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_object_obj_funcs();

        let f: Box<Box<Fn(&T, &Event) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_custom_event)(
                obj_data,
                user_data,
                Box::into_raw(f) as *const _,
                transmute(object_custom_trampoline_ud::<T> as usize),
            );
        }

        self
    }

    pub fn set_custom_event<F>(&self, func: F) -> &Self
    where
        F: Fn(&Event) + 'a,
    {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        let f: Box<Box<Fn(&Event) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_custom_event)(
                obj_data,
                ::std::ptr::null(),
                Box::into_raw(f) as *const _,
                transmute(object_custom_trampoline as usize),
            );
        }

        self
    }
}
pub trait ButtonGroupTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_button_group_obj_funcs(&self) -> (*const RUBase, *const RUButtonGroupFuncs);
}

impl<'a> ObjectTrait<'a> for ButtonGroup<'a> {
    #[doc(hidden)]
    fn get_object_obj_funcs(&self) -> (*const RUBase, *const RUObjectFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).object_funcs) }
    }
}

impl<'a> ButtonGroupTrait<'a> for ButtonGroup<'a> {
    #[doc(hidden)]
    fn get_button_group_obj_funcs(&self) -> (*const RUBase, *const RUButtonGroupFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).button_group_funcs) }
    }
}
