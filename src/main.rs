use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use leptos::attr::onload;
use leptos::ev::{self, on, PageTransitionEvent};
use leptos::html::{button, Portal};
use leptos::leptos_dom::logging::console_debug_log;
use leptos::prelude::*;

use crate::css_animations::{FadeInGithubProjects, SlideDownOut, SlideInSocials};
use crate::socials::github_projects::GithubProjects;
use crate::socials::social_widget::SocialWidget;
use crate::socials::spotify::SpotifyTracks;
use leptos::mount::mount_to_body;
use leptos_meta::*;
use quad_rand as qrand;
mod body;
mod css_animations;
mod demo;
mod socials;

fn main() {
    mount_to_body(AppRoot);
}

#[derive(Clone)]
enum ButtonVisibilityStatus {
    Shown,
    Hidden,
}

impl ButtonVisibilityStatus {
    fn get_opposite(thing: &ButtonVisibilityStatus) -> ButtonVisibilityStatus {
        match thing {
            ButtonVisibilityStatus::Hidden => ButtonVisibilityStatus::Shown,
            ButtonVisibilityStatus::Shown => ButtonVisibilityStatus::Hidden,
        }
    }
}

#[component]
fn AppRoot() -> impl IntoView {
    provide_meta_context();
    let (show_github, set_github_state) = signal(false);
    let (show_github_button_animate, start_github_button_animate) = signal(false);
    let (show_spotify, set_spotify_state) = signal(false);

    let (on_load, set_load) = signal(true);
    window_event_listener(ev::load, move |_| {
        &set_load.set(true);
    });

    let titles = vec![
        "Failing to write rust since 2025",
        "Perlica is really cute",
        "Perlica x chen when",
        "By Hikari/Kaeniya/CattoYT (I have too many names)",
        "Totally not a paste of vedal.ai",
        "Leptos made this take so long",
        "I really should have just used astro for this site",
        "Hi hack club reviewer!",
    ];
    qrand::srand(web_time::UNIX_EPOCH.elapsed().unwrap().as_nanos() as u64);

    let index: usize = qrand::gen_range(0, titles.len());
    let title = titles
        .get(index)
        .unwrap_or(&"Failing to write rust since 2025ssa");
    console_debug_log(*title);
    console_debug_log(&format!("{index}"));
    let (spotify_button_visibility, set_spotify_button_visibility) =
        signal(ButtonVisibilityStatus::Shown);

    // animations

    view! {
        <SlideInSocials />
        <main
            style:font-family="\"Montserrat\", sans-serif"
            style:font-weight="700"

            style:padding="16px 16px"
            style:margin="15px auto 15px auto"
            style:max-width="40%"
        >
            <div
                style="
                position: fixed;
                bottom: 5%;
                left: 50%;
                transform: translateX(-50%);
                z-index: -1;
                "
                class:cascade-on-load=on_load
            >
                <img
                    src="/public/images/perlica.png"
                    width="60%"
                    style="display: block; margin: auto;"
                    alt="Test Image"
                />
            </div>
            <body::BodyMods />
            <div style:text-align="center" class:cascade-on-load=on_load>

                <div style:color="white">
                    <h1

                        style:font-family="\"Montserrat\", sans-serif"

                        style:font-weight="900"
                        style:font-size="50px"
                    >
                        "Perlica.me"
                    </h1>
                    {title.to_owned()}
                </div>
                // <a href="https://youtube.com/@cattoyt">real</a>

                // social widgets
                {move || {
                    view! {
                        <SlideDownOut />
                        <div
                            style:position="fixed"
                            style:bottom="24px"
                            // style:left="-10%"
                            style:left="40%"
                            style:display="flex"
                            style:justify-content="center"
                            style:opacity="1"
                            style:z-index="9999"
                            style:max-width="fit-content"
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
                                style:font-family="\"Montserrat\", sans-serif"

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
                            style:right="40%"
                            // style:right="0"
                            style:display="flex"
                            style:justify-content="center"

                            style:opacity="1"
                            style:z-index="9999"
                            style:max-width="fit-content"
                            class:slide-down-out=show_spotify
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
                                style:font-family="\"Montserrat\", sans-serif"

                                style:color="White"
                                style:font-weight="800"

                                on:click=move |_| {
                                    console_debug_log("should have written");
                                    let spotifyono = !show_spotify.read().clone();
                                    *set_spotify_state.write() = spotifyono;
                                }
                            >

                                {match spotify_button_visibility.get() {
                                    ButtonVisibilityStatus::Hidden => "Hide Spotify",
                                    ButtonVisibilityStatus::Shown => "Show Spotify",
                                }}
                            // {match show_spotify.get() {
                            // true => "Hide Spotify",
                            // false => "Show Spotify",
                            // }}
                            </button>
                        </div>
                    }
                }}
                <SocialWidget
                    platform="Github".to_string()
                    url="https://github.com/cattoyt".to_string()
                    visibility=None
                />
                <SocialWidget
                    platform="Youtube".to_string()
                    url="https://youtube.com/cattoyt".to_string()
                    visibility=None
                />
                <SocialWidget
                    platform="Slack (HC)".to_string()
                    url="https://hackclub.enterprise.slack.com/team/U0786TENDM5".to_string()
                    visibility=None
                />
                <SocialWidget
                    platform="Twitter".to_string()
                    url="https://x.com/kaeniya_iUA".to_string()
                    visibility=None
                />

                <GithubProjects show_github=show_github />
            </div>

        </main>
        // SPotify overlay
        <Show when=move || { show_spotify.get() }>

            <div
                style:font-family="\"Montserrat\", sans-serif"
                style:font-weight="700"
                style:position="fixed"
                style:right="3%"
                style:bottom="5%"
            >
                <SpotifyTracks show_spotify_playlist=show_spotify />
            </div>

        </Show>
    }
}
