use leptos::prelude::*;
use leptos_meta::*;

pub fn BodyMods() -> impl IntoView {
    provide_meta_context();
    view! {
        <title>"Azuyori.dev"</title>
        <Body
            style:background-color="#111"
            style:font-family="cursive"
            style:padding="16px 16px"
            style:margin="15px auto 15px auto"
            style:max-width="40%"
        />
    }
}
