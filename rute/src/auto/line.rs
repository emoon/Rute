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
/// A QLine describes a finite length line (or a line segment) on a
/// two-dimensional surface. The start and end points of the line are
/// specified using integer point accuracy for coordinates. Use the
/// QLineF constructor to retrieve a floating point copy.
///
/// * ![qline-point.png](qline-point.png)
///
/// * ![qline-coordinates.png](qline-coordinates.png)
///
/// The positions of the line's start and end points can be retrieved
/// using the p1(), x1(), y1(), p2(), x2(), and y2() functions. The
/// dx() and dy() functions return the horizontal and vertical
/// components of the line. Use isNull() to determine whether the
/// QLine represents a valid line or a null line.
///
/// Finally, the line can be translated a given offset using the
/// translate() function.
///
/// **See also:** [`LineF`]
/// [`Polygon`]
/// [`Rect`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct Line<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RULineAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Line<'a> {
    pub fn new() -> Line<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_line)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        Line {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RULine) -> Line<'a> {
        let data =
            unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) };
        let t = Line {
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
    pub(crate) fn new_from_owned(ffi_data: RULine) -> Line<'a> {
        Line {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RULine) -> Line<'a> {
        Line {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Returns `true` if the line is not set up with valid start and end point;
    /// otherwise returns `false.`
    pub fn is_null(&self) -> bool {
        let (obj_data, funcs) = self.get_line_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_null)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the line's start point.
    ///
    /// **See also:** [`x1()`]
    /// [`y1()`]
    /// [`p2()`]
    pub fn p1(&self) -> Point {
        let (obj_data, funcs) = self.get_line_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).p1)(obj_data);
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
    ///
    /// Returns the line's end point.
    ///
    /// **See also:** [`x2()`]
    /// [`y2()`]
    /// [`p1()`]
    pub fn p2(&self) -> Point {
        let (obj_data, funcs) = self.get_line_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).p2)(obj_data);
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
    ///
    /// Returns the x-coordinate of the line's start point.
    ///
    /// **See also:** [`p1()`]
    pub fn x1(&self) -> i32 {
        let (obj_data, funcs) = self.get_line_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).x1)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the y-coordinate of the line's start point.
    ///
    /// **See also:** [`p1()`]
    pub fn y1(&self) -> i32 {
        let (obj_data, funcs) = self.get_line_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).y1)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the x-coordinate of the line's end point.
    ///
    /// **See also:** [`p2()`]
    pub fn x2(&self) -> i32 {
        let (obj_data, funcs) = self.get_line_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).x2)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the y-coordinate of the line's end point.
    ///
    /// **See also:** [`p2()`]
    pub fn y2(&self) -> i32 {
        let (obj_data, funcs) = self.get_line_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).y2)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the horizontal component of the line's vector.
    ///
    /// **See also:** [`dy()`]
    pub fn dx(&self) -> i32 {
        let (obj_data, funcs) = self.get_line_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).dx)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the vertical component of the line's vector.
    ///
    /// **See also:** [`dx()`]
    pub fn dy(&self) -> i32 {
        let (obj_data, funcs) = self.get_line_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).dy)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the center point of this line. This is equivalent to
    /// (p1() + p2()) / 2, except it will never overflow.
    pub fn center(&self) -> Point {
        let (obj_data, funcs) = self.get_line_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).center)(obj_data);
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
    ///
    /// Sets the starting point of this line to *p1.*
    ///
    /// **See also:** [`set_p2()`]
    /// [`p1()`]
    pub fn set_p1<P: PointTrait<'a>>(&self, p1: &P) -> &Self {
        let (obj_p1_1, _funcs) = p1.get_point_obj_funcs();

        let (obj_data, funcs) = self.get_line_obj_funcs();
        unsafe {
            ((*funcs).set_p1)(obj_data, obj_p1_1);
        }
        self
    }
    ///
    /// Sets the end point of this line to *p2.*
    ///
    /// **See also:** [`set_p1()`]
    /// [`p2()`]
    pub fn set_p2<P: PointTrait<'a>>(&self, p2: &P) -> &Self {
        let (obj_p2_1, _funcs) = p2.get_point_obj_funcs();

        let (obj_data, funcs) = self.get_line_obj_funcs();
        unsafe {
            ((*funcs).set_p2)(obj_data, obj_p2_1);
        }
        self
    }
    ///
    /// Sets the start point of this line to *p1* and the end point of this line to *p2.*
    ///
    /// **See also:** [`set_p1()`]
    /// [`set_p2()`]
    /// [`p1()`]
    /// [`p2()`]
    pub fn set_points<P: PointTrait<'a>>(&self, p1: &P, p2: &P) -> &Self {
        let (obj_p1_1, _funcs) = p1.get_point_obj_funcs();
        let (obj_p2_2, _funcs) = p2.get_point_obj_funcs();

        let (obj_data, funcs) = self.get_line_obj_funcs();
        unsafe {
            ((*funcs).set_points)(obj_data, obj_p1_1, obj_p2_2);
        }
        self
    }
    ///
    /// Sets this line to the start in *x1,* *y1* and end in *x2,* *y2.*
    ///
    /// **See also:** [`set_p1()`]
    /// [`set_p2()`]
    /// [`p1()`]
    /// [`p2()`]
    pub fn set_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> &Self {
        let (obj_data, funcs) = self.get_line_obj_funcs();
        unsafe {
            ((*funcs).set_line)(obj_data, x1, y1, x2, y2);
        }
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

impl<'a> From<WrapperRcOwn> for Line<'a> {
    fn from(t: WrapperRcOwn) -> Self {
        let mut data = RULine {
            qt_data: ::std::ptr::null(),
            host_data: ::std::ptr::null(),
            all_funcs: t.all_funcs as *const RULineAllFuncs,
        };

        if t.owned {
            data.host_data = t.data as *const RUBase;
            Line::new_from_rc(data)
        } else {
            data.qt_data = t.data as *const RUBase;
            Line::new_from_temporary(data)
        }
    }
}

pub trait LineTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_line_obj_funcs(&self) -> (*const RUBase, *const RULineFuncs);
}

impl<'a> LineTrait<'a> for Line<'a> {
    #[doc(hidden)]
    fn get_line_obj_funcs(&self) -> (*const RUBase, *const RULineFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).line_funcs) }
    }
}
