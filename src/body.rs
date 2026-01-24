use leptos::prelude::*;
use leptos_meta::*;

pub fn BodyMods() -> impl IntoView {
    provide_meta_context();
    view! {
        <title>"Azuyori.dev"</title>
        <Body style:background-color="#111" />
    }
}
