
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

