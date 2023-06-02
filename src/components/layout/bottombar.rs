use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component]
pub fn BottomBar(props: &Props) -> Html {
    html! {
        <footer>
        {for props.children.iter()}
        </footer>
    }
}
