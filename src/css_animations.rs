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
                to   { opacity: 1; transform: translateX(0px); }
            }
            
            
            
            " {staggered_css}
        </style>
    }
}

#[component]
pub fn slide_down_out() -> impl IntoView {
    view! {
        <style>
            "
            .slide-down-out {
                opacity: 1;
                animation: SlideDownOut 1s cubic-bezier(0.80, -0.35, 0.75, 0.89) forwards;
            }
            @keyframes SlideDownOut {
                from { opacity: 1; bottom: 24px; }
                to   { opacity: 0; bottom: -100px; }
            }
            
            
            
            "
        </style>
    }
}
