
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

pub trait ListWidgetItemType {

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
                ret_val = ListWidget {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<*const RUBase>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = ListWidget {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            Some(ret_val)
        }
    }

    pub fn set_selected(&self, select: bool) -> &Self {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_selected)(obj_data, select);
        }
        self
    }

    pub fn is_selected(&self) -> bool {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_selected)(obj_data);
            ret_val
        }
    }

    pub fn set_hidden(&self, hide: bool) -> &Self {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_hidden)(obj_data, hide);
        }
        self
    }

    pub fn is_hidden(&self) -> bool {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_hidden)(obj_data);
            ret_val
        }
    }

    pub fn flags(&self) -> ItemFlags {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).flags)(obj_data);
           let ret_val = unsafe { transmute::<i32, Rute>(ret_val) };
            ret_val
        }
    }

    pub fn set_flags(&self, flags: ItemFlags) -> &Self {
        let enum_flags_1 = flags as i32;

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_flags)(obj_data, enum_flags_1);
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

    pub fn set_text(&self, text: &str) -> &Self {
        let str_in_text_1 = CString::new(text).unwrap();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_text)(obj_data, str_in_text_1.as_ptr());
        }
        self
    }

    pub fn icon(&self) -> Icon {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).icon)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Icon {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<*const RUBase>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = Icon {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            ret_val
        }
    }

    pub fn status_tip(&self) -> &str {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).status_tip)(obj_data);
           let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }

    pub fn set_status_tip(&self, status_tip: &str) -> &Self {
        let str_in_status_tip_1 = CString::new(status_tip).unwrap();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_status_tip)(obj_data, str_in_status_tip_1.as_ptr());
        }
        self
    }

    pub fn tool_tip(&self) -> &str {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).tool_tip)(obj_data);
           let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }

    pub fn set_tool_tip(&self, tool_tip: &str) -> &Self {
        let str_in_tool_tip_1 = CString::new(tool_tip).unwrap();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_tool_tip)(obj_data, str_in_tool_tip_1.as_ptr());
        }
        self
    }

    pub fn whats_this(&self) -> &str {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).whats_this)(obj_data);
           let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }

    pub fn set_whats_this(&self, whats_this: &str) -> &Self {
        let str_in_whats_this_1 = CString::new(whats_this).unwrap();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_whats_this)(obj_data, str_in_whats_this_1.as_ptr());
        }
        self
    }

    pub fn font(&self) -> Font {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).font)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Font {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<*const RUBase>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = Font {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            ret_val
        }
    }

    pub fn set_font(&self, font: &FontType) -> &Self {
        let (obj_font_1, _funcs) = font.get_font_obj_funcs();

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_font)(obj_data, obj_font_1);
        }
        self
    }

    pub fn text_alignment(&self) -> i32 {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).text_alignment)(obj_data);
            ret_val
        }
    }

    pub fn set_text_alignment(&self, alignment: i32) -> &Self {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_text_alignment)(obj_data, alignment);
        }
        self
    }

    pub fn check_state(&self) -> CheckState {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).check_state)(obj_data);
           let ret_val = unsafe { transmute::<i32, Rute>(ret_val) };
            ret_val
        }
    }

    pub fn set_check_state(&self, state: CheckState) -> &Self {
        let enum_state_1 = state as i32;

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            ((*funcs).set_check_state)(obj_data, enum_state_1);
        }
        self
    }

    pub fn size_hint(&self) -> Size {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).size_hint)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Size {
                    data: Rc::from_raw(t.host_data as *const Cell<Option<*const RUBase>>),
                    _marker: PhantomData,
                };
            } else {
                ret_val = Size {
                    data: Rc::new(Cell::new(Some(t))),
                    _marker: PhantomData,
                };
            }
            ret_val
        }
    }

    pub fn get_type(&self) -> i32 {

        let (obj_data, funcs) = self.get_list_widget_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).get_type)(obj_data);
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
