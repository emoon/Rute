///
/// This shows a basic usage of creating an application but it doesn't really do much.
///
extern crate rute;

use rute::Rute;

fn main() {
    let rute = Rute::new();

    // Create the application
    let app = rute.create_application();

    // Start the application
    app.exec();
}

