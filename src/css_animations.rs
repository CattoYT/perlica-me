use leptos::prelude::*;
#[component]
pub fn fade_in_github_projects() -> impl IntoView {
    view! {
        <style>
            "
            .fade-in {
              --i: 0;
              opacity: 0;
              animation: fadeIn 0.5s ease-out forwards;
              animation-delay: calc(var(--i) * 0.1s);
            }
            @keyframes fadeIn {
                from { opacity: 0; transform: translateY(8px); }
                to   { opacity: 1; transform: translateY(0); }
            }
            
            
            
            "

        </style>
    }
}

//todo: migrate this to stylance, could be better icl

#[component]
pub fn slide_in_socials() -> impl IntoView {
    // thanks to copilot for this approach, almost forgot im in rust and not html+css lmao
    let max_items = 50;
    let staggered_css = (1..=max_items)
        .map(|i| {
            format!(
                ".cascade-on-load > *:nth-child({}) {{ animation-delay: {:.1}s; }}",
                i,
                i as f64 * 0.1
            )
        })
        .collect::<String>();

    view! {
        <style>
            "
            .cascade-on-load > * {
              opacity: 0;
              animation: slideIn 0.5s ease-out forwards;
            }
            @keyframes slideIn {
                from { opacity: 0; transform: translateX(-16px); }
                to   { opacity: 1; transform: translateX(0); }
            }
            
            
            
            " {staggered_css}
        </style>
    }
}
