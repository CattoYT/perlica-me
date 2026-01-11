use leptos::prelude::*;

use leptos::mount::mount_to_body;
use leptos_meta::*;

use crate::socials::SocialWidget;
mod body;
mod demo;
mod socials;
mod style;

fn main() {
    mount_to_body(AppRoot);
}

#[component]
fn AppRoot() -> impl IntoView {
    provide_meta_context();

    let (show_github, set_github_state) = signal(false);

    view! {
        <main>
            <body::BodyMods />
            <div style:text-align="center">

                <div style:color="white">
                    <h1>"Azuyori.dev"</h1>
                </div>
                // <a href="https://youtube.com/@cattoyt">real</a>

                <button on:click=move |_| {
                    *set_github_state.write() = true
                }>"Show Github"</button>
                // social widgets
                <SocialWidget
                    platform="Youtube".to_string()
                    url="https://youtube.com/cattoyt".to_string()
                    visibility=None
                />
                <SocialWidget
                    platform="Twitch".to_string()
                    // YAY
                    url="https://twitch.tv/vedal987".to_string()
                    visibility=None
                />
                <SocialWidget
                    platform="Github".to_string()
                    // YAY
                    url="https://github.com/cattoyt".to_string()
                    visibility=None
                />

                <SocialWidget
                    platform="Github Projects".to_string()
                    // YAY
                    url="https://github.com/cattoyt".to_string()
                    visibility=Some(show_github)
                />

            </div>

        </main>
    }
}
