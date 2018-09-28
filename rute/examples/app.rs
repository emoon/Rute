///
/// This shows a basic usage of creating an application but it doesn't really do much.
///
extern crate rute;

use rute::Rute;
use rute::ApplicationStaticType;

fn main() {
    let rute = Rute::new();

    // Create the application
    //let app = rute.create_application();

    //let widget = rute.create_widget();
    //widget.show();

    rute.application().about_qt();

    /*
    app.set_about_to_quit_event(|| {
        println!("About to quit!");
    });
    */

    // Start the application
    rute.application().exec();
}

