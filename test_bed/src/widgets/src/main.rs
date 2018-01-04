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
}

impl MyApp {
    fn new(ui: Ui) -> MyApp {
        MyApp {
            ui: ui,
        }
    }

    fn new_row_selected(&mut self, row: i32) {
        println!("new row {}", row);
    }

    fn menu_selected(&mut self) {
        println!("menu select");
    }

    fn run(&mut self) {
        let app = self.ui.create_application();
        let main_window = self.ui.create_main_window();
        let list = self.ui.create_list_widget();
        let button = self.ui.create_push_button();

        button.set_text("Button!");
        list.add_item("New Text!");
        list.add_item("Test 4");

        list.set_drag_enabled(true);
        list.set_accept_drops(true);
        list.set_drop_indicator_shown(true);

        main_window.resize(500, 500);
        //main_window.show();

        let open_file = self.ui.create_action();
        open_file.set_text("Open");

        let file_menu = self.ui.create_menu();
        file_menu.set_title("File");
        file_menu.add_action(&open_file);

        let menu_bar = main_window.menu_bar();
        menu_bar.add_menu(&file_menu);

        set_current_row_changed_event!(list, &self, MyApp, MyApp::new_row_selected);
        set_triggered_event!(open_file, &self, MyApp, MyApp::menu_selected);

        let layout = self.ui.create_v_box_layout();

        layout.add_widget(&button);
        layout.add_widget(&list);

        let window = self.ui.create_widget();
        window.set_layout(&layout);

        main_window.set_central_widget(&window);
        main_window.show();

        app.exec();
    }
}


fn main() {
    let wrui_instance = SharedLibUi::new(get_wrui_path()).unwrap();
    let mut app = MyApp::new(wrui_instance.get_ui());
    app.run();
}
