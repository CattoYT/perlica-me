use leptos::prelude::*;

use leptos::mount::mount_to_body;
use leptos_meta::*;

use crate::css_animations::FadeIn;
use crate::socials::SocialWidget;
mod body;
mod css_animations;
mod demo;
mod github_projects;
mod socials;
mod style;

fn main() {
    mount_to_body(AppRoot);
}

#[component]
fn AppRoot() -> impl IntoView {
    provide_meta_context();

    let (show_github, set_github_state) = signal(false);

    // animations

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
                <div>
                    <SocialWidget
                        platform="Youtube".to_string()
                        url="https://youtube.com/cattoyt".to_string()
                        visibility=None
                    />
                    <SocialWidget
                        platform="Twitch".to_string()
                        url="https://twitch.tv/vedal987".to_string()
                        visibility=None
                    />
                    <SocialWidget
                        platform="Github".to_string()
                        url="https://github.com/cattoyt".to_string()
                        visibility=None
                    />
                </div>

                // github projects, hopefully maybe extending out when github is pressed or something
                <FadeIn />
                <div style:opacity="0.0" class:fade-in=show_github>
                    <h3 style:color="white" style:font-size="24px" style:font-weight="bold">
                        "Github projects"
                    </h3>

                    {github_projects::github_project(
                        "Universal Comms Bot",
                        "https://github.com/cattoyt/universal_comms_bot",
                    )}
                </div>
            </div>

        </main>
    }
}
