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

    fn new_row_selected(&mut self, row: i32) {
        println!("new row {}", row);
    }

    fn run(&mut self) {
        let app = self.ui.create_application();
        let main_window = self.ui.create_main_window();
        let list = self.ui.create_list_widget();

        list.add_item("Test");
        list.add_item("Test 4");

        let test_item = list.item(0);
        test_item.set_text("New text!");

        //button.set_text("Press me!");
        //button.resize(100, 100);
        //button.show();

        main_window.resize(500, 500);
        main_window.set_central_widget(&list);
        main_window.show();

        set_current_row_changed_event!(list, self, MyApp, MyApp::new_row_selected);

        app.exec();
    }
}


fn main() {
    let wrui_instance = SharedLibUi::new(get_wrui_path()).unwrap();
    let mut app = MyApp::new(wrui_instance.get_ui());
    app.run();
}
