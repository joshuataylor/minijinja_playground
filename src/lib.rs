use chrono::Utc;
use minijinja::machinery::CompiledTemplate;
use minijinja::value::Value;
use minijinja::{render, Environment};

use std::{fmt, panic};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use web_sys::{HtmlButtonElement};

enum ParseErrorKind {
    Variable,
    MiniJinja,
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseErrorKind::Variable => write!(f, "variables"),
            ParseErrorKind::MiniJinja => write!(f, "minijinja"),
        }
    }
}

struct ParseError {
    kind: ParseErrorKind,
    message: String,
}

static MINIJINJA_TEMPLATE: &str = include_str!("../rendered.html.jinja");

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // console_error_panic_hook::set_once();
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");

    // Add "display: none" to the loading div
    let loading_div = document.get_element_by_id("loading").unwrap_throw();
    loading_div
        .set_attribute("style", "display: none")
        .unwrap_throw();

    // Remove display: none from the wrapper div
    let wrapper_div = document.get_element_by_id("wrapper").unwrap_throw();
    wrapper_div
        .set_attribute("style", "display: block")
        .unwrap_throw();

    let rendered_element = document
        .get_element_by_id("render-wrapper")
        .expect("should have #render-wrapper on the page");

    let template_element = document
        .get_element_by_id("template")
        .expect("should have #template on the page");

    let variables_element = document
        .get_element_by_id("variables")
        .expect("should have #variables on the page");

    let template_string = match render_template(
        &template_element
            .dyn_ref::<HtmlTextAreaElement>()
            .expect("#template be an `HtmlTextAreaElement`")
            .value(),
        &variables_element
            .dyn_ref::<HtmlTextAreaElement>()
            .expect("#template be an `HtmlTextAreaElement`")
            .value(),
    ) {
        Ok(rendered) => rendered,
        Err(e) => {
            render!(MINIJINJA_TEMPLATE, error_message => e.message, error_kind => e.kind.to_string())
        }
    };

    rendered_element.set_inner_html(&template_string);

    let update_preview_closure = Closure::<dyn Fn()>::new(move || {
        let template_string = match render_template(
            &template_element
                .dyn_ref::<HtmlTextAreaElement>()
                .expect("#template be an `HtmlTextAreaElement`")
                .value(),
            &variables_element
                .dyn_ref::<HtmlTextAreaElement>()
                .expect("#template be an `HtmlTextAreaElement`")
                .value(),
        ) {
            Ok(rendered) => rendered,
            Err(e) => {
                render!(MINIJINJA_TEMPLATE, error_message => e.message, error_kind => e.kind.to_string())
            }
        };

        rendered_element.set_inner_html(&template_string);
    });

    document
        .get_element_by_id("render")
        .expect("should have the render div on the page")
        .dyn_ref::<HtmlButtonElement>()
        .expect("#render button should be a `HtmlButtonElement`")
        .set_onclick(Some(update_preview_closure.as_ref().unchecked_ref()));

    update_preview_closure.forget();

    Ok(())
}

fn validate_variables(variables: &str) -> Result<Value, ParseError> {
    if variables.is_empty() {
        // @todo this is dumb, just return an empty object?
        return Ok(Value::default());
    }

    // The variables here may or not be valid JSON, we don't care unless the user has input later.
    serde_json_wasm::from_str(variables).map_err(wasm_error_to_string)
}

fn render_template(template_string: &str, variables_string: &str) -> Result<String, ParseError> {
    let variables = validate_variables(variables_string)?;

    // Here we define the environment each time.
    // Might be better to define it once, but given it's quick to do this,
    // and the heaviest part is the rendering, I'm not sure it's worth it.
    let mut env = Environment::new();
    env.set_debug(true);

    // Later we'll support multiple templates, so let's just use this now, instead
    // of direct rendering via the `render!` macro
    env.add_template("playground", template_string)
        .map_err(minijinja_error_to_string)?;

    let template = env
        .get_template("playground")
        .map_err(minijinja_error_to_string)?;

    // @todo use timestamp_nanos later
    let start: i64 = Utc::now().timestamp_micros();
    let rendered = template
        .render(variables)
        .map_err(minijinja_error_to_string)?;
    let diff = Utc::now().timestamp_micros() - start;

    // Can we do this without recompiling the template?
    let compiled =
        CompiledTemplate::from_name_and_source("playground", template_string).unwrap_throw();

    let mut instructions_list: String = String::new();

    let mut last_line: Option<usize> = None;
    for i in 0..compiled.instructions.len() {
        let instruction = compiled.instructions.get(i).unwrap_throw();
        let instruction_line = compiled.instructions.get_line(i);
        let line_info = if Some(instruction_line) == Some(last_line) {
            format!("{}", instruction_line.unwrap_or(0))
        } else {
            "-".to_string()
        };

        let id = format!("{i:>05x}");
        let instruction_output = format!("{instruction:?}");

        instructions_list.push_str(&format!("<tr><td class='whitespace-nowrap pl-4 pr-3 text-sm font-medium text-gray-900 py-2'>{line_info}</td><td class='whitespace-nowrap'>{id}</td><td class='whitespace-nowrap'>{instruction_output}</td></tr>"));

        last_line = instruction_line;
    }
    let fuel_info = env.fuel().unwrap_or(0);
    Ok(
        render!(MINIJINJA_TEMPLATE, rendered => rendered, errors => "", instructions => instructions_list, fuel_cost => fuel_info.to_string(), render_time => format!("{diff}ms")),
    )
}

fn minijinja_error_to_string(error: minijinja::Error) -> ParseError {
    ParseError {
        kind: ParseErrorKind::MiniJinja,
        message: format!("{error:#?}"),
    }
}

fn wasm_error_to_string(error: serde_json_wasm::de::Error) -> ParseError {
    ParseError {
        kind: ParseErrorKind::Variable,
        message: format!("{error:#?} - {error}"),
    }
}
