
use std::cell::Cell;
use std::marker::PhantomData;
use std::mem::transmute;
use std::os::raw::{c_void, c_char};
use std::cell::RefCell;
use std::rc::Rc;
use std::ffi::CString;
use auto::rute_auto_ffi::*;



#[derive(Clone)]
pub struct Application<'a> {
    data: Rc<Cell<Option<RUApplication>>>,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Application<'a> {
    pub fn set_style(&self, style: &str) -> &Application<'a> {
        let str_in_style_1 = CString::new(style).unwrap();

        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            ((*funcs).set_style)(obj_data, str_in_style_1.as_ptr());
        }
        self
    
    }

    pub fn exec(&self) -> i32 {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).exec)(obj_data);
        
            ret_val
          
        }
    
    }
    fn get_application_obj_funcs(&self) -> (*const RUBase, *const RUApplicationFuncs) {
        let obj = self.data.get().unwrap();
        (obj.privd, obj.application_funcs)
    }
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
        let ffi_data = unsafe { ((*self.rute_ffi).create_application)(::std::ptr::null(), ::std::ptr::null()) };
        Application {
            data: Rc::new(Cell::new(Some(ffi_data))),
            _marker: PhantomData,
        }
    }
}
