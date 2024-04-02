use stylist::{css, yew::styled_component, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub children: Children,
    pub id: AttrValue,
    pub label: AttrValue,
}

#[styled_component]
pub fn BBCollapse(props: &Props) -> Html {
    let button_style = Style::new(css! {
        width: 100%;
    })
    .unwrap();

    html! {
        <>
            <nav class="navbar navbar navbar-expand-lg">
              <div class="container-fluid">
                <button class={classes!("navbar-toggler", button_style.clone())} type="button" data-bs-toggle="collapse" data-bs-target="#navbarToggleExternalContent" aria-controls="navbarToggleExternalContent" aria-expanded="false" aria-label="Toggle navigation">
                  <span>{props.label.clone()}</span>
                </button>
                <div class="collapse navbar-collapse" id="navbarToggleExternalContent" data-bs-theme="dark">
                    {props.children.clone()}
                    <button class={classes!("navbar-toggler", button_style)} type="button" data-bs-toggle="collapse" data-bs-target="#navbarToggleExternalContent" aria-controls="navbarToggleExternalContent" aria-expanded="false" aria-label="Toggle navigation">
                      <span>{props.label.clone()}</span>
                    </button>
                </div>
              </div>
            </nav>
        </>
    }
}
