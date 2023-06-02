use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component]
pub fn Drawer(props: &Props) -> Html {
    html! {
            <aside id={"primary"}>
                {for props.children.iter()}
            </aside>
    }
}
