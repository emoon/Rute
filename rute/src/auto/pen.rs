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
/// A pen has a style(), width(), brush(), capStyle() and joinStyle().
///
/// The pen style defines the line type. The brush is used to fill
/// strokes generated with the pen. Use the QBrush class to specify
/// fill styles. The cap style determines the line end caps that can
/// be drawn using QPainter, while the join style describes how joins
/// between two lines are drawn. The pen width can be specified in
/// both integer (width()) and floating point (widthF()) precision. A
/// line width of zero indicates a cosmetic pen. This means that the
/// pen width is always drawn one pixel wide, independent of the [transformation](QPainter%23Coordinate%20Transformations)
/// set on the
/// painter.
///
/// The various settings can easily be modified using the
/// corresponding setStyle(), setWidth(), setBrush(), setCapStyle()
/// and setJoinStyle() functions (note that the painter's pen must be
/// reset when altering the pen's properties).
///
/// For example:
///
/// which is equivalent to
///
/// The default pen is a solid black brush with 1 width, square
/// cap style (Qt::SquareCap), and bevel join style (Qt::BevelJoin).
///
/// In addition QPen provides the color() and setColor()
/// convenience functions to extract and set the color of the pen's
/// brush, respectively. Pens may also be compared and streamed.
///
/// For more information about painting in general, see the [Paint
/// System](Paint%0A%20%20%20%20System)
/// documentation.
///
/// # Pen Style
///
/// Qt provides several built-in styles represented by the
/// Qt::PenStyle enum:
///
/// * ![qpen-solid.png](qpen-solid.png)
///
/// * ![qpen-dash.png](qpen-dash.png)
///
/// * ![qpen-dot.png](qpen-dot.png)
///
/// * Qt::SolidLine
/// * Qt::DashLine
/// * Qt::DotLine
///
/// * ![qpen-dashdot.png](qpen-dashdot.png)
///
/// * ![qpen-dashdotdot.png](qpen-dashdotdot.png)
///
/// * ![qpen-custom.png](qpen-custom.png)
///
/// * Qt::DashDotLine
/// * Qt::DashDotDotLine
/// * Qt::CustomDashLine
///
/// Simply use the setStyle() function to convert the pen style to
/// either of the built-in styles, except the Qt::CustomDashLine style
/// which we will come back to shortly. Setting the style to Qt::NoPen
/// tells the painter to not draw lines or outlines. The default pen
/// style is Qt::SolidLine.
///
/// Since Qt 4.1 it is also possible to specify a custom dash pattern
/// using the setDashPattern() function which implicitly converts the
/// style of the pen to Qt::CustomDashLine. The pattern argument, a
/// QVector, must be specified as an even number of [qreal](qreal)
/// entries
/// where the entries 1, 3, 5... are the dashes and 2, 4, 6... are the
/// spaces. For example, the custom pattern shown above is created
/// using the following code:
///
/// Note that the dash pattern is specified in units of the pens
/// width, e.g. a dash of length 5 in width 10 is 50 pixels long.
///
/// The currently set dash pattern can be retrieved using the
/// dashPattern() function. Use the isSolid() function to determine
/// whether the pen has a solid fill, or not.
///
/// # Cap Style
///
/// The cap style defines how the end points of lines are drawn using
/// QPainter. The cap style only apply to wide lines, i.e. when the
/// width is 1 or greater. The Qt::PenCapStyle enum provides the
/// following styles:
///
/// * ![qpen-square.png](qpen-square.png)
///
/// * ![qpen-flat.png](qpen-flat.png)
///
/// * ![qpen-roundcap.png](qpen-roundcap.png)
///
/// * Qt::SquareCap
/// * Qt::FlatCap
/// * Qt::RoundCap
///
/// The Qt::SquareCap style is a square line end that covers the end
/// point and extends beyond it by half the line width. The
/// Qt::FlatCap style is a square line end that does not cover the end
/// point of the line. And the Qt::RoundCap style is a rounded line
/// end covering the end point.
///
/// The default is Qt::SquareCap.
///
/// Whether or not end points are drawn when the pen width is 0 or 1
/// depends on the cap style. Using Qt::SquareCap or Qt::RoundCap they
/// are drawn, using Qt::FlatCap they are not drawn.
///
/// # Join Style
///
/// The join style defines how joins between two connected lines can
/// be drawn using QPainter. The join style only apply to wide lines,
/// i.e. when the width is 1 or greater. The Qt::PenJoinStyle enum
/// provides the following styles:
///
/// * ![qpen-bevel.png](qpen-bevel.png)
///
/// * ![qpen-miter.png](qpen-miter.png)
///
/// * ![qpen-roundjoin.png](qpen-roundjoin.png)
///
/// * Qt::BevelJoin
/// * Qt::MiterJoin
/// * Qt::RoundJoin
///
/// The Qt::BevelJoin style fills the triangular notch between the two
/// lines. The Qt::MiterJoin style extends the lines to meet at an
/// angle. And the Qt::RoundJoin style fills a circular arc between
/// the two lines.
///
/// The default is Qt::BevelJoin.
///
/// ![qpen-miterlimit.png](qpen-miterlimit.png)
///
/// When the Qt::MiterJoin style is applied, it is possible to use the
/// setMiterLimit() function to specify how far the miter join can
/// extend from the join point. The miterLimit() is used to reduce
/// artifacts between line joins where the lines are close to
/// parallel.
///
/// The miterLimit() must be specified in units of the pens width,
/// e.g. a miter limit of 5 in width 10 is 50 pixels long. The
/// default miter limit is 2, i.e. twice the pen width in pixels.
///
/// * ![qpen-demo.png](qpen-demo.png)
///
/// * **\l {painting/pathstroke** {The Path Stroking Example}} The Path Stroking example shows Qt's built-in dash patterns and shows how custom patterns can be used to extend the range of available patterns.
///
/// **See also:** [`Painter`]
/// [`Brush`]
/// {painting/pathstroke}{Path Stroking Example}
/// {Scribble Example}
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct Pen<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUPenAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Pen<'a> {
    pub fn new() -> Pen<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_pen)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        Pen {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUPen) -> Pen<'a> {
        Pen {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUPen) -> Pen<'a> {
        Pen {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUPen) -> Pen<'a> {
        Pen {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Swaps pen *other* with this pen. This operation is very
    /// fast and never fails.
    pub fn swap<P: PenTrait<'a>>(&self, other: &P) -> &Self {
        let (obj_other_1, _funcs) = other.get_pen_obj_funcs();

        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            ((*funcs).swap)(obj_data, obj_other_1);
        }
        self
    }
    ///
    /// Returns the pen style.
    ///
    /// **See also:** [`set_style()`]
    /// {QPen#Pen Style}{Pen Style}
    pub fn style(&self) -> PenStyle {
        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).style)(obj_data);
            let ret_val = { transmute::<u32, PenStyle>(ret_val) };
            ret_val
        }
    }
    ///
    /// Sets the pen style to the given *style.*
    ///
    /// See the [Qt::PenStyle](Qt::PenStyle)
    /// documentation for a list of the available
    /// styles. Since Qt 4.1 it is also possible to specify a custom dash
    /// pattern using the setDashPattern() function which implicitly
    /// converts the style of the pen to Qt::CustomDashLine.
    ///
    /// **Note**: This function resets the dash offset to zero.
    ///
    /// **See also:** [`style()`]
    /// {QPen#Pen Style}{Pen Style}
    pub fn set_style(&self, arg0: PenStyle) -> &Self {
        let enum_arg0_1 = arg0 as u32;

        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            ((*funcs).set_style)(obj_data, enum_arg0_1);
        }
        self
    }
    pub fn dash_pattern(&self) -> PrimitiveArray<f32> {
        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).dash_pattern)(obj_data);
            let ret_val = PrimitiveArray::<f32>::new(ret_val);
            ret_val
        }
    }
    ///
    /// Returns the dash offset for the pen.
    ///
    /// **See also:** [`set_dash_offset()`]
    pub fn dash_offset(&self) -> f32 {
        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).dash_offset)(obj_data);
            ret_val
        }
    }
    ///
    /// Sets the dash offset (the starting point on the dash pattern) for this pen
    /// to the *offset* specified. The offset is measured in terms of the units used
    /// to specify the dash pattern.
    ///
    /// * ![qpen-dashpattern.png](qpen-dashpattern.png)
    ///
    /// * For example, a pattern where each stroke is four units long, followed by a gap of two units, will begin with the stroke when drawn as a line. However, if the dash offset is set to 4.0, any line drawn will begin with the gap. Values of the offset up to 4.0 will cause part of the stroke to be drawn first, and values of the offset between 4.0 and 6.0 will cause the line to begin with part of the gap.
    ///
    /// **Note**: This implicitly converts the style of the pen to Qt::CustomDashLine.
    pub fn set_dash_offset(&self, doffset: f32) -> &Self {
        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            ((*funcs).set_dash_offset)(obj_data, doffset);
        }
        self
    }
    ///
    /// Returns the miter limit of the pen. The miter limit is only
    /// relevant when the join style is set to Qt::MiterJoin.
    ///
    /// **See also:** [`set_miter_limit()`]
    /// {QPen#Join Style}{Join Style}
    pub fn miter_limit(&self) -> f32 {
        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).miter_limit)(obj_data);
            ret_val
        }
    }
    ///
    /// Sets the miter limit of this pen to the given *limit.*
    ///
    /// ![qpen-miterlimit.png](qpen-miterlimit.png)
    ///
    /// The miter limit describes how far a miter join can extend from the
    /// join point. This is used to reduce artifacts between line joins
    /// where the lines are close to parallel.
    ///
    /// This value does only have effect when the pen style is set to
    /// Qt::MiterJoin. The value is specified in units of the pen's width,
    /// e.g. a miter limit of 5 in width 10 is 50 pixels long. The default
    /// miter limit is 2, i.e. twice the pen width in pixels.
    ///
    /// **See also:** [`miter_limit()`]
    /// [`set_join_style()`]
    /// {QPen#Join Style}{Join Style}
    pub fn set_miter_limit(&self, limit: f32) -> &Self {
        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            ((*funcs).set_miter_limit)(obj_data, limit);
        }
        self
    }
    ///
    /// Returns the pen width with floating point precision.
    ///
    /// **See also:** [`set_width_f()`]
    /// [`width()`]
    pub fn width_f(&self) -> f32 {
        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).width_f)(obj_data);
            ret_val
        }
    }
    ///
    /// Sets the pen width to the given *width* in pixels with floating point
    /// precision.
    ///
    /// A line width of zero indicates a cosmetic pen. This means that the
    /// pen width is always drawn one pixel wide, independent of the [transformation](QPainter%23Coordinate%20Transformations)
    /// on the
    /// painter.
    ///
    /// Setting a pen width with a negative value is not supported.
    ///
    /// **See also:** [`set_width()`]
    /// [`width_f()`]
    pub fn set_width_f(&self, width: f32) -> &Self {
        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            ((*funcs).set_width_f)(obj_data, width);
        }
        self
    }
    ///
    /// Returns the pen width with integer precision.
    ///
    /// **See also:** [`set_width()`]
    /// [`width_f()`]
    ///
    /// Returns the pen width with floating point precision.
    ///
    /// **See also:** [`set_width_f()`]
    /// [`width()`]
    pub fn width(&self) -> i32 {
        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).width)(obj_data);
            ret_val
        }
    }
    ///
    /// Sets the pen width to the given *width* in pixels with integer
    /// precision.
    ///
    /// A line width of zero indicates a cosmetic pen. This means that the
    /// pen width is always drawn one pixel wide, independent of the [transformation](QPainter%23Coordinate%20Transformations)
    /// set on the
    /// painter.
    ///
    /// Setting a pen width with a negative value is not supported.
    ///
    /// **See also:** [`set_width_f()`]
    /// [`width()`]
    ///
    /// Sets the pen width to the given *width* in pixels with floating point
    /// precision.
    ///
    /// A line width of zero indicates a cosmetic pen. This means that the
    /// pen width is always drawn one pixel wide, independent of the [transformation](QPainter%23Coordinate%20Transformations)
    /// on the
    /// painter.
    ///
    /// Setting a pen width with a negative value is not supported.
    ///
    /// **See also:** [`set_width()`]
    /// [`width_f()`]
    pub fn set_width(&self, width: i32) -> &Self {
        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            ((*funcs).set_width)(obj_data, width);
        }
        self
    }
    ///
    /// Returns the color of this pen's brush.
    ///
    /// **See also:** [`brush()`]
    /// [`set_color()`]
    pub fn color(&self) -> Color {
        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).color)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Color::new_from_rc(t);
            } else {
                ret_val = Color::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Sets the color of this pen's brush to the given *color.*
    ///
    /// **See also:** [`set_brush()`]
    /// [`color()`]
    pub fn set_color<C: ColorTrait<'a>>(&self, color: &C) -> &Self {
        let (obj_color_1, _funcs) = color.get_color_obj_funcs();

        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            ((*funcs).set_color)(obj_data, obj_color_1);
        }
        self
    }
    ///
    /// Returns the brush used to fill strokes generated with this pen.
    pub fn brush(&self) -> Brush {
        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).brush)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Sets the brush used to fill strokes generated with this pen to the given
    /// *brush.*
    ///
    /// **See also:** [`brush()`]
    /// [`set_color()`]
    pub fn set_brush<B: BrushTrait<'a>>(&self, brush: &B) -> &Self {
        let (obj_brush_1, _funcs) = brush.get_brush_obj_funcs();

        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            ((*funcs).set_brush)(obj_data, obj_brush_1);
        }
        self
    }
    ///
    /// Returns `true` if the pen has a solid fill, otherwise false.
    ///
    /// **See also:** [`style()`]
    /// [`dash_pattern()`]
    pub fn is_solid(&self) -> bool {
        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_solid)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the pen's cap style.
    ///
    /// **See also:** [`set_cap_style()`]
    /// {QPen#Cap Style}{Cap Style}
    pub fn cap_style(&self) -> PenCapStyle {
        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).cap_style)(obj_data);
            let ret_val = { transmute::<u32, PenCapStyle>(ret_val) };
            ret_val
        }
    }
    ///
    /// Sets the pen's cap style to the given *style.* The default value
    /// is Qt::SquareCap.
    ///
    /// **See also:** [`cap_style()`]
    /// {QPen#Cap Style}{Cap Style}
    pub fn set_cap_style(&self, pcs: PenCapStyle) -> &Self {
        let enum_pcs_1 = pcs as u32;

        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            ((*funcs).set_cap_style)(obj_data, enum_pcs_1);
        }
        self
    }
    ///
    /// Returns the pen's join style.
    ///
    /// **See also:** [`set_join_style()`]
    /// {QPen#Join Style}{Join Style}
    pub fn join_style(&self) -> PenJoinStyle {
        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).join_style)(obj_data);
            let ret_val = PenJoinStyle::from_bits_truncate(ret_val);
            ret_val
        }
    }
    ///
    /// Sets the pen's join style to the given *style.* The default value
    /// is Qt::BevelJoin.
    ///
    /// **See also:** [`join_style()`]
    /// {QPen#Join Style}{Join Style}
    pub fn set_join_style(&self, pcs: PenJoinStyle) -> &Self {
        let enum_pcs_1 = pcs.bits();

        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            ((*funcs).set_join_style)(obj_data, enum_pcs_1);
        }
        self
    }
    ///
    /// Returns `true` if the pen is cosmetic; otherwise returns `false.`
    ///
    /// Cosmetic pens are used to draw strokes that have a constant width
    /// regardless of any transformations applied to the QPainter they are
    /// used with. Drawing a shape with a cosmetic pen ensures that its
    /// outline will have the same thickness at different scale factors.
    ///
    /// A zero width pen is cosmetic by default.
    ///
    /// **See also:** [`set_cosmetic()`]
    /// [`width_f()`]
    pub fn is_cosmetic(&self) -> bool {
        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_cosmetic)(obj_data);
            ret_val
        }
    }
    ///
    /// Sets this pen to cosmetic or non-cosmetic, depending on the value of
    /// *cosmetic.*
    ///
    /// **See also:** [`is_cosmetic()`]
    pub fn set_cosmetic(&self, cosmetic: bool) -> &Self {
        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            ((*funcs).set_cosmetic)(obj_data, cosmetic);
        }
        self
    }
    pub fn is_detached(&self) -> bool {
        let (obj_data, funcs) = self.get_pen_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_detached)(obj_data);
            ret_val
        }
    }
}
pub trait PenTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_pen_obj_funcs(&self) -> (*const RUBase, *const RUPenFuncs);
}

impl<'a> PenTrait<'a> for Pen<'a> {
    #[doc(hidden)]
    fn get_pen_obj_funcs(&self) -> (*const RUBase, *const RUPenFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).pen_funcs) }
    }
}
