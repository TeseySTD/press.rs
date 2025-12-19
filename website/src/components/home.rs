use crate::utils::download_file;
use gloo_file::{File, futures::read_as_bytes};
use press_rs::packager::FileEntry;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let is_compress = use_state(|| true);
    let selected_files = use_state(|| Vec::<File>::new());
    let is_processing = use_state(|| false);

    let on_file_change = {
        let selected_files = selected_files.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                let mut vec = Vec::new();
                for i in 0..files.length() {
                    if let Some(f) = files.get(i) {
                        vec.push(f.into());
                    }
                }
                selected_files.set(vec);
            }
        })
    };

    let on_process = {
        let is_compress = is_compress.clone();
        let selected_files = selected_files.clone();
        let is_processing = is_processing.clone();

        Callback::from(move |_| {
            let files = (*selected_files).clone();
            let is_compress = *is_compress;
            let is_processing = is_processing.clone();

            is_processing.set(true);

            spawn_local(async move {
                if is_compress {
                    let mut entries = Vec::new();
                    for file in files {
                        if let Ok(data) = read_as_bytes(&file).await {
                            entries.push(FileEntry {
                                name: file.name(),
                                data,
                                is_dir: false,
                            });
                        }
                    }

                    let packed_data = press_rs::packager::pack_entries(entries);
                    let compressed = press_rs::compressor::compress_raw(&packed_data);
                    download_file("archive.pressrs", &compressed);
                } else {
                    if let Some(file) = files.get(0) {
                        if let Ok(archive_data) = read_as_bytes(file).await {
                            let decompressed = press_rs::compressor::decompress_raw(&archive_data);
                            let entries = press_rs::packager::unpack_to_entries(decompressed);

                            for entry in entries {
                                if !entry.is_dir {
                                    download_file(&entry.name, &entry.data);
                                }
                            }
                        }
                    }
                }
                is_processing.set(false);
            });
        })
    };

    html! {
        <div class="flex flex-col items-center justify-center min-h-[80vh] px-4 animate-fade-in bg-black">
            <div class="max-w-2xl w-full bg-zinc-900 border border-silver/20 rounded-2xl p-8 shadow-2xl transition-all duration-500 hover:border-silver/50">

                <div class="flex justify-center space-x-8 mb-12">
                    <button
                        onclick={let ic = is_compress.clone(); move |_| ic.set(true)}
                        class={classes!("text-xl", "font-bold", "pb-2", "transition-all", "tracking-widest",
                            if *is_compress { "text-white border-b-2 border-white" } else { "text-silver hover:text-white" })}>
                        {"ENCODE"}
                    </button>
                    <button
                        onclick={let ic = is_compress.clone(); move |_| ic.set(false)}
                        class={classes!("text-xl", "font-bold", "pb-2", "transition-all", "tracking-widest",
                            if !*is_compress { "text-white border-b-2 border-white" } else { "text-silver hover:text-white" })}>
                        {"DECODE"}
                    </button>
                </div>

                <div class="border-2 border-dashed border-silver/30 rounded-xl py-16 flex flex-col items-center group hover:bg-zinc-800/50 transition-all cursor-pointer relative">
                    <input type="file"
                        multiple=true
                        onchange={on_file_change}
                        class="absolute inset-0 opacity-0 cursor-pointer" />

                    <div class="w-20 h-20 mb-4 text-silver group-hover:text-alabaster transition-colors duration-500">
                        <svg fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1"
                                d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12">
                            </path>
                        </svg>
                    </div>

                    <p class="text-silver group-hover:text-alabaster font-medium tracking-wide">
                        {
                            if selected_files.is_empty() {
                                "Choose file or folder to process".to_string()
                            } else {
                                format!("{} items selected", selected_files.len())
                            }
                        }
                    </p>
                </div>

                if !selected_files.is_empty() {
                    <button
                        onclick={on_process}
                        disabled={*is_processing}
                        class="mt-10 w-full py-4 bg-alabaster text-black font-black uppercase tracking-[0.3em] hover:bg-white active:scale-95 transition-all rounded-lg shadow-lg disabled:opacity-30">
                        { if *is_processing { "Processing..." } else { "Execute Process" } }
                    </button>
                }
            </div>

            <p class="mt-12 text-[10px] text-silver/40 uppercase tracking-[0.5em]">{"Powered by press.rs engine"}</p>
        </div>
    }
}
