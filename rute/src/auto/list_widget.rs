
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
pub struct ListWidget<'a> {
    data: Rc<Cell<Option<RUListWidget>>>,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> ListWidget<'a> {
    unsafe extern "C" fn item_pressed_trampoline_ud<T>(
        user_data: *const c_void,
        func: *const c_void,
        item: ListWidgetItemType
    ) {
        let f: &&(Fn(&T, ListWidgetItemType) + 'static) = transmute(func);
        let data = user_data as *const T;
        f(&*data, item);
    }

    pub fn set_item_pressed_event_ud<F, T>(&self, data: &'a T, func: F) -> &ListWidget<'a>
    where
        F: Fn(&T, ListWidgetItemType) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_list_widget_obj_funcs();

        let f: Box<Box<Fn(&T, ListWidgetItemType) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_item_pressed_event)(
                obj_data,
                user_data,
                transmute(Self::item_pressed_trampoline_ud::<T> as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }

    unsafe extern "C" fn item_pressed_trampoline(
        user_data: *const c_void,
        func: *const c_void,
        item: ListWidgetItemType
    ) {
        let f: &&(Fn(ListWidgetItemType) + 'static) = transmute(func);
        f(item);
    }

    pub fn set_item_pressed_event<F>(&self, func: F) -> &ListWidget<'a>
    where
        F: Fn(ListWidgetItemType) + 'a,
    {
        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
        let f: Box<Box<Fn(ListWidgetItemType) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_item_pressed_event)(
                obj_data,
                ::std::ptr::null(),
                transmute(Self::item_pressed_trampoline as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }
}
pub trait ListWidgetType {

    pub fn add_item(&self, label: &str) -> &Self {
        let str_in_label_1 = CString::new(label).unwrap();

        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
        unsafe {
            ((*funcs).add_item)(obj_data, str_in_label_1.as_ptr());
        }
        self
    }

    pub fn clear(&self) -> &Self {

        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
        unsafe {
            ((*funcs).clear)(obj_data);
        }
        self
    }

    fn get_list_widget_obj_funcs(&self) -> (*const RUBase, *const RUListWidgetFuncs);
}

impl<'a> WidgetType for ListWidget<'a> {
    fn get_widget_obj_funcs(&self) -> (*const RUBase, *const RUWidgetFuncs) {
        let obj = self.data.get().unwrap();
        (obj, self.all_funcs.widget_funcs)
    }
}

impl<'a> ListWidgetType for ListWidget<'a> {
    fn get_list_widget_obj_funcs(&self) -> (*const RUBase, *const RUListWidgetFuncs) {
        let obj = self.data.get().unwrap();
        (obj, self.all_funcs.list_widget_funcs)
    }
}
