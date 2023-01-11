use monaco::{api::CodeEditorOptions, sys::editor::BuiltinTheme, yew::CodeEditor};
use std::rc::Rc;
use yew::{html, Component, Context, Html};

fn get_options() -> CodeEditorOptions {
    CodeEditorOptions::default()
        .with_language("rust".to_owned())
        // .with_value(CONTENT.to_owned())
        .with_builtin_theme(BuiltinTheme::VsDark)
        .with_automatic_layout(true)
}

struct RenderForm {
    options: Rc<CodeEditorOptions>,
}

impl Component for RenderForm {
    type Message = ();
    type Properties = ();

    fn create(_context: &Context<Self>) -> Self {
        Self {
            options: Rc::new(get_options()),
        }
    }

    fn changed(&mut self, _context: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {<form>
                <div class="h-96">
                    <label for="template" class="text-lg font-medium text-gray-900">{"Template"}</label>
                    <div class="mt-1 full-height">
                        <CodeEditor classes={"full-height"} options={ self.options.to_sys_options() } />
                    </div>
                </div>

                <div class="mt-10">
                    <label for="variables" class="text-lg font-medium text-gray-900">{"Variables (JSON)"}</label>
                    <div class="mt-2">
                        <textarea id="variables" name="variables" rows="10" cols="80" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"></textarea>
                    </div>
                </div>

                <div>
                    <div class="mt-2">
                        <button id="render" type="button" class="inline-flex items-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
                            {"Render"}
                        </button>
                    </div>
                </div>
            </form>
        }
    }
}

struct App {}
impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_context: &Context<Self>) -> Self {
        Self {}
    }

    fn changed(&mut self, _context: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, _context: &Context<Self>) -> Html {
        html! {<div>
            <nav class="bg-gray-800 ">
                <div class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                    <div class="relative flex h-16 items-center justify-between">
                        <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
                            <div class="flex flex-shrink-0 items-center">
                            </div>
                            <div class="hidden sm:ml-6 sm:block">
                                <div class="flex space-x-4">
                                    <a href="https://minijinja.jdns.me/" class="bg-gray-900 text-white px-3 py-2 rounded-md text-sm font-medium" aria-current="page">{"Debug Playground"}</a>
                                    <a href="https://minijinja.jdns.me/debug" class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium">{"Production Playground"}</a>
                                    <a href="https://github.com/joshuataylor/minijinja-playground" class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium">{"MiniJinja Playground GitHub"}</a>
                                    <a href="https://github.com/mitsuhiko/minijinja" class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium">{"MiniJinja GitHub"}</a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>

            <div class="container mx-auto">
                <div id="wrapper">
                    <div class="flex mt-4">
                        <div class="w-1/2 editor" id="editor">
                            <RenderForm />
                        </div>
                        <div class="flex-1 w-1/2 ml-10">
                            <div id="render-wrapper" class="container mx-auto"></div>
                        </div>
                    </div>

                    <div class="output" id="output">
                    </div>
                </div>
            </div>
        </div>
        }
    }
}

fn main() {
    wasm_log::init(wasm_log::Config::default());
    log::info!("Some info");

    yew::Renderer::<App>::new().render();
}
