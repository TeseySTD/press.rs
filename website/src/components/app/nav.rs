use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="flex justify-between items-center px-6 py-4 bg-gradient-to-r from-silver/5 to-black border-b border-silver/10 backdrop-blur-md sticky top-0 z-50 ">
            <Link<Route> to={Route::Home} classes="no-underline">
                <div class="flex items-center space-x-3 cursor-pointer transition-all duration-300 group">
                    <div class="w-10 h-10 rounded-sm flex items-center justify-center text-black font-black text-sm tracking-tighter">
                        <img src="assets/logo.svg" class="w-10 h-10 opacity-80 group-hover:opacity-100 transition-opacity"/>
                    </div>
                    <div class="flex flex-col">
                        <span class="text-xl font-major-mono tracking-tighter text-white leading-none uppercase">
                            {"PRESS.RS"}
                        </span>
                        <span class="text-[10px] text-silver tracking-[0.3em] uppercase mt-1">
                            {"Compression"}
                        </span>
                    </div>
                </div>
            </Link<Route>>
            <div class="flex space-x-10 text-xs uppercase tracking-[0.2em] font-medium">
                <Link<Route> to={Route::Home} classes="text-silver hover:text-white transition-all duration-300 hover:scale-105">
                    {"Home"}
                </Link<Route>>
                <Link<Route> to={Route::Docs} classes="text-silver hover:text-white transition-all duration-300 hover:scale-105">
                    {"Documentation"}
                </Link<Route>>
            </div>
        </nav>
    }
}
