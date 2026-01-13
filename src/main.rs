use leptos::attr::onload;
use leptos::ev::{self, on, PageTransitionEvent};
use leptos::prelude::*;

use leptos::mount::mount_to_body;
use leptos_meta::*;

use crate::css_animations::{FadeInGithubProjects, SlideInSocials};
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
    let (on_load, set_load) = signal(true);
    window_event_listener(ev::load, move |_| {
        &set_load.set(true);
    });

    // animations

    view! {
        <SlideInSocials/>
        <main >
            <body::BodyMods />
            <div style:text-align="center" class:cascade-on-load=on_load>

                <div style:color="white">
                    <h1>"Azuyori.dev"</h1>
                </div>
                // <a href="https://youtube.com/@cattoyt">real</a>


                // TODO: change this to something a little more stylist
                <button on:click=move |_| {
                    *set_github_state.write() = true
                }>"Show Github Projects"</button>
                // social widgets


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


                // github projects, hopefully maybe extending out when github is pressed or something
                // TODO: make this unclickable when not rendered, or only load when button is pressed
                <FadeInGithubProjects />
                <div>
                    <h3
                        class:fade-in=show_github
                        style:opacity="0.0"
                        style:--i="0"
                        style:color="white"
                        style:font-size="24px"
                        style:font-weight="bold"
                    >

                        "Github projects"
                    </h3>

                    <div class:fade-in=show_github style:opacity="0.0" style:--i="1">
                        {github_projects::github_project(
                            "Universal Comms Bot",
                            "https://github.com/cattoyt/universal_comms_bot",
                        )}
                    </div>
                    <div class:fade-in=show_github style:opacity="0.0" style:--i="2">
                        {github_projects::github_project(
                            "Azuyori.dev",
                            "https://github.com/cattoyt/azuyori-dev",
                        )}

                    </div>
                </div>
            </div>

        </main>
    }
}
