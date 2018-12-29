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
/// A ListWidgetItem represents a single item in a ListWidget. Each item can
/// hold several pieces of information, and will display them appropriately.
///
/// The item view convenience classes use a classic item-based interface rather
/// than a pure model/view approach. For a more flexible list view widget,
/// consider using the ListView class with a standard model.
///
/// List items can be inserted automatically into a list, when they are
/// constructed, by specifying the list widget:
///
/// Alternatively, list items can also be created without a parent widget, and
/// later inserted into a list using QListWidget::insertItem().
///
/// List items are typically used to display text() and an icon(). These are
/// set with the setText() and setIcon() functions. The appearance of the text
/// can be customized with setFont(), setForeground(), and setBackground().
/// Text in list items can be aligned using the setTextAlignment() function.
/// Tooltips, status tips and help can be added to list items
/// with setToolTip(), setStatusTip(), and setWhatsThis().
///
/// By default, items are enabled, selectable, checkable, and can be the source
/// of drag and drop operations.
///
/// Each item's flags can be changed by calling setFlags() with the appropriate
/// value (see Qt::ItemFlags). Checkable items can be checked, unchecked and
/// partially checked with the setCheckState() function. The corresponding
/// checkState() function indicates the item's current check state.
///
/// The isHidden() function can be used to determine whether the item is
/// hidden. To hide an item, use setHidden().
///
/// # Subclassing
///
/// When subclassing QListWidgetItem to provide custom items, it is possible to
/// define new types for them enabling them to be distinguished from standard
/// items. For subclasses that require this feature, ensure that you call the
/// base class constructor with a new type value equal to or greater than
/// [UserType,](UserType,)
/// within *your* constructor.
///
/// **See also:** [`ListWidget`]
/// {Model/View Programming}
/// [`TreeWidgetItem`]
/// [`TableWidgetItem`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct ListWidgetItem<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUListWidgetItemAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> ListWidgetItem<'a> {
    pub fn new() -> ListWidgetItem<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_list_widget_item)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        ListWidgetItem {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUListWidgetItem) -> ListWidgetItem<'a> {
        let data =
            unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) };
        let t = ListWidgetItem {
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
    pub(crate) fn new_from_owned(ffi_data: RUListWidgetItem) -> ListWidgetItem<'a> {
        ListWidgetItem {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUListWidgetItem) -> ListWidgetItem<'a> {
        ListWidgetItem {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Returns the list widget containing the item.
    pub fn list_widget(&self) -> Option<ListWidget> {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).list_widget)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = ListWidget::new_from_rc(t);
            } else {
                ret_val = ListWidget::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Sets the selected state of the item to *select.*
    ///
    /// **See also:** [`is_selected()`]
    pub fn set_selected(&self, select: bool) -> &Self {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_selected)(obj_data, select);
        }
        self
    }
    ///
    /// Returns `true` if the item is selected; otherwise returns `false.`
    ///
    /// **See also:** [`set_selected()`]
    pub fn is_selected(&self) -> bool {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_selected)(obj_data);
            ret_val
        }
    }
    ///
    /// Hides the item if *hide* is true; otherwise shows the item.
    ///
    /// **See also:** [`is_hidden()`]
    pub fn set_hidden(&self, hide: bool) -> &Self {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_hidden)(obj_data, hide);
        }
        self
    }
    ///
    /// Returns `true` if the item is hidden; otherwise returns `false.`
    ///
    /// **See also:** [`set_hidden()`]
    pub fn is_hidden(&self) -> bool {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_hidden)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the item flags for this item (see [Qt::ItemFlags](Qt::ItemFlags)
    /// ).
    pub fn flags(&self) -> ItemFlags {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).flags)(obj_data);
            let ret_val = ItemFlags::from_bits_truncate(ret_val);
            ret_val
        }
    }
    ///
    /// Sets the item flags for the list item to *flags.*
    ///
    /// **See also:** [`t::item_flags()`]
    pub fn set_flags(&self, flags: ItemFlags) -> &Self {
        let enum_flags_1 = flags.bits();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_flags)(obj_data, enum_flags_1);
        }
        self
    }
    ///
    /// Returns the list item's text.
    ///
    /// **See also:** [`set_text()`]
    ///
    /// Returns the text alignment for the list item.
    ///
    /// **See also:** [`t::alignment_flag()`]
    ///
    /// Returns the color used to display the list item's text.
    ///
    /// This function is deprecated. Use foreground() instead.
    pub fn text(&self) -> String {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).text)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }
    ///
    /// Sets the text for the list widget item's to the given *text.*
    ///
    /// **See also:** [`text()`]
    ///
    /// Sets the list item's text alignment to *alignment.*
    ///
    /// **See also:** [`t::alignment_flag()`]
    ///
    /// This function is deprecated. Use setForeground() instead.
    pub fn set_text(&self, text: &str) -> &Self {
        let str_in_text_1 = CString::new(text).unwrap();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_text)(obj_data, str_in_text_1.as_ptr());
        }
        self
    }
    ///
    /// Returns the list item's icon.
    ///
    /// **See also:** [`set_icon()`]
    /// {QAbstractItemView::iconSize}{iconSize}
    pub fn icon(&self) -> Icon {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).icon)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Icon::new_from_rc(t);
            } else {
                ret_val = Icon::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Sets the icon for the list item to the given *icon.*
    ///
    /// **See also:** [`icon()`]
    /// [`text()`]
    /// {QAbstractItemView::iconSize}{iconSize}
    pub fn set_icon<I: IconTrait<'a>>(&self, icon: &I) -> &Self {
        let (obj_icon_1, _funcs) = icon.get_icon_obj_funcs();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_icon)(obj_data, obj_icon_1);
        }
        self
    }
    ///
    /// Returns the list item's status tip.
    ///
    /// **See also:** [`set_status_tip()`]
    pub fn status_tip(&self) -> String {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).status_tip)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }
    ///
    /// Sets the status tip for the list item to the text specified by
    /// *statusTip.* QListWidget mouseTracking needs to be enabled for this
    /// feature to work.
    ///
    /// **See also:** [`status_tip()`]
    /// [`set_tool_tip()`]
    /// [`set_whats_this()`]
    /// [`Widget::set_mouse_tracking`]
    pub fn set_status_tip(&self, status_tip: &str) -> &Self {
        let str_in_status_tip_1 = CString::new(status_tip).unwrap();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_status_tip)(obj_data, str_in_status_tip_1.as_ptr());
        }
        self
    }
    ///
    /// Returns the list item's tooltip.
    ///
    /// **See also:** [`set_tool_tip()`]
    /// [`status_tip()`]
    /// [`whats_this()`]
    pub fn tool_tip(&self) -> String {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).tool_tip)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }
    ///
    /// Sets the tooltip for the list item to the text specified by *toolTip.*
    ///
    /// **See also:** [`tool_tip()`]
    /// [`set_status_tip()`]
    /// [`set_whats_this()`]
    pub fn set_tool_tip(&self, tool_tip: &str) -> &Self {
        let str_in_tool_tip_1 = CString::new(tool_tip).unwrap();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_tool_tip)(obj_data, str_in_tool_tip_1.as_ptr());
        }
        self
    }
    ///
    /// Returns the list item's help text.
    ///
    /// **See also:** [`set_whats_this()`]
    /// [`status_tip()`]
    /// [`tool_tip()`]
    pub fn whats_this(&self) -> String {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).whats_this)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }
    ///
    /// Sets the help for the list item to the text specified by
    /// *whatsThis.*
    ///
    /// **See also:** [`whats_this()`]
    /// [`set_status_tip()`]
    /// [`set_tool_tip()`]
    pub fn set_whats_this(&self, whats_this: &str) -> &Self {
        let str_in_whats_this_1 = CString::new(whats_this).unwrap();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_whats_this)(obj_data, str_in_whats_this_1.as_ptr());
        }
        self
    }
    ///
    /// Returns the font used to display this list item's text.
    pub fn font(&self) -> Font {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).font)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Font::new_from_rc(t);
            } else {
                ret_val = Font::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Sets the font used when painting the item to the given *font.*
    pub fn set_font<F: FontTrait<'a>>(&self, font: &F) -> &Self {
        let (obj_font_1, _funcs) = font.get_font_obj_funcs();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_font)(obj_data, obj_font_1);
        }
        self
    }
    ///
    /// Returns the text alignment for the list item.
    ///
    /// **See also:** [`t::alignment_flag()`]
    pub fn text_alignment(&self) -> i32 {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).text_alignment)(obj_data);
            ret_val
        }
    }
    ///
    /// Sets the list item's text alignment to *alignment.*
    ///
    /// **See also:** [`t::alignment_flag()`]
    pub fn set_text_alignment(&self, alignment: i32) -> &Self {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_text_alignment)(obj_data, alignment);
        }
        self
    }
    ///
    /// Returns the brush used to display the list item's background.
    ///
    /// **See also:** [`set_background()`]
    /// [`foreground()`]
    pub fn background(&self) -> Brush {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).background)(obj_data);
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
    /// This function is deprecated. Use setBackground() instead.
    ///
    /// Sets the background brush of the list item to the given *brush.*
    ///
    /// **See also:** [`background()`]
    /// [`set_foreground()`]
    pub fn set_background<B: BrushTrait<'a>>(&self, brush: &B) -> &Self {
        let (obj_brush_1, _funcs) = brush.get_brush_obj_funcs();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_background)(obj_data, obj_brush_1);
        }
        self
    }
    ///
    /// Returns the color used to display the list item's text.
    ///
    /// This function is deprecated. Use foreground() instead.
    pub fn text_color(&self) -> Color {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).text_color)(obj_data);
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
    /// This function is deprecated. Use setForeground() instead.
    pub fn set_text_color<C: ColorTrait<'a>>(&self, color: &C) -> &Self {
        let (obj_color_1, _funcs) = color.get_color_obj_funcs();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_text_color)(obj_data, obj_color_1);
        }
        self
    }
    ///
    /// Returns the brush used to display the list item's foreground (e.g. text).
    ///
    /// **See also:** [`set_foreground()`]
    /// [`background()`]
    pub fn foreground(&self) -> Brush {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).foreground)(obj_data);
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
    /// Sets the foreground brush of the list item to the given *brush.*
    ///
    /// **See also:** [`foreground()`]
    /// [`set_background()`]
    pub fn set_foreground<B: BrushTrait<'a>>(&self, brush: &B) -> &Self {
        let (obj_brush_1, _funcs) = brush.get_brush_obj_funcs();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_foreground)(obj_data, obj_brush_1);
        }
        self
    }
    ///
    /// Returns the checked state of the list item (see [Qt::CheckState](Qt::CheckState)
    /// ).
    ///
    /// **See also:** [`flags()`]
    pub fn check_state(&self) -> CheckState {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).check_state)(obj_data);
            let ret_val = { transmute::<u32, CheckState>(ret_val) };
            ret_val
        }
    }
    ///
    /// Sets the check state of the list item to *state.*
    ///
    /// **See also:** [`check_state()`]
    pub fn set_check_state(&self, state: CheckState) -> &Self {
        let enum_state_1 = state as u32;

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_check_state)(obj_data, enum_state_1);
        }
        self
    }
    ///
    /// Returns the size hint set for the list item.
    pub fn size_hint(&self) -> Size {
        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
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
    /// Sets the size hint for the list item to be *size.* If no size hint is set,
    /// the item delegate will compute the size hint based on the item data.
    pub fn set_size_hint<S: SizeTrait<'a>>(&self, size: &S) -> &Self {
        let (obj_size_1, _funcs) = size.get_size_obj_funcs();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_size_hint)(obj_data, obj_size_1);
        }
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

impl<'a> From<WrapperRcOwn> for ListWidgetItem<'a> {
    fn from(t: WrapperRcOwn) -> Self {
        let mut data = RUListWidgetItem {
            qt_data: ::std::ptr::null(),
            host_data: ::std::ptr::null(),
            all_funcs: t.all_funcs as *const RUListWidgetItemAllFuncs,
        };

        if t.owned {
            data.host_data = t.data as *const RUBase;
            ListWidgetItem::new_from_rc(data)
        } else {
            data.qt_data = t.data as *const RUBase;
            ListWidgetItem::new_from_temporary(data)
        }
    }
}

pub trait ListWidgetItemTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_list_widget_item_obj_funcs(&self) -> (*const RUBase, *const RUListWidgetItemFuncs);
}

impl<'a> ListWidgetItemTrait<'a> for ListWidgetItem<'a> {
    #[doc(hidden)]
    fn get_list_widget_item_obj_funcs(&self) -> (*const RUBase, *const RUListWidgetItemFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).list_widget_item_funcs) }
    }
}
#[repr(u32)]
pub enum ItemType {
    Type = 0,
    UserType = 1000,
}
