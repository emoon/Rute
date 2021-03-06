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
/// Mouse events occur when a mouse button is pressed or released
/// inside a widget, or when the mouse cursor is moved.
///
/// Mouse move events will occur only when a mouse button is pressed
/// down, unless mouse tracking has been enabled with
/// QWidget::setMouseTracking().
///
/// Qt automatically grabs the mouse when a mouse button is pressed
/// inside a widget; the widget will continue to receive mouse events
/// until the last mouse button is released.
///
/// A mouse event contains a special accept flag that indicates
/// whether the receiver wants the event. You should call ignore() if
/// the mouse event is not handled by your widget. A mouse event is
/// propagated up the parent widget chain until a widget accepts it
/// with accept(), or an event filter consumes it.
///
/// **Note**: If a mouse event is propagated to a [widget](QWidget)
/// for
/// which Qt::WA_NoMousePropagation has been set, that mouse event
/// will not be propagated further up the parent widget chain.
///
/// The state of the keyboard modifier keys can be found by calling the
/// [modifiers()](QInputEvent::modifiers())
/// function, inherited from
/// QInputEvent.
///
/// The functions pos(), x(), and y() give the cursor position
/// relative to the widget that receives the mouse event. If you
/// move the widget as a result of the mouse event, use the global
/// position returned by globalPos() to avoid a shaking motion.
///
/// The QWidget::setEnabled() function can be used to enable or
/// disable mouse and keyboard events for a widget.
///
/// Reimplement the QWidget event handlers, QWidget::mousePressEvent(),
/// QWidget::mouseReleaseEvent(), QWidget::mouseDoubleClickEvent(),
/// and QWidget::mouseMoveEvent() to receive mouse events in your own
/// widgets.
///
/// **See also:** [`Widget::set_mouse_tracking`]
/// [`Widget::grab_mouse`]
/// [`Cursor::pos`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct MouseEvent<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUMouseEventAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> MouseEvent<'a> {
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUMouseEvent) -> MouseEvent<'a> {
        MouseEvent {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUMouseEvent) -> MouseEvent<'a> {
        MouseEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUMouseEvent) -> MouseEvent<'a> {
        MouseEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Returns the position of the mouse cursor, relative to the widget
    /// that received the event.
    ///
    /// If you move the widget as a result of the mouse event, use the
    /// global position returned by globalPos() to avoid a shaking
    /// motion.
    ///
    /// **See also:** [`x()`]
    /// [`y()`]
    /// [`global_pos()`]
    ///
    /// Use localPos() instead.
    pub fn pos(&self) -> Point {
        let (obj_data, funcs) = self.get_mouse_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).pos)(obj_data);
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
    /// Returns the global position of the mouse cursor *at the time
    /// of the event* . This is important on asynchronous window systems
    /// like X11. Whenever you move your widgets around in response to
    /// mouse events, globalPos() may differ a lot from the current
    /// pointer position QCursor::pos(), and from
    /// QWidget::mapToGlobal(pos()).
    ///
    /// **See also:** [`global_x()`]
    /// [`global_y()`]
    pub fn global_pos(&self) -> Point {
        let (obj_data, funcs) = self.get_mouse_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).global_pos)(obj_data);
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
    /// Returns the x position of the mouse cursor, relative to the
    /// widget that received the event.
    ///
    /// **See also:** [`y()`]
    /// [`pos()`]
    pub fn x(&self) -> i32 {
        let (obj_data, funcs) = self.get_mouse_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).x)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the y position of the mouse cursor, relative to the
    /// widget that received the event.
    ///
    /// **See also:** [`x()`]
    /// [`pos()`]
    pub fn y(&self) -> i32 {
        let (obj_data, funcs) = self.get_mouse_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).y)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the global x position of the mouse cursor at the time of
    /// the event.
    ///
    /// **See also:** [`global_y()`]
    /// [`global_pos()`]
    pub fn global_x(&self) -> i32 {
        let (obj_data, funcs) = self.get_mouse_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).global_x)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the global y position of the mouse cursor at the time of
    /// the event.
    ///
    /// **See also:** [`global_x()`]
    /// [`global_pos()`]
    pub fn global_y(&self) -> i32 {
        let (obj_data, funcs) = self.get_mouse_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).global_y)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the position of the mouse cursor as a QPointF, relative to the
    /// widget or item that received the event.
    ///
    /// If you move the widget as a result of the mouse event, use the
    /// screen position returned by screenPos() to avoid a shaking
    /// motion.
    ///
    /// **See also:** [`x()`]
    /// [`y()`]
    /// [`window_pos()`]
    /// [`screen_pos()`]
    pub fn local_pos(&self) -> Option<PointF> {
        let (obj_data, funcs) = self.get_mouse_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).local_pos)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = PointF::new_from_rc(t);
            } else {
                ret_val = PointF::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the position of the mouse cursor as a QPointF, relative to the
    /// window that received the event.
    ///
    /// If you move the widget as a result of the mouse event, use the
    /// global position returned by globalPos() to avoid a shaking
    /// motion.
    ///
    /// **See also:** [`x()`]
    /// [`y()`]
    /// [`pos()`]
    /// [`local_pos()`]
    /// [`screen_pos()`]
    pub fn window_pos(&self) -> Option<PointF> {
        let (obj_data, funcs) = self.get_mouse_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).window_pos)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = PointF::new_from_rc(t);
            } else {
                ret_val = PointF::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the position of the mouse cursor as a QPointF, relative to the
    /// screen that received the event.
    ///
    /// **See also:** [`x()`]
    /// [`y()`]
    /// [`pos()`]
    /// [`local_pos()`]
    /// [`window_pos()`]
    pub fn screen_pos(&self) -> Option<PointF> {
        let (obj_data, funcs) = self.get_mouse_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).screen_pos)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = PointF::new_from_rc(t);
            } else {
                ret_val = PointF::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the button that caused the event.
    ///
    /// Note that the returned value is always Qt::NoButton for mouse
    /// move events.
    ///
    /// **See also:** [`buttons()`]
    /// [`t::mouse_button()`]
    ///
    /// Returns the button state when the event was generated. The button
    /// state is a combination of Qt::LeftButton, Qt::RightButton,
    /// Qt::MidButton using the OR operator. For mouse move events,
    /// this is all buttons that are pressed down. For mouse press and
    /// double click events this includes the button that caused the
    /// event. For mouse release events this excludes the button that
    /// caused the event.
    ///
    /// **See also:** [`button()`]
    /// [`t::mouse_button()`]
    pub fn button(&self) -> MouseButton {
        let (obj_data, funcs) = self.get_mouse_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).button)(obj_data);
            let ret_val = MouseButton::from_bits_truncate(ret_val);
            ret_val
        }
    }
    ///
    /// Returns the button state when the event was generated. The button
    /// state is a combination of Qt::LeftButton, Qt::RightButton,
    /// Qt::MidButton using the OR operator. For mouse move events,
    /// this is all buttons that are pressed down. For mouse press and
    /// double click events this includes the button that caused the
    /// event. For mouse release events this excludes the button that
    /// caused the event.
    ///
    /// **See also:** [`button()`]
    /// [`t::mouse_button()`]
    pub fn buttons(&self) -> MouseButtons {
        let (obj_data, funcs) = self.get_mouse_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).buttons)(obj_data);
            let ret_val = MouseButtons::from_bits_truncate(ret_val);
            ret_val
        }
    }
    pub fn set_local_pos<P: PointFTrait<'a>>(&self, local_position: &P) -> &Self {
        let (obj_local_position_1, _funcs) = local_position.get_point_f_obj_funcs();

        let (obj_data, funcs) = self.get_mouse_event_obj_funcs();
        unsafe {
            ((*funcs).set_local_pos)(obj_data, obj_local_position_1);
        }
        self
    }
    ///
    /// Returns information about the mouse event source.
    ///
    /// The mouse event source can be used to distinguish between genuine
    /// and artificial mouse events. The latter are events that are
    /// synthesized from touch events by the operating system or Qt itself.
    ///
    /// **Note**: Many platforms provide no such information. On such platforms
    /// [Qt::MouseEventNotSynthesized](Qt::MouseEventNotSynthesized)
    /// is returned always.
    ///
    /// **See also:** [`t::mouse_event_source()`]
    /// **See also:** [`GraphicsSceneMouseEvent::source`]
    pub fn source(&self) -> MouseEventSource {
        let (obj_data, funcs) = self.get_mouse_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).source)(obj_data);
            let ret_val = { transmute::<u32, MouseEventSource>(ret_val) };
            ret_val
        }
    }
    ///
    /// Returns the mouse event flags.
    ///
    /// The mouse event flags provide additional information about a mouse event.
    ///
    /// **See also:** [`t::mouse_event_flag()`]
    /// **See also:** [`GraphicsSceneMouseEvent::flags`]
    pub fn flags(&self) -> MouseEventFlags {
        let (obj_data, funcs) = self.get_mouse_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).flags)(obj_data);
            let ret_val = MouseEventFlags::from_bits_truncate(ret_val);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn modifiers(&self) -> KeyboardModifiers {
        let (obj_data, funcs) = self.get_input_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).modifiers)(obj_data);
            let ret_val = KeyboardModifiers::from_bits_truncate(ret_val);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn set_modifiers(&self, amodifiers: KeyboardModifiers) -> &Self {
        let enum_amodifiers_1 = amodifiers.bits();

        let (obj_data, funcs) = self.get_input_event_obj_funcs();
        unsafe {
            ((*funcs).set_modifiers)(obj_data, enum_amodifiers_1);
        }
        self
    }
    #[doc(hidden)]
    pub fn timestamp(&self) -> u64 {
        let (obj_data, funcs) = self.get_input_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).timestamp)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn set_timestamp(&self, atimestamp: u64) -> &Self {
        let (obj_data, funcs) = self.get_input_event_obj_funcs();
        unsafe {
            ((*funcs).set_timestamp)(obj_data, atimestamp);
        }
        self
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

    pub fn build(&self) -> Self {
        self.clone()
    }
}
pub trait MouseEventTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_mouse_event_obj_funcs(&self) -> (*const RUBase, *const RUMouseEventFuncs);
}

impl<'a> EventTrait<'a> for MouseEvent<'a> {
    #[doc(hidden)]
    fn get_event_obj_funcs(&self) -> (*const RUBase, *const RUEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).event_funcs) }
    }
}

impl<'a> InputEventTrait<'a> for MouseEvent<'a> {
    #[doc(hidden)]
    fn get_input_event_obj_funcs(&self) -> (*const RUBase, *const RUInputEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).input_event_funcs) }
    }
}

impl<'a> MouseEventTrait<'a> for MouseEvent<'a> {
    #[doc(hidden)]
    fn get_mouse_event_obj_funcs(&self) -> (*const RUBase, *const RUMouseEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).mouse_event_funcs) }
    }
}
