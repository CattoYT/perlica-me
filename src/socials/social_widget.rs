use std::f32::consts::E;

use leptos::{ev::MouseEvent, html::HtmlElement, prelude::*, tachys::view};

#[component]
pub fn SocialWidget(
    platform: String,
    url: String,
    visibility: Option<ReadSignal<bool>>,
) -> impl IntoView {
    view! {
        <a
            href=url
            target="_blank"
            style:display="block"
            style:text-decoration="none"
            style:text-decoration-line="none"
            style:color="inherit"
            style:visibility=move || match visibility {
                Some(visibility) => {
                    if visibility.get() { "visible".to_string() } else { "hidden".to_string() }
                }
                None => "true".to_string(),
            }
        >
            // make the things unclickable when invisible
            <div
                style:text-align="center"
                style:gap="12px"
                style:background="#171717"
                style:border="1px solid #2a2a2a"
                style:border-radius="12px"
                style:padding="16px 16px"
                style:margin="15px auto 15px auto"
                style:max-width="80%"
                style:webkit-box-shadow="0px 0px 13px 2px rgba(125,41,173,0.9)"
                style:moz-box-shadow="0px 0px 13px 2px rgba(125,41,173,0.9)"
                style:box-shadow="0px 0px 6px 2px rgba(125,41,173,0.44)"
                style:font-family="\"Montserrat\", sans-serif"
                style:font-weight="700"
            >

                // style:color="#eee"

                <span style:text-decoration="none" style:color="white">
                    {platform}
                </span>

            </div>
        </a>
    }
}
