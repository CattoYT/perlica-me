use leptos::prelude::*;

use crate::css_animations;

fn github_project<'a>(project_name: &'a str, url: &'a str) -> impl IntoView {
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

#[component]
pub fn GithubProjects(show_github: ReadSignal<bool>) -> impl IntoView {
    view! {
            <Show
      when=move || { show_github.get() }
      fallback=|| view! { }
    >
          <css_animations::FadeInGithubProjects />
                  <div>
                      <h3
                          class:fade-in=show_github
                          style:opacity="0.0"
                          style:--i="0"
                          style:color="white"
                          style:font-size="24px"
                          style:font-weight="bold"
                      >

                          "Github projects (That i'm proud of)"
                      </h3>

                      <div class:fade-in=show_github style:opacity="0.0" style:--i="1">
                          {github_project(
                              "Universal Comms Bot",
                              "https://github.com/cattoyt/universal_comms_bot",
                          )}
                      </div>
                      <div class:fade-in=show_github style:opacity="0.0" style:--i="2">
                          {github_project(
                              "Azuyori.dev (This website!)",
                              "https://github.com/cattoyt/azuyori-dev",
                          )}

                      </div>
                  </div>
                </Show>
                  }
}
