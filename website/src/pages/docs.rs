use pulldown_cmark::{Options, Parser, html as markdown_html};
use yew::prelude::*;

#[function_component(Docs)]
pub fn docs() -> Html {
    let markdown_content = r#"
# Documentation
## Overview
`press.rs` is a high-performance compression library written in Rust.

### Core Functions
- `compress(data: &[u8]) -> Vec<u8>`: Compresses the input byte stream.
- `decompress(data: &[u8]) -> Result<Vec<u8>, Error>`: Restores the original data.

### Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
press-rs = "0.1.0"
"#;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(markdown_content, options);
    let mut html_output = String::new();
    markdown_html::push_html(&mut html_output, parser);

    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    div.set_inner_html(&html_output);

    html! {
        <div class="max-w-4xl mx-auto px-6 py-20 prose prose-invert prose-silver">
            <div class="doc-container bg-zinc-900/50 p-10 rounded-2xl border border-silver/10">
                { Html::from_html_unchecked(AttrValue::from(html_output)) }
            </div>
        </div>
    }
}
