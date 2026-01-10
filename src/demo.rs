use leptos::prelude::*;


#[component]
fn DemoApp() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div style:text-align="center">
        <button
            on:click=move |_| set_count.set(count.get() + 1)
                // set the `style` attribute

                // and toggle individual CSS properties with `style:`

                style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
                style:max-width="400px"
                // Set a CSS variable for stylesheet use
                style=("--columns", move || count.get().to_string())
        >

            "Click me: "
            {count}

        </button>
        <p>
            "Double count: "
            {move || count.get() * 2}
        </p>
        </div>
    }
}
