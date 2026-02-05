use leptos::prelude::*;

use crate::{css_animations::SlideInSpotify, socials::github_projects, FadeInGithubProjects};

#[component]
pub fn SpotifyTracks(show_spotify_playlist: ReadSignal<bool>) -> impl IntoView {
    view! {

    <SlideInSpotify /> //reuse cuz i mean it works right XD
            <div class:cascade-slide-in-spotify=show_spotify_playlist>
                <h3
                    // class:fade-in=show_spotify_playlist
                    style:opacity="0.0"
                    style:--i="0"
                    style:color="white"
                    style:font-size="24px"
                    style:font-weight="bold"
                >

                    "My spotify playlists"
                </h3>

                <div
                // class:fade-in=show_spotify_playlist
                style:opacity="0.0" style:--i="1">
                    {spotify_widget(
                        "Relax",
                        "https://open.spotify.com/playlist/0y0qg8RpeWIZEXOY6zNlHP",


                    )}
                </div>
                                <div
                // class:fade-in=show_spotify_playlist
                style:opacity="0.0" style:--i="2">
                    {spotify_widget(
                        "Bebision",
                        "https://open.spotify.com/playlist/2pD4F2gaDVdPWkS5P7NEo4",
                    )}

                </div>
            </div>
            }
}

fn spotify_widget<'a>(project_name: &'a str, url: &'a str) -> impl IntoView {
    view! {
        <a href=url target="_blank">
            <div
                style:text-align="center"

                style:gap="12px"
                style:background="#171717"
                style:border="1px solid #2a2a2a"
                style:border-radius="12px"
                style:padding="14px 16px"
                style:margin="15px auto 15px auto"
                style:max-width="60%"
                style:webkit-box-shadow="0px 0px 13px 2px rgba(125,41,173,0.9)"
                style:moz-box-shadow="0px 0px 13px 2px rgba(125,41,173,0.9)"
                style:box-shadow="0px 0px 6px 2px rgba(125,41,173,0.44)"
                style:font-family="\"Montserrat\", sans-serif"
                style:font-weight="700"

                style:text-decoration="none"
                style:color="white"
            >
                // style:color="#eee"
                {project_name}

            </div>
        </a>
    }
}
