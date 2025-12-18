use crate::utils::download_file;
use gloo_file::File;
use gloo_file::futures::read_as_bytes;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let is_compress = use_state(|| true);
    let selected_file = use_state(|| None::<File>);
    let is_processing = use_state(|| false);

    let on_file_change = {
        let selected_file = selected_file.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    selected_file.set(Some(file.into()));
                }
            }
        })
    };

    let on_process = {
        Callback::from(move |_| {
            !todo!("Implement processing logic");
        })
    };

    html! {
        <div class="flex flex-col items-center justify-center min-h-[80vh] px-4 animate-fade-in">
            <div class="max-w-2xl w-full bg-black border border-silver/20 rounded-2xl p-8 shadow-[0_0_50px_-12px_rgba(255,255,255,0.1)] transition-all duration-500 hover:border-silver/50">
                // Mode switcher
                <div class="flex justify-center space-x-8 mb-12">
                    <button onclick={let is_compress = is_compress.clone(); move |_| is_compress.set(true)}
                        class={classes!("text-xl", "font-bold", "pb-2", "tracking-tighter", "transition-all", if *is_compress { "text-white border-b-2 border-white" } else { "text-silver/40 hover:text-white" })}>
                        {"COMPRESS"}
                    </button>
                    <button onclick={let is_compress = is_compress.clone(); move |_| is_compress.set(false)}
                        class={classes!("text-xl", "font-bold", "pb-2", "tracking-tighter", "transition-all", if !*is_compress { "text-white border-b-2 border-white" } else { "text-silver/40 hover:text-white" })}>
                        {"DECOMPRESS"}
                    </button>
                </div>

                // File uploader
                <div class="border-2 border-dashed border-silver/10 rounded-xl py-16 flex flex-col items-center group hover:bg-zinc-900/30 transition-all cursor-pointer relative">
                    <input type="file" onchange={on_file_change} class="absolute inset-0 opacity-0 cursor-pointer" />
                    <div class="w-16 h-16 mb-4 text-silver/20 group-hover:text-white transition-colors duration-700">
                        <svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" /></svg>
                    </div>
                    <p class="text-silver/60 group-hover:text-white font-medium">
                        { if let Some(f) = &*selected_file { f.name() } else { "Drop file here or browse".to_string() } }
                    </p>
                </div>

                // Action button
                if selected_file.is_some() {
                    <button onclick={on_process} disabled={*is_processing}
                        class="mt-8 w-full py-4 bg-white text-black font-black uppercase tracking-widest hover:bg-alabaster active:scale-[0.98] transition-all rounded-lg disabled:opacity-50">
                        { if *is_processing { "Processing..." } else if *is_compress { "Compress Data" } else { "Decompress Data" } }
                    </button>
                }
            </div>

            <p class="mt-8 text-[10px] text-silver/30 uppercase tracking-[0.4em]">{"Press.rs Engine v0.1.0"}</p>
        </div>
    }
}
