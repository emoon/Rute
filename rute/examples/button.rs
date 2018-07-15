extern crate rute;

use std::cell::Cell;
use std::mem::transmute;
use std::os::raw::c_void;

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
        user_data: *mut c_void,
        callback: unsafe extern "C" fn(),
        wrapped_func: *const c_void,
    );
}

#[derive(Clone)]
struct Widget {
    data: Box<Cell<Option<RUWidget>>>,
}

#[derive(Clone)]
struct Slider {
    data: Box<Cell<Option<RUWidget>>>,
}

impl Widget {
    pub fn new() -> Widget {
        unsafe {
            let ffi_data = create_widget();
            Widget {
                data: Box::new(Cell::new(Some(ffi_data))),
            }
        }
    }

    pub fn show(self) -> Self {
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

/*
   fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

unsafe extern "C" fn changed_trampoline<P>(this: *mut ffi::GtkComboBox, f: glib_ffi::gpointer) where P: IsA<ComboBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}
*/

impl Slider {
    pub fn new() -> Slider {
        unsafe {
            let ffi_data = create_slider();
            Slider {
                data: Box::new(Cell::new(Some(ffi_data))),
            }
        }
    }

    pub fn show(self) -> Self {
        let data = self.data.get().unwrap().privd;
        unsafe {
            slider_show(data);
        }
        self
    }

    pub fn value_changed<F, T>(self, data: &mut T, func: F) -> Slider
    where
        F: Fn(&mut T, i32) + 'static,
    {
        unsafe {
            let widget_data = self.data.get().unwrap().privd;
            let f: Box<Box<Fn(&mut T, i32) + 'static>> = Box::new(Box::new(func));
            let user_data: *mut c_void = transmute(data);
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

fn main() {
    unsafe {
        create_application();
    }

    let mut temp = 12u32;

    Slider::new()
        .value_changed(&mut temp, |temp, value| {
            println!("value {} - {}", temp, value);
        })
        .show();

    //Slider::new().value_changed(&mut temp, callbacked).show();

    unsafe {
        run_application();
    }
}
