mod app;
mod titles;

use self::app::App;

pub fn start_prototype() {
    yew::Renderer::<App>::new().render();
}
