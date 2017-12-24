extern crate wrui;

use wrui::SharedLibUi;

#[cfg(target_os="windows")]
fn get_wrui_path() -> &'static str {
    "t2-output/win64-msvc-debug-default/wrui_qt.dll"
}

#[cfg(target_os="macos")]
fn get_wrui_path() -> &'static str {
    "t2-output/macosx-clang-debug-default/libwrui_qt.dylib"
}

fn main() {
    let wrui_instance = SharedLibUi::new(get_wrui_path()).unwrap();
    let ui = wrui_instance.get_ui();

    let app = ui.create_application();
    let button = ui.create_push_button();

    button.set_text("Press me!");
    button.show();

    app.exec();
}
