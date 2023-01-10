use chrono::Utc;
use minijinja::machinery::CompiledTemplate;
use minijinja::value::Value;
use minijinja::{render, Environment};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlTextAreaElement};
use web_sys::{Document, Element, HtmlButtonElement, Window};

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

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

    setup_minijinja_form(&window, &document)?;

    Ok(())
}

fn setup_minijinja_form(_window: &Window, document: &Document) -> Result<(), JsValue> {
    let rendered_element = document
        .get_element_by_id("render-wrapper")
        .expect("should have #render-wrapper on the page");

    let template_element = document
        .get_element_by_id("template")
        .expect("should have #template on the page");

    let variables_element = document
        .get_element_by_id("variables")
        .expect("should have #variables on the page");

    let update_preview_closure = Closure::<dyn Fn()>::new(move || {
        update_preview(&rendered_element, &template_element, &variables_element)
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

fn update_preview(template_element: &Element, template: &Element, variables: &Element) {
    let template_textarea = template
        .dyn_ref::<HtmlTextAreaElement>()
        .expect("#template be an `HtmlTextAreaElement`");

    let variables_textarea = variables
        .dyn_ref::<HtmlTextAreaElement>()
        .expect("#variables_textarea be an `HtmlTextAreaElement`");

    let variables_value = variables_textarea.value();

    // The variables here may or not be valid JSON, we don't care unless the user has input later.
    // @todo only check when the user has input
    // @todo check that the variables are valid JSON before this point?
    let parsed_variables = serde_json_wasm::from_str(&variables_value);

    if !variables_value.is_empty() && parsed_variables.is_err() {
        template_element.set_inner_html("Variables is not a valid JSON object.");
        return;
    }

    let ctx: Value = if variables_value.is_empty() {
        serde_json_wasm::from_str("{}").unwrap()
    } else {
        parsed_variables.unwrap_throw()
    };

    let templates_value = template_textarea.value();

    // Here we define the environment each time.
    // Might be better to define it once, but given it's quick to do this,
    // and the heaviest part is the rendering, I'm not sure it's worth it.
    let mut env = Environment::new();
    env.set_debug(true);

    // Later we'll support multiple templates, so let's just use this now, instead
    // of direct rendering via the `render!` macro
    env.add_template("playground", &templates_value)
        .unwrap_throw();

    let template = env.get_template("playground").unwrap_throw();

    // @todo use timestamp_nanos later
    let start: i64 = Utc::now().timestamp_micros();
    let rendered = template.render(ctx).unwrap_throw();
    let diff = Utc::now().timestamp_micros() - start;

    // @#todo: I will fix this double compile later. Forgive me for my sins.
    let compiled =
        CompiledTemplate::from_name_and_source("playground", &templates_value).unwrap_throw();
    let compiled_instructions = minijinja::machinery::instructions_list(compiled.instructions);

    let compiled_template =
        CompiledTemplate::from_name_and_source("playground", &templates_value).unwrap_throw();

    // List containing HTML elements of instructions
    let mut instructions_list: String = String::new();

    let mut last_line: Option<usize> = None;
    for (idx, instr) in compiled_instructions.iter().enumerate() {
        let line = compiled_template.instructions.get_line(idx);
        let line_info = if line == last_line {
            format!("{}", line.unwrap())
        } else {
            "-".to_string()
        };

        let instruction_info = format!("{idx:>05x}");
        let instruction_info2 = format!("{instr:?}");

        instructions_list.push_str(&format!("<tr><td class='whitespace-nowrap'>{line_info}</td><td class='whitespace-nowrap'>{instruction_info}</td><td class='whitespace-nowrap'>{instruction_info2}</td></tr>"));

        last_line = line;
    }

    // fuel info
    let fuel_info = env.fuel().unwrap_or(0);
    let mj_template = include_str!("../rendered.html.jinja");

    let output = render!(mj_template, rendered => rendered, errors => "", instructions => instructions_list, fuel_cost => fuel_info.to_string(), render_time => format!("{diff}ms"));
    template_element.set_inner_html(&output);
}
