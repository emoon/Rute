extern crate rute;

use std::cell::Cell;
use std::marker::PhantomData;
use std::mem::transmute;
use std::os::raw::{c_void, c_char};
use std::cell::RefCell;
use std::rc::Rc;
use std::ffi::CString;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                                FFI SECTION
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct RUBase {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct RUWidgetFuncs {
    pub show: extern "C" fn(self_c: *const RUBase),
    pub set_parent: extern "C" fn(self_c: *const RUBase, parent: *const RUBase),
    pub set_size: extern "C" fn(self_c: *const RUBase, width: i32, height: i32),
}

#[repr(C)]
pub struct RUListWidgetFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub add_item: extern "C" fn(self_c: *const RUBase, in_0: *const RUListWidgetItem),
    pub clear: extern "C" fn(self_c: *const RUBase),
    pub selected_items: extern "C" fn(self_c: *const RUBase) -> RUArray,
    pub set_current_row: extern "C" fn(self_c: *const RUBase, value: i32),
}

#[repr(C)]
pub struct RUListWidgetItemFuncs {
    pub set_text: extern "C" fn(self_c: *const RUBase, text: *const c_char),
    pub icon: extern "C" fn(self_c: *const RUBase) -> RUIcon,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUWidget {
    pub privd: *const RUBase,
    pub widget_funcs: *const RUWidgetFuncs,
}

#[repr(C)]
pub struct RUIconFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub cache_key: extern "C" fn(self_c: *const RUBase) -> u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUIcon {
    pub privd: *const RUBase,
    pub icon_funcs: *const RUIconFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUListWidget {
    pub privd: *const RUBase,
    pub widget_funcs: *const RUWidgetFuncs,
    pub list_widget_funcs: *const RUListWidgetFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUListWidgetItem {
    pub privd: *const RUBase,
    pub list_widget_item_funcs: *const RUListWidgetItemFuncs,
}

#[repr(C)]
pub struct RUApplicationFuncs {
    pub exec: extern "C" fn(self_c: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUApplication {
    pub privd: *const RUBase,
    pub application_funcs: *const RUApplicationFuncs,
}

#[repr(C)]
pub struct RuteFFI {
    pub create_application: extern "C" fn(priv_data: *const RUBase) -> RUApplication,
    pub create_widget: extern "C" fn(priv_data: *const RUBase) -> RUWidget,
    pub create_list_widget: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        delete_data: *const c_void) -> RUListWidget,
    pub create_list_widget_item: extern "C" fn(
        priv_data: *const RUBase,
        callback: unsafe extern "C" fn(),
        delete_data: *const c_void) -> RUListWidgetItem,
}

extern "C" {
    fn rute_get() -> *const RuteFFI;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                               Rust Implementation
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Widget<'a> {
    data: Rc<Cell<Option<RUWidget>>>,
    _marker: PhantomData<std::cell::Cell<&'a ()>>,
}

#[derive(Clone)]
pub struct ListWidget<'a> {
    data: Rc<Cell<Option<RUListWidget>>>,
    _marker: PhantomData<std::cell::Cell<&'a ()>>,
}

#[derive(Clone)]
pub struct ListWidgetItem<'a> {
    data: Rc<Cell<Option<RUListWidgetItem>>>,
    _marker: PhantomData<std::cell::Cell<&'a ()>>,
}

#[derive(Clone)]
pub struct Slider<'a> {
    data: Rc<Cell<Option<RUWidget>>>,
    _marker: PhantomData<std::cell::Cell<&'a ()>>,
}

#[derive(Clone)]
pub struct Icon {
    data: Rc<Cell<Option<RUIcon>>>,
    owner: bool,
}

impl Drop for Icon {
    fn drop(&mut self) {
        println!("Icon: About to drop");
        if self.owner {
            println!("Icon: Dropping as owner");
            let obj = self.data.get().unwrap();
            unsafe {
                ((*obj.icon_funcs).destroy)(obj.privd);
            }

            self.data.set(None);
        }
    }
}

impl Icon {
    pub fn destroy(&self) {
        if self.owner {
            let obj = self.data.get().unwrap();
            unsafe {
                ((*obj.icon_funcs).destroy)(obj.privd);
            }
            self.data.set(None);
        } else {
            println!("Icon: Trying to destroy data not being owner of");
        }
    }

    pub fn cache_key(&self) -> u64 {
        let obj = self.data.get().unwrap();
        unsafe {
            ((*obj.icon_funcs).cache_key)(obj.privd)
        }
    }
}


#[derive(Clone)]
pub struct Application<'a> {
    data: Rc<Cell<Option<RUApplication>>>,
    _marker: PhantomData<std::cell::Cell<&'a ()>>,
}

struct Rute<'a> {
    rute_ffi: *const RuteFFI,
    _marker: PhantomData<std::cell::Cell<&'a ()>>,
}

pub trait WidgetType {
    fn get_widget_type_obj(&self) -> *const RUBase;
}

impl<'a> WidgetType for Widget<'a> {
    fn get_widget_type_obj(&self) -> *const RUBase {
        let obj = self.data.get().unwrap();
        obj.privd
    }
}

impl<'a> WidgetType for ListWidget<'a> {
    fn get_widget_type_obj(&self) -> *const RUBase {
        let obj = self.data.get().unwrap();
        obj.privd
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct WrapperRcOwn(*const c_void);

#[repr(C)]
pub struct RUArray {
    pub elements: *const WrapperRcOwn,
    pub count: i32,
}

pub struct RefArray<'a, T, F> {
    array: RUArray,
    index: isize,
    owner: bool,
    _temp_0: PhantomData<F>,
    _temp_1: PhantomData<T>,
    _dummy: PhantomData<&'a u32>,
}

impl<'a> From<WrapperRcOwn> for ListWidgetItem<'a> {
    fn from(t: WrapperRcOwn) -> Self {
        ListWidgetItem {
            data: unsafe { Rc::from_raw(t.0 as *const Cell<Option<RUListWidgetItem>>) },
            _marker: PhantomData,
        }
    }
}

impl<'a, T, F> Iterator for RefArray<'a, T, F> 
where
    T: std::convert::From<WrapperRcOwn>,
    F: Clone,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        if index >= self.array.count as isize {
            None
        } else {
            self.index += 1;
            unsafe {
                let data = self.array.elements as *const WrapperRcOwn;
                let t = &*data.offset(index);
                Some(t.clone().into())
            }
        }
    }
}

unsafe extern "C" fn rute_object_delete_callback<T>(data: *const c_void) {
    let d = Rc::from_raw(data as *const Cell<Option<T>>);
    d.set(None);
}

impl<'a> Rute<'a> {
    pub fn new() -> Rute<'a> {
        Rute {
            rute_ffi: unsafe { rute_get() },
            _marker: PhantomData,
        }
    }

    pub fn create_widget(&self) -> Widget<'a> {
        let ffi_data = unsafe { ((*self.rute_ffi).create_widget)(std::ptr::null()) };
        Widget {
            data: Rc::new(Cell::new(Some(ffi_data))),
            _marker: PhantomData,
        }
    }

    pub fn create_list_widget(&self) -> ListWidget<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*self.rute_ffi).create_list_widget)(
                std::ptr::null(),
                transmute(rute_object_delete_callback::<RUListWidget> as usize),
                Rc::into_raw(data.clone()) as *const c_void)
        };

        data.set(Some(ffi_data));

        ListWidget {
            data,
            _marker: PhantomData,
        }
    }

    pub fn create_list_widget_item(&self) -> ListWidgetItem<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*self.rute_ffi).create_list_widget_item)(
                std::ptr::null(),
                transmute(rute_object_delete_callback::<RUListWidgetItem> as usize),
                Rc::into_raw(data.clone()) as *const c_void)
        };

        data.set(Some(ffi_data));

        ListWidgetItem {
            data,
            _marker: PhantomData,
        }
    }

    pub fn create_application(&self) -> Application<'a> {
        let ffi_data = unsafe { ((*self.rute_ffi).create_application)(std::ptr::null()) };
        Application {
            data: Rc::new(Cell::new(Some(ffi_data))),
            _marker: PhantomData,
        }
    }
}

impl<'a> Widget<'a> {
    pub fn show(self) -> Widget<'a> {
        let obj = self.data.get().unwrap();
        unsafe {
            ((*obj.widget_funcs).show)(obj.privd);
        }
        self
    }

    pub fn set_size(self, width: i32, height: i32) -> Widget<'a> {
        let obj = self.data.get().unwrap();
        unsafe {
            ((*obj.widget_funcs).set_size)(obj.privd, width, height);
        }
        self
    }

    pub fn set_parent(&self, parent: &WidgetType) {
        let parent_obj = parent.get_widget_type_obj();
        let obj = self.data.get().unwrap();
        unsafe {
            ((*obj.widget_funcs).set_parent)(obj.privd, parent_obj);
        }
    }
}

impl<'a> ListWidget<'a> {
    pub fn show(&self) -> &ListWidget<'a> {
        let obj = self.data.get().unwrap();
        unsafe {
            ((*obj.widget_funcs).show)(obj.privd);
        }
        self
    }

    pub fn clear(&self) -> &ListWidget<'a> {
        let obj = self.data.get().unwrap();
        unsafe {
            ((*obj.list_widget_funcs).clear)(obj.privd);
        }
        self
    }

    pub fn set_current_row(&self, value: i32) -> &ListWidget<'a> {
        let obj = self.data.get().unwrap();
        unsafe {
            ((*obj.list_widget_funcs).set_current_row)(obj.privd, value);
        }
        self
    }

    pub fn selected_items(&self) -> RefArray<ListWidgetItem, WrapperRcOwn> {
        let obj = self.data.get().unwrap();
        let raw_array = unsafe {
            ((*obj.list_widget_funcs).selected_items)(obj.privd)
        };

        RefArray {
            array: raw_array,
            index: 0,
            owner: false,
            _dummy: PhantomData,
            _temp_0: PhantomData,
            _temp_1: PhantomData,
        }
    }

    pub fn add_item(&self, item: &ListWidgetItem) -> &ListWidget<'a> {
        let obj = self.data.get().unwrap();
        let in_0 = item.data.get().unwrap();

        unsafe {
            ((*obj.list_widget_funcs).add_item)(obj.privd, &in_0);
        }
        self
    }

    pub fn destroy(&self) {
        let obj = self.data.get().unwrap();
        unsafe {
            ((*obj.list_widget_funcs).destroy)(obj.privd);
        }
    }

    pub fn build(&self) -> ListWidget<'a> {
        self.clone()
    }

    pub fn set_parent(&self, parent: &WidgetType) {
        let parent_obj = parent.get_widget_type_obj();
        let obj = self.data.get().unwrap();
        unsafe {
            ((*obj.widget_funcs).set_parent)(obj.privd, parent_obj);
        }
    }
}

impl<'a> ListWidgetItem<'a> {
    pub fn set_text(&self, text: &str) -> &ListWidgetItem<'a> {
        let obj = self.data.get().unwrap();
        let str_in_text_1 = CString::new(text).unwrap();

        unsafe {
            ((*obj.list_widget_item_funcs).set_text)(obj.privd, str_in_text_1.as_ptr());
        }
        self
    }

    pub fn build(&self) -> ListWidgetItem<'a> {
        self.clone()
    }

    pub fn icon(&self) -> Icon {
        let obj = self.data.get().unwrap();

        let icon_ffi = unsafe {
            ((*obj.list_widget_item_funcs).icon)(obj.privd)
        };

        Icon {
            data: Rc::new(Cell::new(Some(icon_ffi))),
            owner: true,
        }
    }
}

impl<'a> Application<'a> {
    pub fn exec(self) -> Application<'a> {
        let obj = self.data.get().unwrap();
        unsafe {
            ((*obj.application_funcs).exec)(obj.privd);
        }
        self
    }
}

/*
unsafe extern "C" fn slider_value_changed_trampoline<T>(
    user_data: *const c_void,
    func: *const c_void,
    value: i32,
) {
    let f: &&(Fn(&mut T, i32) + 'static) = transmute(func);
    let data = user_data as *mut T;
    f(&mut *data, value);
}

impl<'a> Slider<'a> {
    pub fn show(self) -> Slider<'a> {
        let data = self.data.get().unwrap().privd;
        unsafe {
            slider_show(data);
        }
        self
    }

    pub fn value_changed<F, T>(self, data: &'a T, func: F) -> Slider<'a>
    where
        F: Fn(&T, i32) + 'a,
        T: 'a,
    {
        let widget_data = self.data.get().unwrap().privd;
        let f: Box<Box<Fn(&T, i32) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            slider_connect_value_changed(
                widget_data,
                user_data,
                transmute(slider_value_changed_trampoline::<T> as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }
}
*/

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                  Application testing
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct UiState {
    value: i32,
}

struct MyApp<'a> {
    ui: Rute<'a>,
    icon: Option<Icon>,
    shared_state: RefCell<UiState>,
}

impl<'a> MyApp<'a> {
    fn new() -> MyApp<'a> {
        MyApp {
            ui: Rute::new(),
            icon: None,
            shared_state: RefCell::new( UiState { value: 0 }),
        }
    }

    fn setup_ui(&'a mut self) {
        let widget = self.ui.create_widget().set_size(400, 400);
        let list = self.ui.create_list_widget().show().build();

        let item = self.ui.create_list_widget_item().set_text("Test").build();

        list.set_parent(&widget);
        list.add_item(&item);
        list.set_current_row(0);

        let icon = item.icon();
        println!("hash {}", icon.cache_key());

        widget.show();

        self.icon = Some(icon);

        //list.clear();

        //item.set_text("Test 2");

        //list.destroy();


        /*
        self.ui.create_slider().value_changed(self, |state, value| {
            let mut state = state.shared_state.borrow_mut();
            println!("prev value {}", state.value);
            state.value = value;
            println!("value {}", value);
        }).show();
        */
    }
}

fn main() {
    let mut app = MyApp::new();

    // Create the main Qt application
    let ui_app = app.ui.create_application();

    app.setup_ui();

    // run the application
    ui_app.exec();
}
