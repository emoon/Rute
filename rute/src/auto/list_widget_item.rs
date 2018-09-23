
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

#[allow(unused_imports)]
use std::ffi::{CString, CStr};

use rute_ffi_base::*;


#[allow(unused_imports)]use auto::*;
use auto::list_widget_item_ffi::*;

#[derive(Clone)]
pub struct ListWidgetItem<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUListWidgetItemAllFuncs,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

pub trait ListWidgetItemType {

    fn list_widget(&self) -> Option<ListWidget> {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).list_widget)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = ListWidget {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<*const RUBase>>),
                    all_funcs: t.all_funcs,
                    _marker: PhantomData,
                };
            } else {
                ret_val = ListWidget {
                    data: Rc::new(Cell::new(Some(t.qt_data as *const RUBase))),
                    all_funcs: t.all_funcs,
                    _marker: PhantomData,
                };
            }
            Some(ret_val)
        }
    }

    fn set_selected(&self, select: bool) -> &Self {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_selected)(obj_data, select);
        }
        self
    }

    fn is_selected(&self) -> bool {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_selected)(obj_data);
            ret_val
        }
    }

    fn set_hidden(&self, hide: bool) -> &Self {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_hidden)(obj_data, hide);
        }
        self
    }

    fn is_hidden(&self) -> bool {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_hidden)(obj_data);
            ret_val
        }
    }

    fn text(&self) -> String {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).text)(obj_data);
           let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }

    fn set_text(&self, text: &str) -> &Self {
        let str_in_text_1 = CString::new(text).unwrap();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_text)(obj_data, str_in_text_1.as_ptr());
        }
        self
    }

    fn status_tip(&self) -> String {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).status_tip)(obj_data);
           let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }

    fn set_status_tip(&self, status_tip: &str) -> &Self {
        let str_in_status_tip_1 = CString::new(status_tip).unwrap();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_status_tip)(obj_data, str_in_status_tip_1.as_ptr());
        }
        self
    }

    fn tool_tip(&self) -> String {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).tool_tip)(obj_data);
           let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }

    fn set_tool_tip(&self, tool_tip: &str) -> &Self {
        let str_in_tool_tip_1 = CString::new(tool_tip).unwrap();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_tool_tip)(obj_data, str_in_tool_tip_1.as_ptr());
        }
        self
    }

    fn whats_this(&self) -> String {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).whats_this)(obj_data);
           let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }

    fn set_whats_this(&self, whats_this: &str) -> &Self {
        let str_in_whats_this_1 = CString::new(whats_this).unwrap();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_whats_this)(obj_data, str_in_whats_this_1.as_ptr());
        }
        self
    }

    fn text_alignment(&self) -> i32 {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).text_alignment)(obj_data);
            ret_val
        }
    }

    fn set_text_alignment(&self, alignment: i32) -> &Self {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_text_alignment)(obj_data, alignment);
        }
        self
    }

    fn get_list_widget_item_obj_funcs(&self) -> (*const RUBase, *const RUListWidgetItemFuncs);
}

impl<'a> ListWidgetItemType for ListWidgetItem<'a> {
    fn get_list_widget_item_obj_funcs(&self) -> (*const RUBase, *const RUListWidgetItemFuncs) {
        let obj = self.data.get().unwrap();
        unsafe {
            (obj, (*self.all_funcs).list_widget_item_funcs)
        }
    }
}
