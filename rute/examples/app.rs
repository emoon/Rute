///
/// This shows a basic usage of creating an application but it doesn't really do much.
///
extern crate rute;

use rute::*;

fn main() {
    Rute::new();

    // Create the application
    let _app = Application::new();

    // Show built-in about qt dialog
    Application::about_qt();

    // Start the application
    Application::exec();
}
