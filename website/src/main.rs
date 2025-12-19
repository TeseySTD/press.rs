mod app;
mod components;
mod pages;
mod utils;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
