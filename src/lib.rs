use minijinja::value::Value;
use minijinja::Environment;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use web_sys::{Document, Element, HtmlButtonElement, Window};

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
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
    let output_element = document
        .get_element_by_id("output")
        .expect("should have #current-time on the page");

    let template_element = document
        .get_element_by_id("template")
        .expect("should have #template on the page");

    let variables_element = document
        .get_element_by_id("variables")
        .expect("should have #variables on the page");

    let update_preview_closure = Closure::<dyn Fn()>::new(move || {
        update_preview(&output_element, &template_element, &variables_element)
    });

    fn update_preview(output: &Element, template: &Element, variables: &Element) {
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
            output.set_inner_html("Variables is not a valid JSON object.");
            return;
        }

        let ctx: Value = if variables_value.is_empty() {
            serde_json_wasm::from_str("{}").unwrap()
        } else {
            parsed_variables.unwrap()
        };

        let templates_value = template_textarea.value();

        // Here we define the environment each time.
        // Might be better to define it once, but given it's quick to do this,
        // and the heaviest part is the rendering, I'm not sure it's worth it.
        let mut env = Environment::new();

        env.add_template("playground", &templates_value).unwrap();

        let template = env.get_template("playground").unwrap();
        let rendered = template.render(ctx).unwrap();

        output.set_inner_html(&rendered);
    }

    document
        .get_element_by_id("render")
        .expect("should have the render div on the page")
        .dyn_ref::<HtmlButtonElement>()
        .expect("#render button should be a `HtmlButtonElement`")
        .set_onclick(Some(update_preview_closure.as_ref().unchecked_ref()));

    update_preview_closure.forget();

    Ok(())
}
