use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileDropZoneProps {
    pub on_change: Callback<Event>,
    pub files_count: usize,
}

#[function_component(FileDropZone)]
pub fn file_drop_zone(props: &FileDropZoneProps) -> Html {
    html! {
        <div class="border-2 border-dashed border-silver/30 rounded-xl py-16 flex flex-col items-center group hover:bg-zinc-800/50 transition-all cursor-pointer relative">
            <input type="file"
                multiple=true
                onchange={props.on_change.clone()}
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
                    if props.files_count == 0 {
                        "Choose file or folder to process".to_string()
                    } else {
                        format!("{} items selected", props.files_count)
                    }
                }
            </p>
        </div>
    }
}