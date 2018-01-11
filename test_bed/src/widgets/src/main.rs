#[macro_use]
extern crate wrui;

use std::os::raw::c_void;

use wrui::{SharedLibUi, Ui};
use wrui::wrui::*;

#[cfg(target_os = "windows")]
fn get_wrui_path() -> &'static str {
    "t2-output/win64-msvc-debug-default/wrui_qt.dll"
}

#[cfg(target_os = "macos")]
fn get_wrui_path() -> &'static str {
    "t2-output/macosx-clang-debug-default/libwrui_qt.dylib"
}

#[repr(C)]
struct MyApp<'a> {
    ui: &'a Ui,
    list: ListWidget,
    main_win: Widget,
    app: Application,
}

impl<'a> MyApp<'a> {
    fn new(ui: &'a Ui) -> MyApp {
        MyApp {
            ui: ui,
            app: ui.create_application(),
            list: ui.create_list_widget(),
            main_win: ui.create_widget(),
        }
    }

    fn new_row_selected(&mut self, row: i32) {
        println!("new row {}", row);
    }

    fn menu_selected(&mut self) {
        //println!("menu_selected: obj.privd {:p}", self.list.obj.as_ref().unwrap().privd);

        for selected in &self.list.selected_items() {
            println!("selected item {}", selected.text());
        }

        println!("menu select");
    }

    fn drag_enter(&mut self, event: &DragEnterEvent) {
        println!("Dropping files!");
        event.accept();
    }

    fn drop_files(&mut self, event: &DropEvent) {
        for url in event.mime_data().urls() {
            if url.is_local_file() {
                self.list.add_item(&url.to_local_file());
                //println!("Has local file {}", url.to_local_file());
            } else {
                println!("File is not local");
            }
        }

        event.accept_proposed_action();
    }

    fn custom_draw_widget(&mut self, _event: &PaintEvent) {
        println!("begin drawing\n");

        let mut painter = self.ui.create_painter();

        painter.begin(&self.main_win);
        painter.draw_line(0, 0, 20, 20);
        painter.end();
        painter.destroy();

        println!("end drawing!");
    }

    fn run(&mut self) {
        let main_window = self.ui.create_main_window();
        let button = self.ui.create_push_button();

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
        set_drag_enter_event!(self.list, self, MyApp, MyApp::drag_enter);
        set_drop_event!(self.list, self, MyApp, MyApp::drop_files);
        set_current_row_changed_event!(self.list, self, MyApp, MyApp::new_row_selected);

        set_triggered_event!(open_file, self, MyApp, MyApp::menu_selected);

        let layout = self.ui.create_v_box_layout();

        layout.add_widget(&button);
        layout.add_widget(&self.list);

        //let window = self.ui.create_widget();
        self.main_win.set_layout(&layout);

        set_paint_event!(self.main_win, self, MyApp, MyApp::custom_draw_widget);

        main_window.set_central_widget(&self.main_win);
        main_window.show();

        self.app.exec();
    }
}

fn main() {
    let wrui_instance = SharedLibUi::new(get_wrui_path()).unwrap();
    let ui = wrui_instance.get_ui();
    let mut app = MyApp::new(&ui);
    app.run();
}
