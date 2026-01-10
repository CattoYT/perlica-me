
use leptos::html::{Div, Html, HtmlElement, Title};
use leptos::prelude::*;

use leptos::mount::mount_to_body;
mod demo;
fn main() {
    mount_to_body(AppRoot);
}

#[component]
fn AppRoot() -> impl IntoView {
    
    
    view! {
        <div style:text-align="centre">
            <title>
            
            "Azuyori.dev"
            
            </title>
        </div>
    }
}

