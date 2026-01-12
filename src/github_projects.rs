use leptos::prelude::*;

pub fn github_project<'a>(project_name: &'a str, url: &'a str) -> impl IntoView {
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
