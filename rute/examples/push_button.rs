///
/// This shows a basic usage of creating an push button and a callback
///
extern crate rute;

use rute::*;

fn main() {
    Rute::new();

    // Create the application
    let _app = Application::new();

    let button = PushButton::new();

    button
        .set_text("Push me!")
        .set_pressed_event(|| println!("Button was pressed!"))
        .show();

    // Start the application
    Application::exec();
}
