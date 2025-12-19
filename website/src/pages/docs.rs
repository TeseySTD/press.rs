use pulldown_cmark::{Options, Parser, html as markdown_html};
use wasm_bindgen::prelude::*;
use web_sys::{Element, window};
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs)]
    fn highlightAll();
}

#[function_component(Docs)]
pub fn docs() -> Html {
    let markdown_content = include_str!("../data/docs.md");

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(markdown_content, options);
    let mut html_output = String::new();
    markdown_html::push_html(&mut html_output, parser);

    use_effect_with((), move |_| {
        highlightAll();

        let document = window().unwrap().document().unwrap();
        let pre_blocks = document.query_selector_all("pre").unwrap();

        for i in 0..pre_blocks.length() {
            if let Some(pre) = pre_blocks.get(i).and_then(|n| n.dyn_into::<Element>().ok()) {
                let _ = pre.set_attribute(
                    "class",
                    &format!(
                        "{} relative group",
                        pre.get_attribute("class").unwrap_or_default()
                    ),
                );

                if pre.query_selector(".copy-button").unwrap().is_none() {
                    let btn = document.create_element("button").unwrap();
                    let _ = btn.set_attribute("class", "copy-button absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity bg-white/10 hover:bg-white/20 text-white/50 hover:text-white px-2 py-1 rounded text-[10px] font-bold uppercase tracking-tighter");
                    btn.set_inner_html("Copy");

                    let pre_clone = pre.clone();
                    let btn_clone = btn.clone();
                    let on_copy = Closure::wrap(Box::new(move |_e: web_sys::Event| {
                        let text = pre_clone
                            .text_content()
                            .unwrap_or_default()
                            .replace("Copy", "");
                        let navigator = window().unwrap().navigator();
                        let _ = navigator.clipboard().write_text(&text);

                        btn_clone.set_inner_html("Done!");
                        let b = btn_clone.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            gloo_timers::future::TimeoutFuture::new(1000).await;
                            b.set_inner_html("Copy");
                        });
                    })
                        as Box<dyn FnMut(web_sys::Event)>);

                    let _ = btn.add_event_listener_with_callback(
                        "click",
                        on_copy.as_ref().unchecked_ref(),
                    );
                    on_copy.forget();

                    let _ = pre.append_child(&btn);
                }
            }
        }
        || ()
    });

    html! {
        <div class="min-h-screen bg-black text-alabaster px-6 py-20 animate-fade-in">
            <div class="max-w-4xl mx-auto">
                <div class="flex flex-col md:flex-row gap-12">
                    <main class="flex-grow">
                        <div class="prose prose-invert prose-zinc max-w-none
                        prose-headings:text-white prose-headings:tracking-tight
                        prose-a:text-silver hover:prose-a:text-white
                        prose-code:text-silver prose-code:bg-transparent prose-code:p-0
                        prose-pre:bg-[#1a1b26] prose-pre:border prose-pre:border-white/5 prose-pre:rounded-xl prose-pre:p-0">

                            { Html::from_html_unchecked(AttrValue::from(html_output)) }
                        </div>
                    </main>
                </div>
            </div>
        </div>
    }
}
