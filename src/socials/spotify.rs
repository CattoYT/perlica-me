use leptos::prelude::*;

use crate::{socials::github_projects, FadeInGithubProjects};

#[component]
fn SpotifyTracks(show_spotify_playlist: ReadSignal<bool>) -> impl IntoView {
    view! {
            <Show
      when=move || { show_spotify_playlist.get() }
      fallback=|| view! { }
    >
          <FadeInGithubProjects /> //reuse cuz i mean it works right XD
                  <div>
                      <h3
                          class:fade-in=show_spotify_playlist
                          style:opacity="0.0"
                          style:--i="0"
                          style:color="white"
                          style:font-size="24px"
                          style:font-weight="bold"
                      >

                          "Github projects (That i'm proud of)"
                      </h3>

                      <div class:fade-in=show_spotify_playlist style:opacity="0.0" style:--i="1">
                          {spotify_widget(
                              "Relax",
                              "https://open.spotify.com/playlist/0y0qg8RpeWIZEXOY6zNlHP",
                          )}
                      </div>
                      <div class:fade-in=show_spotify_playlist style:opacity="0.0" style:--i="2">
                          {spotify_widget(
                              "Azuyori.dev (This website!)",
                              "https://github.com/cattoyt/azuyori-dev",
                          )}

                      </div>
                  </div>
                </Show>
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
                style:font-family="sans-serif"
                style:font-weight="600"
                style:text-decoration="none"
                style:color="white"
            >
                // style:color="#eee"
                {project_name}

            </div>
        </a>
    }
}
