extern crate rute;

use rute::auto::rute_enums::AlignmentFlag;
use rute::*;
use std::cell::RefCell;

const NUM_DIGI_BUTTONS: usize = 10;

#[derive(Clone, Copy, Debug)]
enum UnaryOperator {
    /// Square root
    Sqrt,
    /// Power
    Pow,
    /// 1 / x
    Recip,
}

#[derive(Clone, Copy, Debug)]
enum Operator {
	/// Addition
	Add,
	/// Subtraction
	Sub,
	/// Dividec
	Div,
	/// Multiply
	Mul,
}

#[derive(Default)]
struct CalculatorState {
    sum_in_memory: f64,
    sum_so_far: f64,
    factor_so_far: f64,
    pending_additive_aperator: Option<Operator>,
    pending_multiplicative_operator: Option<Operator>,
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
        if let Some(font) = self.display.font() {
            font.set_point_size(font.point_size() + 8);
            self.display.set_font(&font);
        }

        let layout = GridLayout::new()
            .set_size_constraint(SizeConstraint::SetFixedSize)
            .build();

        // Construct the digit buttons
        for i in 0..NUM_DIGI_BUTTONS {
            let button = self.create_button(&i.to_string(), move |calculator| {
                calculator.digit_clicked(i)
            });

            // place the zero at the bottom
            if i == 0 {
                layout.add_widget_row_column(&button, 5, 1, AlignmentFlag::AlignDefault);
            } else {
                let row = ((9 - i) / 3) + 2;
                let column = ((i - 1) % 3) + 1;
                layout.add_widget_row_column(
                    &button,
                    row as i32,
                    column as i32,
                    AlignmentFlag::AlignDefault,
                );
            }
        }

        // Create the remaining buttons

        let point_button = self.create_button(".", Self::point_clicked);
        let change_sign_button = self.create_button("-/+", Self::change_sign_clicked);

        let backspace_button = self.create_button("Backspace", Self::backspace_clicked);
        let clear_button = self.create_button("Clear", Self::backspace_clicked);
        let clear_all_button = self.create_button("Clear All", |calculator| calculator.clear_all());

        // Memory buttons

        let clear_memory_button = self.create_button("MC", Self::clear_memory);
        let read_memory_button = self.create_button("MR", Self::read_memory);
        let set_memory_button = self.create_button("MS", Self::set_memory);
        let add_to_memory_button = self.create_button("M+", Self::add_to_memory);

        // Div/Mul buttons

        let divide_button = self.create_button("/", |calculator| {
        	calculator.mul_div_operator(Operator::Div)
        });

        let multiply_button = self.create_button("*", |calculator| {
        	calculator.mul_div_operator(Operator::Mul)
        });

        // Add/Sub buttons

        let addition_button = self.create_button("+", |calculator| {
        	calculator.add_sub_operator(Operator::Add)
        });

        let subtract_button = self.create_button("-", |calculator| {
        	calculator.add_sub_operator(Operator::Sub)
        });

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

        let equal_button = self.create_button("=", Self::equal_clicked);

        // Add the remaining buttons to the layout

        layout.add_widget_row_column_span(
            &self.display,
            0, 0, 1, 6,
            rute::AlignmentFlag::AlignDefault,
        );

		layout.add_widget_row_column_span(&backspace_button, 1, 0, 1, 2, AlignmentFlag::AlignDefault);
		layout.add_widget_row_column_span(&clear_button, 1, 2, 1, 2, AlignmentFlag::AlignDefault);
		layout.add_widget_row_column_span(&clear_all_button, 1, 4, 1, 2, AlignmentFlag::AlignDefault);

		layout.add_widget_row_column(&clear_memory_button, 2, 0, AlignmentFlag::AlignDefault);
		layout.add_widget_row_column(&read_memory_button, 3, 0, AlignmentFlag::AlignDefault);
		layout.add_widget_row_column(&set_memory_button, 4, 0, AlignmentFlag::AlignDefault);
		layout.add_widget_row_column(&add_to_memory_button, 5, 0, AlignmentFlag::AlignDefault);

		layout.add_widget_row_column(&point_button, 5, 2, AlignmentFlag::AlignDefault);
		layout.add_widget_row_column(&change_sign_button, 5, 3, AlignmentFlag::AlignDefault);

		layout.add_widget_row_column(&divide_button, 2, 4, AlignmentFlag::AlignDefault);
		layout.add_widget_row_column(&multiply_button, 3, 4, AlignmentFlag::AlignDefault);
		layout.add_widget_row_column(&subtract_button, 4, 4, AlignmentFlag::AlignDefault);
		layout.add_widget_row_column(&addition_button, 5, 4, AlignmentFlag::AlignDefault);

		layout.add_widget_row_column(&sqrt_button, 2, 5, AlignmentFlag::AlignDefault);
		layout.add_widget_row_column(&pow_button, 3, 5, AlignmentFlag::AlignDefault);
		layout.add_widget_row_column(&recip_button, 4, 5,AlignmentFlag::AlignDefault);
		layout.add_widget_row_column(&equal_button, 5, 5, AlignmentFlag::AlignDefault);

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

        println!("set text {}", display_text);

        self.display.set_text(&format!("{}{}", display_text, value));
    }

    fn backspace_clicked(&self) {
        let mut state = self.state.borrow_mut();

        if state.waiting_for_operand {
        	return;
        }

        let text = self.display.text();

    }

    fn clear_memory(&self) {

    }

    fn read_memory(&self) {

    }

    fn set_memory(&self) {

    }

    fn add_to_memory(&self) {

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

    fn mul_div_operator(&self, operator: Operator) {
        let mut state = self.state.borrow_mut();
        let mut operand = self.display.text().parse::<f64>().unwrap();

        if let Some(operator) = state.pending_multiplicative_operator {
        	if !Self::calculate(&mut state, operand, operator) {
        		self.abort_operation();
        		return;
        	}

        	self.display.set_text(&state.factor_so_far.to_string());
        }

        state.pending_multiplicative_operator = Some(operator);
        state.waiting_for_operand = true;
    }

    fn add_sub_operator(&self, clicked_operator: Operator) {
        let mut state = self.state.borrow_mut();
        let mut operand = self.display.text().parse::<f64>().unwrap();

        if let Some(operator) = state.pending_multiplicative_operator {
        	if !Self::calculate(&mut state, operand, operator) {
        		self.abort_operation();
        		return;
        	}

        	self.display.set_text(&state.factor_so_far.to_string());
        	operand = state.factor_so_far;
        	state.factor_so_far = 0.0;
        	state.pending_multiplicative_operator = None;
        }

        if let Some(operator) = state.pending_additive_aperator {
        	if !Self::calculate(&mut state, operand, operator) {
        		return self.abort_operation();
        	}

        	self.display.set_text(&state.sum_so_far.to_string());
        } else {
        	state.sum_so_far = operand;
        }

        state.pending_multiplicative_operator = Some(clicked_operator);
        state.waiting_for_operand = true;
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

    fn equal_clicked(&self) {
        let mut operand = self.display.text().parse::<f64>().unwrap();
        let mut state = self.state.borrow_mut();

        if let Some(operator) = state.pending_multiplicative_operator {
        	if !Self::calculate(&mut state, operand, operator) {
        		return self.abort_operation();
        	}

			operand = state.factor_so_far;
			state.factor_so_far = 0.0;
			state.pending_multiplicative_operator = None;
        }

        if let Some(operator) = state.pending_additive_aperator {
        	if !Self::calculate(&mut state, operand, operator) {
        		return self.abort_operation();
        	}

        	state.pending_additive_aperator = None;
        } else {
        	state.sum_so_far = operand;
        }

        self.display.set_text(&state.sum_so_far.to_string());
        state.sum_so_far = 0.0;
        state.waiting_for_operand = true;
    }

    fn calculate(state: &mut CalculatorState, right: f64, operator: Operator) -> bool {
    	match operator {
    		Operator::Add => state.sum_so_far += right, 
    		Operator::Sub => state.sum_so_far -= right, 
    		Operator::Mul => state.factor_so_far *= right, 
    		Operator::Div => {
    			if right == 0.0 {
    				return false;
    			}

    			state.factor_so_far /= right;
    		}
    	}
    	true
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
        ToolButton::new()
			.set_size_policy_2(rute::Policy::Preferred, rute::Policy::Preferred)
			.set_pressed_event_ud(self, func)
			.set_text(text)
			.build()
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
