use leptos::prelude::*;
#[component]
pub fn fade_in() -> impl IntoView {
    view! {
        // TODO: rewrite this into an actual css file cuz i dont get syntax highlighting lol; a solid part of this is ctrl c
        <style>
            "
            .fade-in {
                animation: fadeIn 0.5s ease-out forwards;
            
            }
            .fade-in > div:nth-child(n) {
                animation-delay: calc((var(--i, 1) - 1) * 0.05s);
            
            }
            @keyframes fadeIn {
                from { opacity: 0; transform: translateY(8px); }
                to   { opacity: 1; transform: translateY(0); }
            }
            "

        // TODO: do like a cascading thing with the animation as well
        </style>
    }
}
