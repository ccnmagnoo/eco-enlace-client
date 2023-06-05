use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component]
pub fn BottomBar(props: &Props) -> Html {
    html! {
        <footer>
            <section>
                {for props.children.iter()}
            </section>
        <section class="auspices">
            <span>
                {"auspicio"}
            </span>
            <a href="https://www.facebook.com/EnergiaValpo/" target="_blank">
            <span>
                {"@energíaValpo"}
            </span>
            <img id="energia_valpo_logo" src="https://scontent.fscl3-1.fna.fbcdn.net/v/t39.30808-6/329003534_1492480671276258_2819404882572271341_n.jpg?_nc_cat=106&ccb=1-7&_nc_sid=09cbfe&_nc_ohc=knn0ryWA43sAX9tOvJi&_nc_ht=scontent.fscl3-1.fna&oh=00_AfD3NhwoM3lXiyflJAkwDaG34rFqCZQM4j95fGkWkaQsYg&oe=6483B2FC" alt="energia_logo" />
            </a>

        </section>
        </footer>
    }
}
