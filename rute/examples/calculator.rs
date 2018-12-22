extern crate rute;

use rute::auto::rute_enums::AlignmentFlag;
use rute::*;
use std::cell::RefCell;

const NUM_DIGI_BUTTONS: usize = 10;

enum UnaryOperator {
    /// Square root
    Sqrt,
    /// Power
    Pow,
    /// 1 / x
    Recip,
}

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
        self.display
            .set_read_only(true)
            .set_alignment(AlignmentFlag::AlignRight)
            .set_max_length(15);

        // Bump the size of the font
        /*
        if let Some(font) = self.display.font() {
            font.set_point_size(font.point_size() + 8);
            self.display.set_font(&font);
        }
        */

        let layout = GridLayout::new();

        // Construct the digit buttons
        for i in 0..NUM_DIGI_BUTTONS {
            let button = self.create_button(&i.to_string(), move |calculator| {
                calculator.digit_clicked(i)
            });

            // place the zero at the bottom
            if i == 0 {
                layout.add_widget_row_column(&button, 5, 1, rute::AlignmentFlag::AlignDefault);
            } else {
                let row = ((9 - i) / 3) + 2;
                let column = ((i - 1) % 3) + 1;
                layout.add_widget_row_column(
                    &button,
                    row as i32,
                    column as i32,
                    rute::AlignmentFlag::AlignDefault,
                );
            }
        }

        // Create the remaining buttons

        let point_button = self.create_button(".", Self::point_clicked);
        let change_sign_button = self.create_button("-/+", Self::change_sign_clicked);
        //let backspace_button = self.create_button("Backspace", Self::backspace_clicked);

        // Unray buttons

        let sqrt_button = self.create_button("Sqrt", |calculator| {
            calculator.unaray_operator(UnaryOperator::Sqrt)
        });
        let pow_button = self.create_button("Pow", |calculator| {
            calculator.unaray_operator(UnaryOperator::Pow)
        });
        let recip_button = self.create_button("1/x", |calculator| {
            calculator.unaray_operator(UnaryOperator::Recip)
        });

        // let backspace_button = ToolButton::new()
        //  .set_text("Backspace")
        //  .set_pressed_event_ud(self, Self::change_sign_clicked)
        //  .build();

        // Add the remaining buttons to the layout

        layout.add_widget_row_column_span(
            &self.display,
            0,
            0,
            1,
            6,
            rute::AlignmentFlag::AlignRight,
        );

        // Set layout and show
        self.main_widget
            .set_layout(&layout)
            .set_window_title("Calculator")
            .show();
    }

    /// Called every time a digit is being pressed
    fn digit_clicked(&self, value: usize) {
        let mut state = self.state.borrow_mut();
        let mut display_text = self.display.text();

        if display_text == "0" && value == 0 {
            return;
        }

        if state.waiting_for_operand {
            display_text.clear();
            state.waiting_for_operand = false;
        }

        self.display.set_text(&format!("{}{}", display_text, value));
    }

    /// Called when "." is clicked
    fn point_clicked(&self) {
        let mut state = self.state.borrow_mut();

        if state.waiting_for_operand {
            self.display.set_text("0");
        }

        let display_text = self.display.text();
        if !display_text.contains(".") {
            self.display.set_text(&format!("{}.", display_text));
        }

        state.waiting_for_operand = false;
    }

    /// Called when +/- is called
    fn change_sign_clicked(&self) {
        let text = self.display.text();
        let value = text.parse::<f64>().unwrap();

        if value > 0.0 {
            self.display.set_text(&format!("-{}", text));
        } else {
            self.display.set_text(&text[1..]);
        }
    }

    fn unaray_operator(&self, operator: UnaryOperator) {
        let operand = self.display.text().parse::<f64>().unwrap();
        let mut result = 0.0;

        match operator {
            UnaryOperator::Sqrt => {
                if operand < 0.0 {
                    return self.abort_operation();
                } else {
                    result = operand.sqrt();
                }
            }

            UnaryOperator::Recip => {
                if operand == 0.0 {
                    return self.abort_operation();
                } else {
                    result = 1.0 / operand;
                }
            }

            UnaryOperator::Pow => result = operand.powf(2.0),
        }

        self.display.set_text(&result.to_string());
        self.state.borrow_mut().waiting_for_operand = true;
    }

    fn abort_operation(&self) {
        self.clear_all();
        self.display.set_text("####");
    }

    fn clear_all(&self) {
        self.display.set_text("0");
        *self.state.borrow_mut() = CalculatorState::default();
    }

    fn create_button<F>(&self, text: &str, func: F) -> ToolButton
    where
        F: Fn(&Self) + 'a,
    {
        let button = ToolButton::new();
        button
            .set_size_policy_2(rute::Policy::Expanding, rute::Policy::Preferred)
            .set_pressed_event_ud(self, func);
        button.set_text(text);
        button
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
