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

// Auto-generated imports
use auto::*;

///
/// A palette consists of three color groups: *Active,* *Disabled,*
/// and *Inactive.* All widgets in Qt contain a palette and
/// use their palette to draw themselves. This makes the user
/// interface easily configurable and easier to keep consistent.
///
/// If you create a new widget we strongly recommend that you use the
/// colors in the palette rather than hard-coding specific colors.
///
/// The color groups:
/// * The Active group is used for the window that has keyboard focus.
/// * The Inactive group is used for other windows.
/// * The Disabled group is used for widgets (not windows) that are disabled for some reason.
///
/// Both active and inactive windows can contain disabled widgets.
/// (Disabled widgets are often called *inaccessible* or *grayed
/// out* .)
///
/// In most styles, Active and Inactive look the same.
///
/// Colors and brushes can be set for particular roles in any of a palette's
/// color groups with setColor() and setBrush(). A color group contains a
/// group of colors used by widgets for drawing themselves. We recommend that
/// widgets use color group roles from the palette such as and
/// rather than literal colors like or . The color
/// roles are enumerated and defined in the [ColorRole](ColorRole)
/// documentation.
///
/// We strongly recommend that you use the default palette of the
/// current style (returned by QGuiApplication::palette()) and
/// modify that as necessary. This is done by Qt's widgets when they
/// are drawn.
///
/// To modify a color group you call the functions
/// setColor() and setBrush(), depending on whether you want a pure
/// color or a pixmap pattern.
///
/// There are also corresponding color() and brush() getters, and a
/// commonly used convenience function to get the ColorRole for the current ColorGroup:
/// window(), windowText(), base(), etc.
///
/// You can copy a palette using the copy constructor and test to see
/// if two palettes are *identical* using isCopyOf().
///
/// QPalette is optimized by the use of [implicit sharing](implicit%20sharing)
///
/// so it is very efficient to pass QPalette objects as arguments.
///
/// **Warning**: Some styles do not use the palette for all drawing, for
/// instance, if they make use of native theme engines. This is the
/// case for both the Windows Vista and the MacOS
/// styles.
///
/// **See also:** [`Application::set_palette`]
/// [`Widget::set_palette`]
/// [`Color`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct Palette<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUPaletteAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Palette<'a> {
    pub fn new() -> Palette<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_palette)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        Palette {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUPalette) -> Palette<'a> {
        Palette {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUPalette) -> Palette<'a> {
        Palette {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUPalette) -> Palette<'a> {
        Palette {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Swaps this palette instance with *other.* This function is very
    /// fast and never fails.
    pub fn swap<P: PaletteTrait<'a>>(&self, other: &P) -> &Self {
        let (obj_other_1, _funcs) = other.get_palette_obj_funcs();

        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            ((*funcs).swap)(obj_data, obj_other_1);
        }
        self
    }
    ///
    /// Returns the palette's current color group.
    pub fn current_color_group(&self) -> ColorGroup {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).current_color_group)(obj_data);
            let ret_val = { transmute::<i32, ColorGroup>(ret_val) };
            ret_val
        }
    }
    ///
    /// Set the palette's current color group to *cg.*
    pub fn set_current_color_group(&self, cg: ColorGroup) -> &Self {
        let enum_cg_1 = cg as i32;

        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            ((*funcs).set_current_color_group)(obj_data, enum_cg_1);
        }
        self
    }
    ///
    /// **Overloads**
    /// Returns the color that has been set for the given color *role* in
    /// the current ColorGroup.
    ///
    /// **See also:** [`brush()`]
    /// ColorRole
    ///
    /// Returns the color in the specified color *group,* used for the
    /// given color *role.*
    ///
    /// **See also:** [`brush()`]
    /// [`set_color()`]
    /// ColorRole
    pub fn color(&self, cg: ColorGroup, cr: ColorRole) -> Option<Color> {
        let enum_cg_1 = cg as i32;
        let enum_cr_2 = cr as i32;

        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).color)(obj_data, enum_cg_1, enum_cr_2);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Color::new_from_rc(t);
            } else {
                ret_val = Color::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// **Overloads**
    /// Returns the brush that has been set for the given color *role* in
    /// the current ColorGroup.
    ///
    /// **See also:** [`color()`]
    /// [`set_brush()`]
    /// ColorRole
    ///
    /// Returns the brush in the specified color *group,* used for the
    /// given color *role.*
    ///
    /// **See also:** [`color()`]
    /// [`set_brush()`]
    /// ColorRole
    pub fn brush(&self, cg: ColorGroup, cr: ColorRole) -> Option<Brush> {
        let enum_cg_1 = cg as i32;
        let enum_cr_2 = cr as i32;

        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).brush)(obj_data, enum_cg_1, enum_cr_2);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// **Overloads**
    /// Sets the color used for the given color *role,* in all color
    /// groups, to the specified solid *color.*
    ///
    /// **See also:** [`brush()`]
    /// [`set_color()`]
    /// ColorRole
    ///
    /// Sets the color in the specified color *group,* used for the given
    /// color *role,* to the specified solid *color.*
    ///
    /// **See also:** [`set_brush()`]
    /// [`color()`]
    /// ColorRole
    ///
    /// Sets a the group at *cg.* You can pass either brushes, pixmaps or
    /// plain colors for *windowText,* *button,* *light,* *dark,* *mid,* *text,* *bright_text,* *base* and *window.*
    ///
    /// **See also:** [`Brush`]
    pub fn set_color<C: ColorTrait<'a>>(&self, cg: ColorGroup, cr: ColorRole, color: &C) -> &Self {
        let enum_cg_1 = cg as i32;
        let enum_cr_2 = cr as i32;
        let (obj_color_3, _funcs) = color.get_color_obj_funcs();

        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            ((*funcs).set_color)(obj_data, enum_cg_1, enum_cr_2, obj_color_3);
        }
        self
    }
    ///
    /// **Overloads**
    /// Sets the color used for the given color *role,* in all color
    /// groups, to the specified solid *color.*
    ///
    /// **See also:** [`brush()`]
    /// [`set_color()`]
    /// ColorRole
    ///
    /// Sets the color in the specified color *group,* used for the given
    /// color *role,* to the specified solid *color.*
    ///
    /// **See also:** [`set_brush()`]
    /// [`color()`]
    /// ColorRole
    ///
    /// Sets a the group at *cg.* You can pass either brushes, pixmaps or
    /// plain colors for *windowText,* *button,* *light,* *dark,* *mid,* *text,* *bright_text,* *base* and *window.*
    ///
    /// **See also:** [`Brush`]
    pub fn set_color_2<C: ColorTrait<'a>>(&self, cr: ColorRole, color: &C) -> &Self {
        let enum_cr_1 = cr as i32;
        let (obj_color_2, _funcs) = color.get_color_obj_funcs();

        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            ((*funcs).set_color_2)(obj_data, enum_cr_1, obj_color_2);
        }
        self
    }
    ///
    /// Sets the brush for the given color *role* to the specified *brush* for all groups in the palette.
    ///
    /// **See also:** [`brush()`]
    /// [`set_color()`]
    /// ColorRole
    ///
    /// **Overloads**
    /// Sets the brush in the specified color *group,* used for the given
    /// color *role,* to *brush.*
    ///
    /// **See also:** [`brush()`]
    /// [`set_color()`]
    /// ColorRole
    pub fn set_brush<B: BrushTrait<'a>>(&self, cr: ColorRole, brush: &B) -> &Self {
        let enum_cr_1 = cr as i32;
        let (obj_brush_2, _funcs) = brush.get_brush_obj_funcs();

        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            ((*funcs).set_brush)(obj_data, enum_cr_1, obj_brush_2);
        }
        self
    }
    ///
    /// Returns `true` if the ColorGroup *cg* and ColorRole *cr* has been
    /// set previously on this palette; otherwise returns `false.`
    ///
    /// **See also:** [`set_brush()`]
    pub fn is_brush_set(&self, cg: ColorGroup, cr: ColorRole) -> bool {
        let enum_cg_1 = cg as i32;
        let enum_cr_2 = cr as i32;

        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_brush_set)(obj_data, enum_cg_1, enum_cr_2);
            ret_val
        }
    }
    ///
    /// Sets the brush for the given color *role* to the specified *brush* for all groups in the palette.
    ///
    /// **See also:** [`brush()`]
    /// [`set_color()`]
    /// ColorRole
    ///
    /// **Overloads**
    /// Sets the brush in the specified color *group,* used for the given
    /// color *role,* to *brush.*
    ///
    /// **See also:** [`brush()`]
    /// [`set_color()`]
    /// ColorRole
    pub fn set_brush_2<B: BrushTrait<'a>>(
        &self,
        cg: ColorGroup,
        cr: ColorRole,
        brush: &B,
    ) -> &Self {
        let enum_cg_1 = cg as i32;
        let enum_cr_2 = cr as i32;
        let (obj_brush_3, _funcs) = brush.get_brush_obj_funcs();

        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            ((*funcs).set_brush_2)(obj_data, enum_cg_1, enum_cr_2, obj_brush_3);
        }
        self
    }
    ///
    /// Sets a the group at *cg.* You can pass either brushes, pixmaps or
    /// plain colors for *windowText,* *button,* *light,* *dark,* *mid,* *text,* *bright_text,* *base* and *window.*
    ///
    /// **See also:** [`Brush`]
    pub fn set_color_group<B: BrushTrait<'a>>(
        &self,
        cr: ColorGroup,
        window_text: &B,
        button: &B,
        light: &B,
        dark: &B,
        mid: &B,
        text: &B,
        bright_text: &B,
        base: &B,
        window: &B,
    ) -> &Self {
        let enum_cr_1 = cr as i32;
        let (obj_window_text_2, _funcs) = window_text.get_brush_obj_funcs();
        let (obj_button_3, _funcs) = button.get_brush_obj_funcs();
        let (obj_light_4, _funcs) = light.get_brush_obj_funcs();
        let (obj_dark_5, _funcs) = dark.get_brush_obj_funcs();
        let (obj_mid_6, _funcs) = mid.get_brush_obj_funcs();
        let (obj_text_7, _funcs) = text.get_brush_obj_funcs();
        let (obj_bright_text_8, _funcs) = bright_text.get_brush_obj_funcs();
        let (obj_base_9, _funcs) = base.get_brush_obj_funcs();
        let (obj_window_10, _funcs) = window.get_brush_obj_funcs();

        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            ((*funcs).set_color_group)(
                obj_data,
                enum_cr_1,
                obj_window_text_2,
                obj_button_3,
                obj_light_4,
                obj_dark_5,
                obj_mid_6,
                obj_text_7,
                obj_bright_text_8,
                obj_base_9,
                obj_window_10,
            );
        }
        self
    }
    ///
    /// Returns `true` (usually quickly) if color group *cg1* is equal to
    /// *cg2;* otherwise returns `false.`
    pub fn is_equal(&self, cr1: ColorGroup, cr2: ColorGroup) -> bool {
        let enum_cr1_1 = cr1 as i32;
        let enum_cr2_2 = cr2 as i32;

        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_equal)(obj_data, enum_cr1_1, enum_cr2_2);
            ret_val
        }
    }
    ///
    /// **Overloads**
    /// Returns the color that has been set for the given color *role* in
    /// the current ColorGroup.
    ///
    /// **See also:** [`brush()`]
    /// ColorRole
    ///
    /// Returns the color in the specified color *group,* used for the
    /// given color *role.*
    ///
    /// **See also:** [`brush()`]
    /// [`set_color()`]
    /// ColorRole
    pub fn color_2(&self, cr: ColorRole) -> Option<Color> {
        let enum_cr_1 = cr as i32;

        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).color_2)(obj_data, enum_cr_1);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Color::new_from_rc(t);
            } else {
                ret_val = Color::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// **Overloads**
    /// Returns the brush that has been set for the given color *role* in
    /// the current ColorGroup.
    ///
    /// **See also:** [`color()`]
    /// [`set_brush()`]
    /// ColorRole
    ///
    /// Returns the brush in the specified color *group,* used for the
    /// given color *role.*
    ///
    /// **See also:** [`color()`]
    /// [`set_brush()`]
    /// ColorRole
    pub fn brush_2(&self, cr: ColorRole) -> Option<Brush> {
        let enum_cr_1 = cr as i32;

        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).brush_2)(obj_data, enum_cr_1);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Use windowText() instead.
    pub fn foreground(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).foreground)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the window text (general foreground) brush of the
    /// current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn window_text(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).window_text)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the button brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    ///
    /// Returns the button text foreground brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn button(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).button)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the light brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn light(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).light)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the dark brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn dark(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).dark)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the midlight brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    ///
    /// Returns the mid brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn mid(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).mid)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the text foreground brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn text(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).text)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the base brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn base(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).base)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the alternate base brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn alternate_base(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).alternate_base)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the tool tip base brush of the current color group. This brush is
    /// used by QToolTip and QWhatsThis.
    ///
    /// **Note**: Tool tips use the Inactive color group of QPalette, because tool
    /// tips are not active windows.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn tool_tip_base(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).tool_tip_base)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the tool tip text brush of the current color group. This brush is
    /// used by QToolTip and QWhatsThis.
    ///
    /// **Note**: Tool tips use the Inactive color group of QPalette, because tool
    /// tips are not active windows.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn tool_tip_text(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).tool_tip_text)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Use window() instead.
    pub fn background(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).background)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the window text (general foreground) brush of the
    /// current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    ///
    /// Returns the window (general background) brush of the current
    /// color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn window(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).window)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the midlight brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn midlight(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).midlight)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the bright text foreground brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn bright_text(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).bright_text)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the button text foreground brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn button_text(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).button_text)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the shadow brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn shadow(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).shadow)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the highlight brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    ///
    /// Returns the highlighted text brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn highlight(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).highlight)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the highlighted text brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn highlighted_text(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).highlighted_text)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the unvisited link text brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    ///
    /// Returns the visited link text brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn link(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).link)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the visited link text brush of the current color group.
    ///
    /// **See also:** ColorRole
    /// [`brush()`]
    pub fn link_visited(&self) -> Option<Brush> {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).link_visited)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns `true` if this palette and *p* are copies of each other,
    /// i.e. one of them was created as a copy of the other and neither
    /// was subsequently modified; otherwise returns `false.` This is much
    /// stricter than equality.
    ///
    /// **See also:** [`operator()`]
    /// [`operator()`]
    pub fn is_copy_of<P: PaletteTrait<'a>>(&self, p: &P) -> bool {
        let (obj_p_1, _funcs) = p.get_palette_obj_funcs();

        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_copy_of)(obj_data, obj_p_1);
            ret_val
        }
    }
    ///
    /// Returns a number that identifies the contents of this QPalette
    /// object. Distinct QPalette objects can have the same key if
    /// they refer to the same contents.
    ///
    /// The cacheKey() will change when the palette is altered.
    pub fn cache_key(&self) -> i64 {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).cache_key)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns a new QPalette that has attributes copied from *other.*
    pub fn resolve<P: PaletteTrait<'a>>(&self, arg0: &P) -> Palette {
        let (obj_arg0_1, _funcs) = arg0.get_palette_obj_funcs();

        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).resolve)(obj_data, obj_arg0_1);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Palette::new_from_rc(t);
            } else {
                ret_val = Palette::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns a new QPalette that has attributes copied from *other.*
    pub fn resolve_2(&self) -> u32 {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).resolve_2)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns a new QPalette that has attributes copied from *other.*
    pub fn resolve_3(&self, mask: u32) -> &Self {
        let (obj_data, funcs) = self.get_palette_obj_funcs();
        unsafe {
            ((*funcs).resolve_3)(obj_data, mask);
        }
        self
    }
}
pub trait PaletteTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_palette_obj_funcs(&self) -> (*const RUBase, *const RUPaletteFuncs);
}

impl<'a> PaletteTrait<'a> for Palette<'a> {
    #[doc(hidden)]
    fn get_palette_obj_funcs(&self) -> (*const RUBase, *const RUPaletteFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).palette_funcs) }
    }
}
#[repr(u32)]
pub enum ColorGroup {
    Active,
    Disabled,
    Inactive,
    NColorGroups,
    Current,
    All,
    Normal,
}

#[repr(u32)]
pub enum ColorRole {
    WindowText,
    Button,
    Light,
    Midlight,
    Dark,
    Mid,
    Text,
    BrightText,
    ButtonText,
    Base,
    Window,
    Shadow,
    Highlight,
    HighlightedText,
    Link,
    LinkVisited,
    AlternateBase,
    NoRole,
    ToolTipBase,
    ToolTipText,
    NColorRoles,
    Foreground,
    Background,
}
