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
/// A size is specified by a width() and a height(). It can be set in
/// the constructor and changed using the setWidth(), setHeight(), or
/// scale() functions, or using arithmetic operators. A size can also
/// be manipulated directly by retrieving references to the width and
/// height using the rwidth() and rheight() functions. Finally, the
/// width and height can be swapped using the transpose() function.
///
/// The isValid() function determines if a size is valid (a valid size
/// has both width and height greater than or equal to zero). The isEmpty()
/// function returns `true` if either of the width and height is less
/// than, or equal to, zero, while the isNull() function returns `true`
/// only if both the width and the height is zero.
///
/// Use the expandedTo() function to retrieve a size which holds the
/// maximum height and width of *this* size and a given
/// size. Similarly, the boundedTo() function returns a size which
/// holds the minimum height and width of *this* size and a given
/// size.
///
/// QSize objects can be streamed as well as compared.
///
/// **See also:** [`SizeF`]
/// [`Point`]
/// [`Rect`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct Size<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUSizeAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Size<'a> {
    pub fn new() -> Size<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_size)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        Size {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUSize) -> Size<'a> {
        let data =
            unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) };
        let t = Size {
            data: data.clone(),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        };

        // this is to allow us to clone inside instead of the outside in iterators and such
        // as this is always used in that context
        ::std::mem::forget(data);

        t
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUSize) -> Size<'a> {
        Size {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUSize) -> Size<'a> {
        Size {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Returns `true` if both the width and height is 0; otherwise returns
    /// false.
    ///
    /// **See also:** [`is_valid()`]
    /// [`is_empty()`]
    pub fn is_null(&self) -> bool {
        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_null)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns `true` if either of the width and height is less than or
    /// equal to 0; otherwise returns `false.`
    ///
    /// **See also:** [`is_null()`]
    /// [`is_valid()`]
    pub fn is_empty(&self) -> bool {
        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_empty)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns `true` if both the width and height is equal to or greater
    /// than 0; otherwise returns `false.`
    ///
    /// **See also:** [`is_null()`]
    /// [`is_empty()`]
    pub fn is_valid(&self) -> bool {
        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_valid)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the width.
    ///
    /// **See also:** [`height()`]
    /// [`set_width()`]
    pub fn width(&self) -> i32 {
        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).width)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the height.
    ///
    /// **See also:** [`width()`]
    /// [`set_height()`]
    pub fn height(&self) -> i32 {
        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).height)(obj_data);
            ret_val
        }
    }
    ///
    /// Sets the width to the given *width.*
    ///
    /// **See also:** [`rwidth()`]
    /// [`width()`]
    /// [`set_height()`]
    pub fn set_width(&self, w: i32) -> &Self {
        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            ((*funcs).set_width)(obj_data, w);
        }
        self
    }
    ///
    /// Sets the height to the given *height.*
    ///
    /// **See also:** [`rheight()`]
    /// [`height()`]
    /// [`set_width()`]
    pub fn set_height(&self, h: i32) -> &Self {
        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            ((*funcs).set_height)(obj_data, h);
        }
        self
    }
    ///
    /// Scales the size to a rectangle with the given *width* and *height,* according to the specified *mode:*
    ///
    /// * If *mode* is Qt::IgnoreAspectRatio, the size is set to ( *width,* *height).*
    /// * If *mode* is Qt::KeepAspectRatio, the current size is scaled to a rectangle as large as possible inside ( *width,* *height),* preserving the aspect ratio.
    /// * If *mode* is Qt::KeepAspectRatioByExpanding, the current size is scaled to a rectangle as small as possible outside ( *width,* *height),* preserving the aspect ratio.
    ///
    /// Example:
    ///
    /// **See also:** [`set_width()`]
    /// [`set_height()`]
    /// [`scaled()`]
    ///
    /// **Overloads**
    /// Scales the size to a rectangle with the given *size,* according to
    /// the specified *mode.*
    ///
    /// Return a size scaled to a rectangle with the given *width* and *height,* according to the specified *mode.*
    ///
    /// **See also:** [`scale()`]
    ///
    /// **Overloads**
    /// Return a size scaled to a rectangle with the given size *s,*
    /// according to the specified *mode.*
    pub fn scale(&self, w: i32, h: i32, mode: AspectRatioMode) -> &Self {
        let enum_mode_3 = mode as u32;

        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            ((*funcs).scale)(obj_data, w, h, enum_mode_3);
        }
        self
    }
    ///
    /// Scales the size to a rectangle with the given *width* and *height,* according to the specified *mode:*
    ///
    /// * If *mode* is Qt::IgnoreAspectRatio, the size is set to ( *width,* *height).*
    /// * If *mode* is Qt::KeepAspectRatio, the current size is scaled to a rectangle as large as possible inside ( *width,* *height),* preserving the aspect ratio.
    /// * If *mode* is Qt::KeepAspectRatioByExpanding, the current size is scaled to a rectangle as small as possible outside ( *width,* *height),* preserving the aspect ratio.
    ///
    /// Example:
    ///
    /// **See also:** [`set_width()`]
    /// [`set_height()`]
    /// [`scaled()`]
    ///
    /// **Overloads**
    /// Scales the size to a rectangle with the given *size,* according to
    /// the specified *mode.*
    ///
    /// Return a size scaled to a rectangle with the given *width* and *height,* according to the specified *mode.*
    ///
    /// **See also:** [`scale()`]
    ///
    /// **Overloads**
    /// Return a size scaled to a rectangle with the given size *s,*
    /// according to the specified *mode.*
    pub fn scale_2<S: SizeTrait<'a>>(&self, s: &S, mode: AspectRatioMode) -> &Self {
        let (obj_s_1, _funcs) = s.get_size_obj_funcs();
        let enum_mode_2 = mode as u32;

        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            ((*funcs).scale_2)(obj_data, obj_s_1, enum_mode_2);
        }
        self
    }
    ///
    /// Return a size scaled to a rectangle with the given *width* and *height,* according to the specified *mode.*
    ///
    /// **See also:** [`scale()`]
    ///
    /// **Overloads**
    /// Return a size scaled to a rectangle with the given size *s,*
    /// according to the specified *mode.*
    pub fn scaled(&self, w: i32, h: i32, mode: AspectRatioMode) -> Size {
        let enum_mode_3 = mode as u32;

        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).scaled)(obj_data, w, h, enum_mode_3);
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
    /// Return a size scaled to a rectangle with the given *width* and *height,* according to the specified *mode.*
    ///
    /// **See also:** [`scale()`]
    ///
    /// **Overloads**
    /// Return a size scaled to a rectangle with the given size *s,*
    /// according to the specified *mode.*
    pub fn scaled_2<S: SizeTrait<'a>>(&self, s: &S, mode: AspectRatioMode) -> Size {
        let (obj_s_1, _funcs) = s.get_size_obj_funcs();
        let enum_mode_2 = mode as u32;

        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).scaled_2)(obj_data, obj_s_1, enum_mode_2);
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
    /// Returns a size holding the maximum width and height of this size
    /// and the given *otherSize.*
    ///
    /// **See also:** [`bounded_to()`]
    /// [`scale()`]
    pub fn expanded_to<S: SizeTrait<'a>>(&self, arg0: &S) -> Size {
        let (obj_arg0_1, _funcs) = arg0.get_size_obj_funcs();

        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).expanded_to)(obj_data, obj_arg0_1);
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
    /// Returns a size holding the minimum width and height of this size
    /// and the given *otherSize.*
    ///
    /// **See also:** [`expanded_to()`]
    /// [`scale()`]
    pub fn bounded_to<S: SizeTrait<'a>>(&self, arg0: &S) -> Size {
        let (obj_arg0_1, _funcs) = arg0.get_size_obj_funcs();

        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).bounded_to)(obj_data, obj_arg0_1);
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
    /// Returns a reference to the width.
    ///
    /// Using a reference makes it possible to manipulate the width
    /// directly. For example:
    ///
    /// **See also:** [`rheight()`]
    /// [`set_width()`]
    pub fn rwidth(&self) -> i32 {
        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).rwidth)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns a reference to the height.
    ///
    /// Using a reference makes it possible to manipulate the height
    /// directly. For example:
    ///
    /// **See also:** [`rwidth()`]
    /// [`set_height()`]
    pub fn rheight(&self) -> i32 {
        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).rheight)(obj_data);
            ret_val
        }
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

impl<'a> From<WrapperRcOwn> for Size<'a> {
    fn from(t: WrapperRcOwn) -> Self {
        let mut data = RUSize {
            qt_data: ::std::ptr::null(),
            host_data: ::std::ptr::null(),
            all_funcs: t.all_funcs as *const RUSizeAllFuncs,
        };

        if t.owned {
            data.host_data = t.data as *const RUBase;
            Size::new_from_rc(data)
        } else {
            data.qt_data = t.data as *const RUBase;
            Size::new_from_temporary(data)
        }
    }
}

pub trait SizeTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_size_obj_funcs(&self) -> (*const RUBase, *const RUSizeFuncs);
}

impl<'a> SizeTrait<'a> for Size<'a> {
    #[doc(hidden)]
    fn get_size_obj_funcs(&self) -> (*const RUBase, *const RUSizeFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).size_funcs) }
    }
}
