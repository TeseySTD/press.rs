use crate::components::app::nav::Nav;
use crate::pages::docs::Docs;
use crate::pages::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/docs")]
    Docs,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Docs => html! { <Docs /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="min-h-screen bg-black transition-all duration-700">
                <Nav />
                <main class="animate-fade-in">
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}
