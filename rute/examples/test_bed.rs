extern crate rute;

use std::cell::Cell;
use std::marker::PhantomData;
//use std::mem::transmute;
//use std::os::raw::c_void;
use std::cell::RefCell;

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
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUWidget {
    pub privd: *const RUBase,
    pub widget_funcs: *const RUWidgetFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUListWidget {
    pub privd: *const RUBase,
    pub widget_funcs: *const RUWidgetFuncs,
    pub list_widget_funcs: *const RUWidgetFuncs,
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
    pub create_list_widget: extern "C" fn(priv_data: *const RUBase) -> RUListWidget,
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
    data: Box<Cell<Option<RUWidget>>>,
    _marker: PhantomData<std::cell::Cell<&'a ()>>,
}

#[derive(Clone)]
pub struct ListWidget<'a> {
    data: Box<Cell<Option<RUListWidget>>>,
    _marker: PhantomData<std::cell::Cell<&'a ()>>,
}

#[derive(Clone)]
pub struct Slider<'a> {
    data: Box<Cell<Option<RUWidget>>>,
    _marker: PhantomData<std::cell::Cell<&'a ()>>,
}

#[derive(Clone)]
pub struct Application<'a> {
    data: Box<Cell<Option<RUApplication>>>,
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
            data: Box::new(Cell::new(Some(ffi_data))),
            _marker: PhantomData,
        }
    }

    pub fn create_list_widget(&self) -> ListWidget<'a> {
        let ffi_data = unsafe { ((*self.rute_ffi).create_list_widget)(std::ptr::null()) };
        ListWidget {
            data: Box::new(Cell::new(Some(ffi_data))),
            _marker: PhantomData,
        }
    }

    pub fn create_application(&self) -> Application<'a> {
        let ffi_data = unsafe { ((*self.rute_ffi).create_application)(std::ptr::null()) };
        Application {
            data: Box::new(Cell::new(Some(ffi_data))),
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

    pub fn set_parent(self, parent: &WidgetType) -> Widget<'a> {
        let parent_obj = parent.get_widget_type_obj();
        let obj = self.data.get().unwrap();
        unsafe {
            ((*obj.widget_funcs).set_parent)(obj.privd, parent_obj);
        }
        self
    }
}

impl<'a> ListWidget<'a> {
    pub fn show(self) -> ListWidget<'a> {
        let obj = self.data.get().unwrap();
        unsafe {
            ((*obj.widget_funcs).show)(obj.privd);
        }
        self
    }

    pub fn set_parent(self, parent: &WidgetType) -> ListWidget<'a> {
        let parent_obj = parent.get_widget_type_obj();
        let obj = self.data.get().unwrap();
        unsafe {
            ((*obj.widget_funcs).set_parent)(obj.privd, parent_obj);
        }
        self
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
    shared_state: RefCell<UiState>,
}

impl<'a> MyApp<'a> {
    fn new() -> MyApp<'a> {
        MyApp {
            ui: Rute::new(),
            shared_state: RefCell::new(UiState { value: 0 }),
        }
    }

    fn setup_ui(&'a mut self) {
        let widget = self.ui.create_widget();
        let list = self.ui.create_list_widget();

        list.set_parent(&widget);

        widget.show();

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
