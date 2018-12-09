extern crate rute;

use rute::*;
use std::cell::RefCell;
use rute::auto::rute_enums::AlignmentFlag;

const NUM_DIGI_BUTTONS: usize = 10;

#[derive(Default)]
struct CalculatorState {
    sum_in_memory: f64,
    sum_so_far: f64,
    factor_so_far: f64,
    pending_additive_aperator: String,
    pending_multiplicative_operator: String,
    waiting_for_operand: bool,
}

struct Calculator<'a> {
    state: RefCell<CalculatorState>,
    main_widget: Widget<'a>,
    display: LineEdit<'a>,
}

impl<'a> Calculator<'a> {
    fn new() -> Calculator<'a> {
        Calculator {
            main_widget: Widget::new(),
            display: LineEdit::new(),
            state: RefCell::new(CalculatorState::default()),
        }
    }

    ///
    /// Creates the UI for the calculator
    ///
    fn setup_ui(&'a mut self) {
        // Create the display of the calculator
        self.display.set_read_only(true)
               .set_alignment(AlignmentFlag::AlignRight)
               .set_max_length(15);

        // Bump the size of the font
        /*
        if let Some(font) = self.display.font() {
            font.set_point_size(font.point_size() + 8);
            self.display.set_font(&font);
        }
        */

        let main_widget = Widget::new();
        let layout = GridLayout::new();

        // Construct the digit buttons
        for i in 0..NUM_DIGI_BUTTONS {
            let button = ToolButton::new();
            button.set_pressed_event_ud(self, Self::digit_clicked);
            button.set_text(&format!("{}", i));

            // place the zero at the bottom
            if i == 0 {
                layout.add_widget_row_column(&button, 5, 1, rute::AlignmentFlag::AlignRight);
            } else {
                let row = ((9 - i) / 3) + 2;
                let column = ((i - 1) % 3) + 1;
                layout.add_widget_row_column(&button, row as i32, column as i32, rute::AlignmentFlag::AlignRight);
            }
        }

        // Stoe display and setup main widget
        self.main_widget.set_layout(&layout);
        self.main_widget.set_window_title("Calculator");
        self.main_widget.show();
    }

    ///
    /// Called every time a digit is being pressed
    ///
    fn digit_clicked(&self) {
        println!("pressing button!");
    }
}

fn main() {
    Rute::new();

    let _app = Application::new();

    // Setup the calculator
    let mut calculator = Calculator::new();
    calculator.setup_ui();

    // run the application
    Application::exec();
}
