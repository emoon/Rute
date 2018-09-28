///
/// This shows a basic usage of creating an application but it doesn't really do much.
///
extern crate rute;

use rute::*;
//use rute::ApplicationStaticType;

fn main() {
    let rute = Rute::new();

    // Create the application
    let app = rute.create_application();

    // Show built-in about qt dialog
    app.about_qt();

    // Start the application
    rute.application().exec();
}

