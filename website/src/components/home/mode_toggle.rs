use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModeToggleProps {
    pub is_compress: bool,
    pub on_change: Callback<bool>,
}

#[function_component(ModeToggle)]
pub fn mode_toggle(props: &ModeToggleProps) -> Html {
    let on_encode = {
        let on_change = props.on_change.clone();
        move |_| on_change.emit(true)
    };
    let on_decode = {
        let on_change = props.on_change.clone();
        move |_| on_change.emit(false)
    };

    html! {
        <div class="flex justify-center space-x-8 mb-12">
            <button
                onclick={on_encode}
                class={classes!("text-xl", "font-bold", "pb-2", "border-b-2", "transition-all", "tracking-widest",
                    if props.is_compress { "text-white  border-white" } else { "text-silver border-transparent hover:text-white" })}>
                {"ENCODE"}
            </button>
            <button
                onclick={on_decode}
                class={classes!("text-xl", "font-bold", "pb-2", "border-b-2", "transition-all", "tracking-widest",
                    if !props.is_compress { "text-white  border-white" } else { "text-silver border-transparent hover:text-white" })}>
                {"DECODE"}
            </button>
        </div>
    }
}
