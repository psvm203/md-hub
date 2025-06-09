use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <img class={"w-64"} src={"images/markdown.png"} />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
