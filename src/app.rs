use crate::components::layout::*;
use bottombar::BottomBar;
use drawer::Drawer;
use panel::Panel;
use topbar::TopBar;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
    <>
    <header>
        <TopBar>
            {"navbar"}
        </TopBar>
    </header>

    <div class="central-container">
        <Drawer>
            {"left section"}
        </Drawer>
        <main>

            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            <code>{"let data:i64 = 10"}</code>

        </main>
        <Panel> {"right section"}</Panel>
    </div>

    <BottomBar>
        <span>{"item"}</span>
        <span>{"item"}</span>
        <span>{"item"}</span>
    </BottomBar>

    </>
    }
}
