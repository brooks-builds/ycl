use self::app::App;

mod app;

pub fn start_prototype() {
    yew::Renderer::<App>::new().render();
}
