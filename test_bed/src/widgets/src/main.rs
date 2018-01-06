#[macro_use]
extern crate wrui;

use wrui::{SharedLibUi, Ui};
use wrui::wrui::{ListWidget, Application};

#[cfg(target_os="windows")]
fn get_wrui_path() -> &'static str {
    "t2-output/win64-msvc-debug-default/wrui_qt.dll"
}

#[cfg(target_os="macos")]
fn get_wrui_path() -> &'static str {
    "t2-output/macosx-clang-debug-default/libwrui_qt.dylib"
}

#[repr(C)]
struct MyApp<'a> {
    ui: &'a Ui,
    list: ListWidget,
    app: Application,
}

impl<'a> MyApp<'a> {
    fn new(ui: &'a Ui) -> MyApp {
        MyApp {
            ui: ui,
            app: ui.create_application(),
            list: ui.create_list_widget(),
        }
    }

    fn new_row_selected(&mut self, row: i32) {
        println!("callback: self {:p}", &self);
        println!("callback: obj.privd {:p}", self.list.obj.as_ref().unwrap().privd);
        println!("callback: self.list {:p}", &self.list);

        println!("new row {}", row);
    }

    fn menu_selected(&mut self) {
        println!("menu_selected: obj.privd {:p}", self.list.obj.as_ref().unwrap().privd);

        for selected in &self.list.items() {
            selected.set_text("selected");
        }

        println!("menu select");
    }

    fn run(&self) {
        let main_window = self.ui.create_main_window();
        let button = self.ui.create_push_button();

        println!("obj.privd {:p}", self.list.obj.as_ref().unwrap().privd);

        button.set_text("Button!");
        self.list.add_item("New Text!");
        self.list.add_item("Test 4");

        self.list.set_drag_enabled(true);
        self.list.set_accept_drops(true);
        self.list.set_drop_indicator_shown(true);

        main_window.resize(500, 500);
        //main_window.show();

        let open_file = self.ui.create_action();
        open_file.set_text("Open");

        let file_menu = self.ui.create_menu();
        file_menu.set_title("File");
        file_menu.add_action(&open_file);

        let menu_bar = main_window.menu_bar();
        menu_bar.add_menu(&file_menu);

        set_current_row_changed_event!(self.list, self, MyApp, MyApp::new_row_selected);
        set_triggered_event!(open_file, self, MyApp, MyApp::menu_selected);

        let layout = self.ui.create_v_box_layout();

        layout.add_widget(&button);
        layout.add_widget(&self.list);

        let window = self.ui.create_widget();
        window.set_layout(&layout);

        main_window.set_central_widget(&window);
        main_window.show();

        println!("setup: obj.privd {:p}", self.list.obj.as_ref().unwrap().privd);
        println!("setup: obj.privd location {:p}", &self.list);
        println!("setup: self     {:p}", self);
        println!("setup: self ref {:p}", &self);
        println!("---------------------------------------------------------------");

        self.app.exec();
    }
}


fn main() {
    let wrui_instance = SharedLibUi::new(get_wrui_path()).unwrap();
    let ui = wrui_instance.get_ui();
    let app = MyApp::new(&ui);
    app.run();
}
