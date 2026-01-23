use leptos::attr::onload;
use leptos::ev::{self, on, PageTransitionEvent};
use leptos::html::{button, Portal};
use leptos::leptos_dom::logging::console_debug_log;
use leptos::prelude::*;

use leptos::mount::mount_to_body;
use leptos_meta::*;

use crate::css_animations::{FadeInGithubProjects, SlideDownOut, SlideInSocials};
use crate::socials::github_projects::GithubProjects;
use crate::socials::social_widget::SocialWidget;
mod body;
mod css_animations;
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
    let (show_github_button_animate, start_github_button_animate) = signal(false);

    let (on_load, set_load) = signal(true);
    window_event_listener(ev::load, move |_| {
        &set_load.set(true);
    });

    // animations

    view! {
        <SlideInSocials />
        <main>

            <body::BodyMods />
            <div style:text-align="center" class:cascade-on-load=on_load>

                <div style:color="white">
                    <h1>"Azuyori.dev"</h1>
                </div>
                // <a href="https://youtube.com/@cattoyt">real</a>

                // social widgets
                {move || {
                    // (!show_github.get())
                    // .then(|| {
                    view! {
                        <SlideDownOut />
                        <div
                            style:position="fixed"
                            style:bottom="24px"
                            style:left="-10%"
                            style:right="0"
                            style:display="flex"
                            style:justify-content="center"
                            style:opacity="1"
                            style:z-index="9999"
                            style:max-width="auto"
                            class:slide-down-out=show_github_button_animate
                        >

                            <button
                                style:text-align="center"
                                style:background="#171717"
                                style:border="1px solid #2a2a2a"
                                style:border-radius="12px"
                                style:margin="15px auto 15px auto"
                                style:max-width="80%"
                                style:webkit-box-shadow="0px 0px 13px 2px rgba(125,41,173,0.9)"
                                style:moz-box-shadow="0px 0px 13px 2px rgba(125,41,173,0.9)"
                                style:box-shadow="0px 0px 6px 2px rgba(125,41,173,0.44)"
                                style:font-family="sans-serif"
                                style:color="White"
                                style:font-weight="800"

                                on:click=move |_| {
                                    console_debug_log("should have written");
                                    *set_github_state.write() = true;
                                    *start_github_button_animate.write() = true;
                                }
                            >
                                "Show Github Projects"
                            </button>
                        </div>

                        // TODO: change div sizes so that they dont overlap
                        <div
                            style:position="fixed"
                            style:bottom="24px"
                            style:left="10%"
                            style:right="0"
                            style:display="flex"
                            style:justify-content="center"
                            style:opacity="1"
                            style:z-index="9999"
                            style:max-width="auto"
                            class:slide-down-out=show_github_button_animate
                        >

                            <button
                                style:text-align="center"
                                style:background="#171717"
                                style:border="1px solid #2a2a2a"
                                style:border-radius="12px"
                                style:margin="15px auto 15px auto"
                                style:max-width="80%"
                                style:webkit-box-shadow="0px 0px 13px 2px rgba(125,41,173,0.9)"
                                style:moz-box-shadow="0px 0px 13px 2px rgba(125,41,173,0.9)"
                                style:box-shadow="0px 0px 6px 2px rgba(125,41,173,0.44)"
                                style:font-family="sans-serif"
                                style:color="White"
                                style:font-weight="800"

                                on:click=move |_| {
                                    console_debug_log("should have written");
                                    *set_github_state.write() = true;
                                    *start_github_button_animate.write() = true;
                                }
                            >
                                "Show Spotify"
                            </button>
                        </div>
                    }
                }}
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

                <GithubProjects show_github=show_github />
            </div>

        </main>
    }
}
