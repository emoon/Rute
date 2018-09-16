
use std::cell::Cell;
use std::marker::PhantomData;
use std::mem::transmute;
use std::os::raw::{c_void, c_char};
use std::cell::RefCell;
use std::rc::Rc;
use std::ffi::CString;
use std::ffi::CStr;
use auto::rute_enums::*;


#[derive(Clone)]
pub struct ListWidgetItem<'a> {
    data: Rc<Cell<Option<RUListWidgetItem>>>,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

#[derive(Clone)]
pub struct ListWidget<'a> {
    data: Rc<Cell<Option<RUListWidget>>>,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

pub trait ListWidgetItemType {

    pub fn set_text(&self, text: &str) -> &Self {
        let str_in_text_1 = CString::new(text).unwrap();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_text)(obj_data, str_in_text_1.as_ptr());
        }
        self
    }

    pub fn text(&self) -> &str {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).text)(obj_data);
           let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }

    fn get_list_widget_item_obj_funcs(&self) -> (*const RUBase, *const RUListWidgetItemFuncs);
}

impl<'a> ListWidgetItemType for ListWidgetItem<'a> {
    fn get_list_widget_item_obj_funcs(&self) -> (*const RUBase, *const RUListWidgetItemFuncs) {
        let obj = self.data.get().unwrap();
        (obj, self.all_funcs.list_widget_item_funcs)
    }
}
impl<'a> ListWidget<'a> {
    unsafe extern "C" fn item_activated_trampoline_ud<T>(
        user_data: *const c_void,
        func: *const c_void,
        item: ListWidgetItem
    ) {
        let f: &&(Fn(&T, ListWidgetItem) + 'static) = transmute(func);
        let data = user_data as *const T;
        f(&*data, item);
    }

    pub fn set_item_activated_event_ud<F, T>(&self, data: &'a T, func: F) -> &ListWidget<'a>
    where
        F: Fn(&T, ListWidgetItem) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_list_widget_obj_funcs();

        let f: Box<Box<Fn(&T, ListWidgetItem) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_item_activated_event)(
                obj_data,
                user_data,
                transmute(Self::item_activated_trampoline_ud::<T> as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }

    unsafe extern "C" fn item_activated_trampoline(
        user_data: *const c_void,
        func: *const c_void,
        item: ListWidgetItem
    ) {
        let f: &&(Fn(ListWidgetItem) + 'static) = transmute(func);
        f(item);
    }

    pub fn set_item_activated_event<F>(&self, func: F) -> &ListWidget<'a>
    where
        F: Fn(ListWidgetItem) + 'a,
    {
        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
        let f: Box<Box<Fn(ListWidgetItem) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_item_activated_event)(
                obj_data,
                ::std::ptr::null(),
                transmute(Self::item_activated_trampoline as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }

    unsafe extern "C" fn current_row_changed_trampoline_ud<T>(
        user_data: *const c_void,
        func: *const c_void,
        row: i32
    ) {
        let f: &&(Fn(&T, i32) + 'static) = transmute(func);
        let data = user_data as *const T;
        f(&*data, row);
    }

    pub fn set_current_row_changed_event_ud<F, T>(&self, data: &'a T, func: F) -> &ListWidget<'a>
    where
        F: Fn(&T, i32) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_list_widget_obj_funcs();

        let f: Box<Box<Fn(&T, i32) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_current_row_changed_event)(
                obj_data,
                user_data,
                transmute(Self::current_row_changed_trampoline_ud::<T> as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }

    unsafe extern "C" fn current_row_changed_trampoline(
        user_data: *const c_void,
        func: *const c_void,
        row: i32
    ) {
        let f: &&(Fn(i32) + 'static) = transmute(func);
        f(row);
    }

    pub fn set_current_row_changed_event<F>(&self, func: F) -> &ListWidget<'a>
    where
        F: Fn(i32) + 'a,
    {
        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
        let f: Box<Box<Fn(i32) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_current_row_changed_event)(
                obj_data,
                ::std::ptr::null(),
                transmute(Self::current_row_changed_trampoline as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }
}
pub trait ListWidgetType {

    pub fn clear(&self) -> &Self {

        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
        unsafe {
            ((*funcs).clear)(obj_data);
        }
        self
    }

    pub fn current_item(&self) -> Option<ListWidgetItem> {

        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).current_item)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = ListWidgetItem {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<RUListWidgetItem>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = ListWidgetItem {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            Some(ret_val)
        }
    }

    pub fn current_row(&self) -> i32 {

        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).current_row)(obj_data);
            ret_val
        }
    }

    pub fn set_current_row(&self, index: i32) -> &Self {

        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
        unsafe {
            ((*funcs).set_current_row)(obj_data, index);
        }
        self
    }

    pub fn count(&self) -> i32 {

        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).count)(obj_data);
            ret_val
        }
    }

    pub fn set_drag_enabled(&self, state: bool) -> &Self {

        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
        unsafe {
            ((*funcs).set_drag_enabled)(obj_data, state);
        }
        self
    }

    pub fn set_drop_indicator_shown(&self, state: bool) -> &Self {

        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
        unsafe {
            ((*funcs).set_drop_indicator_shown)(obj_data, state);
        }
        self
    }

    fn get_list_widget_obj_funcs(&self) -> (*const RUBase, *const RUListWidgetFuncs);
}

impl<'a> ListWidgetType for ListWidget<'a> {
    fn get_list_widget_obj_funcs(&self) -> (*const RUBase, *const RUListWidgetFuncs) {
        let obj = self.data.get().unwrap();
        (obj, self.all_funcs.list_widget_funcs)
    }
}
