use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    let current_route = use_route::<Route>();
    let is_open = use_state(|| false);

    let on_toggle = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    let close_menu = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(false))
    };

    html! {
        <nav class="flex flex-col sticky top-0 z-50">
            // Main navigation
            <div class="flex justify-between items-center px-6 py-4 bg-gradient-to-r from-silver/5 to-black border-b border-silver/10 backdrop-blur-md">

                // Logo
                <Link<Route> to={Route::Home} classes="no-underline">
                    <div class="flex items-center space-x-3 cursor-pointer transition-all duration-300 group" onclick={close_menu.clone()}>
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

                // Desktop menu
                <div class="hidden md:flex space-x-10 text-xs uppercase tracking-[0.2em] font-medium">
                    <Link<Route> to={Route::Home} classes={
                                classes!(
                                    "hover:text-white", "transition-all", "duration-300",
                                    if current_route.as_ref() == Some(&Route::Home) { "text-white" } else { "text-silver" })
                                }>
                        {"Home"}
                    </Link<Route>>
                    <Link<Route> to={Route::Docs} classes={
                                classes!(
                                    "hover:text-white", "transition-all", "duration-300",
                                    if current_route.as_ref() == Some(&Route::Docs) { "text-white" } else { "text-silver" })
                                }>
                        {"Documentation"}
                    </Link<Route>>
                </div>

                // Hamburger button
                <button onclick={on_toggle} class="md:hidden relative w-8 h-8 text-silver hover:text-white focus:outline-none">

                    // X
                    <div class={classes!(
                        "absolute", "inset-0", "flex", "items-center", "justify-center", "transition-all", "duration-300", "transform",
                        if *is_open { "opacity-100 rotate-0 scale-100" } else { "opacity-0 -rotate-90 scale-50" }
                    )}>
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </div>

                    // Hamburger
                    <div class={classes!(
                        "absolute", "inset-0", "flex", "items-center", "justify-center", "transition-all", "duration-300", "transform",
                        if *is_open { "opacity-0 rotate-90 scale-50" } else { "opacity-100 rotate-0 scale-100" }
                    )}>
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7" />
                        </svg>
                    </div>

                </button>
            </div>

            // Dropdown menu
            if *is_open {
                <div class="md:hidden absolute top-full left-0 w-full bg-black/95 border-b border-silver/10 backdrop-blur-xl animate-fade-in z-40">
                    <div class="flex flex-col items-center py-6 space-y-6 text-sm uppercase tracking-[0.2em] font-medium">
                        <span onclick={close_menu.clone()}>
                            <Link<Route> to={Route::Home}
                                classes={classes!(
                                            "hover:text-white", "py-2", "block", "w-full", "text-center", 
                                            if current_route.as_ref() == Some(&Route::Home) { "text-white" } else { "text-silver" }
                                        )}>
                                {"Home"}
                            </Link<Route>>
                        </span>
                        <span onclick={close_menu}>
                            <Link<Route> to={Route::Docs}
                                        classes={classes!(
                                            "hover:text-white", "py-2", "block", "w-full", "text-center",
                                            if current_route.as_ref() == Some(&Route::Docs) { "text-white" } else { "text-silver" }
                                        )}>
                                {"Documentation"}
                            </Link<Route>>
                        </span>
                    </div>
                </div>
            }
        </nav>
    }
}
