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
/// Normally, you don't need to use this class directly. Qt's
/// built-in layout managers provide the following functions for
/// manipulating empty space in layouts:
///
/// * Class
/// * Functions
/// * QHBoxLayout
/// * [addSpacing()](QBoxLayout::addSpacing())
/// , [addStretch()](QBoxLayout::addStretch())
/// , [insertSpacing()](QBoxLayout::insertSpacing())
/// , [insertStretch()](QBoxLayout::insertStretch())
///
/// * QGridLayout
/// * [setRowMinimumHeight()](QGridLayout::setRowMinimumHeight())
/// , [setRowStretch()](QGridLayout::setRowStretch())
/// , [setColumnMinimumWidth()](QGridLayout::setColumnMinimumWidth())
/// , [setColumnStretch()](QGridLayout::setColumnStretch())
///
/// **See also:** [`Layout`]
/// [`WidgetItem`]
/// [`LayoutItem::spacer_item`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct SpacerItem<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUSpacerItemAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> SpacerItem<'a> {
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUSpacerItem) -> SpacerItem<'a> {
        SpacerItem {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUSpacerItem) -> SpacerItem<'a> {
        SpacerItem {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUSpacerItem) -> SpacerItem<'a> {
        SpacerItem {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Changes this spacer item to have preferred width *w,* preferred
    /// height *h,* horizontal size policy *hPolicy* and vertical size
    /// policy *vPolicy.*
    ///
    /// The default values provide a gap that is able to stretch if
    /// nothing else wants the space.
    ///
    /// Note that if changeSize() is called after the spacer item has been added
    /// to a layout, it is necessary to invalidate the layout in order for the
    /// spacer item's new size to take effect.
    ///
    /// **See also:** [`SpacerItem::invalidate`]
    pub fn change_size(&self, w: i32, h: i32, h_data: Policy, v_data: Policy) -> &Self {
        let enum_h_data_3 = h_data as u32;
        let enum_v_data_4 = v_data as u32;

        let (obj_data, funcs) = self.get_spacer_item_obj_funcs();
        unsafe {
            ((*funcs).change_size)(obj_data, w, h, enum_h_data_3, enum_v_data_4);
        }
        self
    }
    ///
    pub fn size_hint(&self) -> Size {
        let (obj_data, funcs) = self.get_spacer_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).size_hint)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Size::new_from_rc(t);
            } else {
                ret_val = Size::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    pub fn minimum_size(&self) -> Size {
        let (obj_data, funcs) = self.get_spacer_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).minimum_size)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Size::new_from_rc(t);
            } else {
                ret_val = Size::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    pub fn maximum_size(&self) -> Size {
        let (obj_data, funcs) = self.get_spacer_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).maximum_size)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Size::new_from_rc(t);
            } else {
                ret_val = Size::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    pub fn expanding_directions(&self) -> Orientations {
        let (obj_data, funcs) = self.get_spacer_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).expanding_directions)(obj_data);
            let ret_val = Orientations::from_bits_truncate(ret_val);
            ret_val
        }
    }
    ///
    /// Returns `true.`
    pub fn is_empty(&self) -> bool {
        let (obj_data, funcs) = self.get_spacer_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_empty)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns a pointer to this object.
    pub fn spacer_item(&self) -> Option<SpacerItem> {
        let (obj_data, funcs) = self.get_spacer_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).spacer_item)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = SpacerItem::new_from_rc(t);
            } else {
                ret_val = SpacerItem::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the size policy of this item.
    pub fn size_policy(&self) -> SizePolicy {
        let (obj_data, funcs) = self.get_spacer_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).size_policy)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = SizePolicy::new_from_rc(t);
            } else {
                ret_val = SizePolicy::new_from_owned(t);
            }
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn has_height_for_width(&self) -> bool {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).has_height_for_width)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn height_for_width(&self, arg0: i32) -> i32 {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).height_for_width)(obj_data, arg0);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn minimum_height_for_width(&self, arg0: i32) -> i32 {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).minimum_height_for_width)(obj_data, arg0);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn invalidate(&self) -> &Self {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            ((*funcs).invalidate)(obj_data);
        }
        self
    }
    #[doc(hidden)]
    pub fn widget(&self) -> Option<Widget> {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).widget)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Widget::new_from_rc(t);
            } else {
                ret_val = Widget::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    #[doc(hidden)]
    pub fn layout(&self) -> Option<Layout> {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).layout)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Layout::new_from_rc(t);
            } else {
                ret_val = Layout::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    #[doc(hidden)]
    pub fn alignment(&self) -> Alignment {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).alignment)(obj_data);
            let ret_val = Alignment::from_bits_truncate(ret_val);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn set_alignment(&self, a: Alignment) -> &Self {
        let enum_a_1 = a.bits();

        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            ((*funcs).set_alignment)(obj_data, enum_a_1);
        }
        self
    }
}
pub trait SpacerItemTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_spacer_item_obj_funcs(&self) -> (*const RUBase, *const RUSpacerItemFuncs);
}

impl<'a> LayoutItemTrait<'a> for SpacerItem<'a> {
    #[doc(hidden)]
    fn get_layout_item_obj_funcs(&self) -> (*const RUBase, *const RULayoutItemFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).layout_item_funcs) }
    }
}

impl<'a> SpacerItemTrait<'a> for SpacerItem<'a> {
    #[doc(hidden)]
    fn get_spacer_item_obj_funcs(&self) -> (*const RUBase, *const RUSpacerItemFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).spacer_item_funcs) }
    }
}
