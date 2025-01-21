mod interpreter;

use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc, Color};
use std::collections::HashMap;
use interpreter::{run_interpreter};
use crate::interpreter::Value;

#[derive(Clone, Data, Lens)]
struct AppState {
    code: String,
    output: String,
}

fn main() {
    let initial_state = AppState {
        code: String::new(),
        output: String::new(),
    };
    let main_window = WindowDesc::new(build_ui())
        .title("BAUDOL: The official BAUx2 IDE")
        .window_size((1000.0, 800.0));

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("bau bau... couldn't launch :(");
}

fn build_ui() -> impl Widget<AppState> {
    use druid::widget::{Flex, TextBox, Button, Scroll};

    let primary_color = Color::rgb8(241, 166, 214);
    let secondary_color = Color::rgb8(145, 168, 209);
    let background_color = Color::rgb8(247, 202, 201);

    let code_input = TextBox::multiline()
        .with_placeholder("BAU \"Bau Bau World!\"")
        .lens(AppState::code)
        .expand_width()
        .height(380.0)
        .background(background_color)
        .padding(10.0);

    let output_textbox = TextBox::multiline()
        .with_placeholder("Bau Bau World!")
        .lens(AppState::output)
        .expand_width()
        .height(220.0)
        .background(secondary_color)
        .padding(10.0);

    let output_scroll = Scroll::new(output_textbox)
        .vertical();

    let execute_button = Button::new("Run")
        .on_click(|_ctx, data: &mut AppState, _env| {
            let mut variables: HashMap<String, Value> = HashMap::new();

            data.output.clear();
            run_interpreter(&data.code, &mut variables, &mut data.output);
        })
        .padding(2.0)
        .background(primary_color)
        .fix_width(60.0)
        .border(primary_color, 4.0)
        .center();

    Flex::column()
        .with_child(execute_button)
        .with_spacer(20.0)
        .with_child(code_input)
        .with_spacer(20.0)
        .with_child(output_scroll)
        .padding(20.0)
        .background(background_color)
}