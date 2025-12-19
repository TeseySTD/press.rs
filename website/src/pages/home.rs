use crate::components::home::file_drop_zone::FileDropZone;
use crate::components::home::mode_toggle::ModeToggle;

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

                <ModeToggle
                    is_compress={*is_compress}
                    on_change={Callback::from(move |val| is_compress.set(val))}
                />

                <FileDropZone
                    on_change={on_file_change}
                    files_count={selected_files.len()}
                />

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
