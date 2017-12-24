#[macro_use]
extern crate wrui;

use wrui::{SharedLibUi, Ui};

#[cfg(target_os="windows")]
fn get_wrui_path() -> &'static str {
    "t2-output/win64-msvc-debug-default/wrui_qt.dll"
}

#[cfg(target_os="macos")]
fn get_wrui_path() -> &'static str {
    "t2-output/macosx-clang-debug-default/libwrui_qt.dylib"
}

struct MyApp {
    ui: Ui,
    pressed_count: usize,
}

impl MyApp {
    fn new(ui: Ui) -> MyApp {
        MyApp {
            ui: ui,
            pressed_count: 0,
        }
    }

    fn pressed_button(&mut self) {
        self.pressed_count += 1;
        println!("Pressed button {} times", self.pressed_count);
    }

    fn run(&mut self) {
        let app = self.ui.create_application();
        let button = self.ui.create_push_button();

        button.set_text("Press me!");
        button.show();

        set_released_event!(button, self, MyApp, MyApp::pressed_button);

        app.exec();
    }
}


fn main() {
    let wrui_instance = SharedLibUi::new(get_wrui_path()).unwrap();
    let mut app = MyApp::new(wrui_instance.get_ui());
    app.run();
}
