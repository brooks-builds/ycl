mod app;
mod pages;
mod router;

use app::App;

pub fn start_prototype() {
    yew::Renderer::<App>::new().render();
}
