///
/// This shows a basic usage of creating an application but it doesn't really do much.
///
extern crate rute;

use rute::Rute;

fn main() {
    let temp = 0;
    let rute = Rute::new();

    // Create the application
    let app = rute.create_application();

    let widget = rute.create_widget();
    widget.show();

    app.about_to_quit(&temp, |_| {
        println!("About to quit!");
    });

    // Start the application
    app.exec();
}

