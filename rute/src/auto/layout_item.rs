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
/// This is used by custom layouts.
///
/// Pure virtual functions are provided to return information about
/// the layout, including, sizeHint(), minimumSize(), maximumSize()
/// and expanding().
///
/// The layout's geometry can be set and retrieved with setGeometry()
/// and geometry(), and its alignment with setAlignment() and
/// alignment().
///
/// isEmpty() returns whether the layout item is empty. If the
/// concrete item is a QWidget, it can be retrieved using widget().
/// Similarly for layout() and spacerItem().
///
/// Some layouts have width and height interdependencies. These can
/// be expressed using hasHeightForWidth(), heightForWidth(), and
/// minimumHeightForWidth(). For more explanation see the *Qt
/// Quarterly* article
/// [Trading
/// Height for Width](http://doc.qt.io/archives/qq/qq04-height-for-width.html)
///
///
/// **See also:** [`Layout`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct LayoutItem<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RULayoutItemAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> LayoutItem<'a> {
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RULayoutItem) -> LayoutItem<'a> {
        LayoutItem {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RULayoutItem) -> LayoutItem<'a> {
        LayoutItem {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RULayoutItem) -> LayoutItem<'a> {
        LayoutItem {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Implemented in subclasses to return the preferred size of this item.
    pub fn size_hint(&self) -> Size {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
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
    /// Implemented in subclasses to return the minimum size of this item.
    pub fn minimum_size(&self) -> Size {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
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
    /// Implemented in subclasses to return the maximum size of this item.
    pub fn maximum_size(&self) -> Size {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
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
    /// Returns whether this layout item can make use of more space than
    /// sizeHint(). A value of Qt::Vertical or Qt::Horizontal means that
    /// it wants to grow in only one dimension, whereas Qt::Vertical |
    /// Qt::Horizontal means that it wants to grow in both dimensions.
    pub fn expanding_directions(&self) -> Orientations {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).expanding_directions)(obj_data);
            let ret_val = Orientations::from_bits_truncate(ret_val);
            ret_val
        }
    }
    ///
    /// Implemented in subclasses to return whether this item is empty,
    /// i.e. whether it contains any widgets.
    pub fn is_empty(&self) -> bool {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_empty)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns `true` if this layout's preferred height depends on its
    /// width; otherwise returns `false.` The default implementation returns
    /// false.
    ///
    /// Reimplement this function in layout managers that support height
    /// for width.
    ///
    /// **See also:** [`height_for_width()`]
    /// [`Widget::height_for_width`]
    pub fn has_height_for_width(&self) -> bool {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).has_height_for_width)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the preferred height for this layout item, given the
    /// width, which is not used in this default implementation.
    ///
    /// The default implementation returns -1, indicating that the
    /// preferred height is independent of the width of the item. Using
    /// the function hasHeightForWidth() will typically be much faster
    /// than calling this function and testing for -1.
    ///
    /// Reimplement this function in layout managers that support height
    /// for width. A typical implementation will look like this:
    ///
    /// Caching is strongly recommended; without it layout will take
    /// exponential time.
    ///
    /// **See also:** [`has_height_for_width()`]
    pub fn height_for_width(&self, arg0: i32) -> i32 {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).height_for_width)(obj_data, arg0);
            ret_val
        }
    }
    ///
    /// Returns the minimum height this widget needs for the given width,
    /// *w.* The default implementation simply returns heightForWidth( *w).*
    pub fn minimum_height_for_width(&self, arg0: i32) -> i32 {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).minimum_height_for_width)(obj_data, arg0);
            ret_val
        }
    }
    ///
    /// Invalidates any cached information in this layout item.
    pub fn invalidate(&self) -> &Self {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            ((*funcs).invalidate)(obj_data);
        }
        self
    }
    ///
    /// If this item manages a QWidget, returns that widget. Otherwise,
    /// `nullptr` is returned.
    ///
    /// **Note**: While the functions layout() and spacerItem() perform casts, this
    /// function returns another object: QLayout and QSpacerItem inherit QLayoutItem,
    /// while QWidget does not.
    ///
    /// **See also:** [`layout()`]
    /// [`spacer_item()`]
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
    ///
    /// If this item is a QLayout, it is returned as a QLayout; otherwise
    /// 0 is returned. This function provides type-safe casting.
    ///
    /// **See also:** [`spacer_item()`]
    /// [`widget()`]
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
    ///
    /// If this item is a QSpacerItem, it is returned as a QSpacerItem;
    /// otherwise 0 is returned. This function provides type-safe casting.
    ///
    /// **See also:** [`layout()`]
    /// [`widget()`]
    pub fn spacer_item(&self) -> Option<SpacerItem> {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
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
    /// Returns the alignment of this item.
    pub fn alignment(&self) -> Alignment {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).alignment)(obj_data);
            let ret_val = Alignment::from_bits_truncate(ret_val);
            ret_val
        }
    }
    ///
    /// Sets the alignment of this item to *alignment.*
    ///
    /// **Note:** Item alignment is only supported by QLayoutItem subclasses
    /// where it would have a visual effect. Except for QSpacerItem, which provides
    /// blank space for layouts, all public Qt classes that inherit QLayoutItem
    /// support item alignment.
    pub fn set_alignment(&self, a: Alignment) -> &Self {
        let enum_a_1 = a.bits();

        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            ((*funcs).set_alignment)(obj_data, enum_a_1);
        }
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}
pub trait LayoutItemTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_layout_item_obj_funcs(&self) -> (*const RUBase, *const RULayoutItemFuncs);
}

impl<'a> LayoutItemTrait<'a> for LayoutItem<'a> {
    #[doc(hidden)]
    fn get_layout_item_obj_funcs(&self) -> (*const RUBase, *const RULayoutItemFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).layout_item_funcs) }
    }
}
