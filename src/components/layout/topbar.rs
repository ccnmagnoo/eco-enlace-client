use super::brand::Brand;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component]
pub fn TopBar(props: &Props) -> Html {
    html! {
        <>
            <Brand/>
            <nav>
                {for props.children.iter()}
            </nav>
        </>
    }
}
