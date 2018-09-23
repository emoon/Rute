
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

#[allow(unused_imports)]
use std::ffi::{CString, CStr};

use rute_ffi_base::*;



unsafe extern "C" fn rute_object_delete_callback(data: *const c_void) {
    let d = Rc::from_raw(data as *const Cell<Option<RUBase>>);
    d.set(None);
}

pub struct Rute<'a> {
    rute_ffi: *const RuteFFI,
    priv_data: *const c_void,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Rute<'a> {
    pub fn new() -> Rute<'a> {
        Rute {
            rute_ffi: unsafe { rute_get() },
            priv_data: ::std::ptr::null(),
            _marker: PhantomData,
        }
    }

    pub fn create_application(&self) -> Application<'a> {
        let ffi_data = unsafe { ((*self.rute_ffi).create_application)(::std::ptr::null()) };
        Application {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data))),
            all_funcs: ffi_data.all_funcs,
            _marker: PhantomData,
        }
    }

    pub fn application(&self) -> ApplicationStatic<'a> {
        let ffi_data = unsafe { ((*self.rute_ffi).get_application)(::std::ptr::null()) };
        ApplicationStatic {
            data: ffi_data.qt_data,
            all_funcs: ffi_data.all_funcs,
            _marker: PhantomData,
        }
    }

    pub fn create_font(&self) -> Font<'a> {
        let ffi_data = unsafe { ((*self.rute_ffi).create_font)(::std::ptr::null()) };
        Font {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data))),
            all_funcs: ffi_data.all_funcs,
            _marker: PhantomData,
        }
    }

    pub fn create_list_widget(&self) -> ListWidget<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*self.rute_ffi).create_list_widget)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback::<RUListWidget> as usize),
                Rc::into_raw(data.clone()) as *const c_void)
        };

        data.set(Some(ffi_data.qt_data));

        ListWidget {
            data,
            all_funcs: ffi_data.all_funcs,
            _marker: PhantomData,
        }
    }

    pub fn create_list_widget_item(&self) -> ListWidgetItem<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*self.rute_ffi).create_list_widget_item)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback::<RUListWidgetItem> as usize),
                Rc::into_raw(data.clone()) as *const c_void)
        };

        data.set(Some(ffi_data.qt_data));

        ListWidgetItem {
            data,
            all_funcs: ffi_data.all_funcs,
            _marker: PhantomData,
        }
    }

    pub fn create_widget(&self) -> Widget<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*self.rute_ffi).create_widget)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback::<RUWidget> as usize),
                Rc::into_raw(data.clone()) as *const c_void)
        };

        data.set(Some(ffi_data.qt_data));

        Widget {
            data,
            all_funcs: ffi_data.all_funcs,
            _marker: PhantomData,
        }
    }
}