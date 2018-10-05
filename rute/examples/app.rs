///
/// This shows a basic usage of creating an application but it doesn't really do much.
///
extern crate rute;

use rute::*;

fn main() {
    Rute::new();

    // Create the application
    let app = Application::new();

    // this event will happen just when the application is about to quit
    app.set_about_to_quit_event(|| println!("about to quit!"));

    // Show built-in about qt dialog
    Application::about_qt();

    // Start the application
    Application::exec();
}
