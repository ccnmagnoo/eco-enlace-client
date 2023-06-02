use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component]
pub fn Panel(props: &Props) -> Html {
    html! {
            <aside id={"secondary"}>
                {for props.children.iter()}
            </aside>
    }
}
