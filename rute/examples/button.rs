extern crate rute;

use std::cell::Cell;
use std::marker::PhantomData;
use std::mem::transmute;
use std::os::raw::c_void;
use std::cell::RefCell;

#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct RUBase {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RUWidget {
    pub privd: *const RUBase,
}

extern "C" {
    fn dummy();
    fn create_application();
    fn run_application();

    fn create_widget() -> RUWidget;
    fn widget_show(widget: *const RUBase);

    fn create_slider() -> RUWidget;
    fn slider_show(widget: *const RUBase);

    fn slider_connect_value_changed(
        widget: *const RUBase,
        user_data: *const c_void,
        callback: unsafe extern "C" fn(),
        wrapped_func: *const c_void,
    );
}

#[derive(Clone)]
struct Widget<'a> {
    data: Box<Cell<Option<RUWidget>>>,
    _marker: PhantomData<std::cell::Cell<&'a ()>>,
}

#[derive(Clone)]
struct Slider<'a> {
    data: Box<Cell<Option<RUWidget>>>,
    _marker: PhantomData<std::cell::Cell<&'a ()>>,
}

struct Rute<'a> {
    _marker: PhantomData<std::cell::Cell<&'a ()>>,
}

/*
impl<'a> Drop for Rute<'a> {
    fn drop(&mut self) {}
}
*/

impl<'a> Rute<'a> {
    pub fn create_widget(&self) -> Widget<'a> {
        let ffi_data = unsafe { create_widget() };
        Widget {
            data: Box::new(Cell::new(Some(ffi_data))),
            _marker: PhantomData,
        }
    }

    pub fn create_slider(&self) -> Slider<'a> {
        let ffi_data = unsafe { create_slider() };
        Slider {
            data: Box::new(Cell::new(Some(ffi_data))),
            _marker: PhantomData,
        }
    }
}

impl<'a> Widget<'a> {
    pub fn show(self) -> Widget<'a> {
        let data = self.data.get().unwrap().privd;
        unsafe {
            widget_show(data);
        }
        self
    }
}

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

fn callbacked(temp: &mut u32, value: i32) {
    println!("value {} - {}", temp, value);
}

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
            ui: Rute { _marker: PhantomData },
            shared_state: RefCell::new(UiState { value: 0 }),
        }
    }

    fn setup_ui(&'a mut self) {
        self.ui.create_slider().value_changed(self, |state, value| {
            let mut state = state.shared_state.borrow_mut();
            println!("prev value {}", state.value);
            state.value = value;
            println!("value {}", value);
        }).show();
    }
}

fn main() {
    unsafe {
        create_application();
    }

    let mut app = MyApp::new();
    app.setup_ui();


    /*
    rute.create_slider().value_changed(&temp, |temp, value| {
        println!("value {} - {}", temp, value);
    }).show();
    */

    //Slider::new().value_changed(&mut temp, callbacked).show();

    unsafe {
        run_application();
    }
}
