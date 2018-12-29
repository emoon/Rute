///
/// This shows a basic usage of the rist widget
///
extern crate rute;

use rute::*;
use rute::auto::rute_enums::AlignmentFlag;

fn main() {
    Rute::new();

    // Create the application
    let _app = Application::new();

    let info_button = PushButton::new()
        .set_text("Show info")
        .build();

    let list = ListWidget::new();
    //let list_info = list.clone();

    let main = Widget::new();

    list.add_widget_item(&ListWidgetItem::new().set_text("Test 1").build());
    list.add_widget_item(&ListWidgetItem::new().set_text("Test 2").build());
    list.add_widget_item(&ListWidgetItem::new().set_text("Test 3").build());

    let layout = HBoxLayout::new()
        .add_widget(&list, 0, AlignmentFlag::AlignDefault)
        .add_widget(&info_button, 0, AlignmentFlag::AlignDefault)
        .build();

    // Show the selected items
    info_button.set_pressed_event(move || {
        println!("Selected items");
        for i in list.selected_items() {
            println!("{}", i.text());
        }
    });

    main.set_layout(&layout).show();

    // Start the application
    Application::exec();
}

