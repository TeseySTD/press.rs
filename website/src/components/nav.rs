use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="flex justify-between items-center px-10 py-6 border-b border-silver/10 backdrop-blur-md sticky top-0 z-50 bg-black/50">
            <div class="flex items-center space-x-3 cursor-pointer">
                <div class="w-10 h-10 bg-white rounded-sm flex items-center justify-center text-black font-black text-sm tracking-tighter">
                    {"PR"}
                </div>
                <div class="flex flex-col">
                    <span class="text-xl font-bold tracking-tighter italic text-white leading-none">{"PRESS.RS"}</span>
                    <span class="text-[10px] text-silver tracking-[0.3em] uppercase">{"Compression"}</span>
                </div>
            </div>
            
            <div class="flex space-x-10 text-xs uppercase tracking-[0.2em] font-medium">
                <Link<Route> to={Route::Home} classes="text-silver hover:text-white transition-all duration-500 hover:scale-105">
                    {"Home"}
                </Link<Route>>
                <Link<Route> to={Route::Docs} classes="text-silver hover:text-white transition-all duration-500 hover:scale-105">
                    {"Documentation"}
                </Link<Route>>
            </div>
        </nav>
    }
}