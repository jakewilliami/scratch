use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::prelude::*;

const WINDOW_TITLE: &str = "scratch";
const INIT_WINDOW_W: u16 = 400;
const INIT_WINDOW_H: u16 = 100;
const STYLE: Asset = asset!("/assets/style.css");
const JULIAMONO: Asset = asset!("/assets/fonts/julia-mono.woff2");

fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                // github.com/DioxusLabs/dioxus/blob/c64415c0/examples/01-app-demos/calculator.rs#L23-L25
                WindowBuilder::new()
                    .with_title(WINDOW_TITLE)
                    .with_inner_size(LogicalSize::new(INIT_WINDOW_W, INIT_WINDOW_H))
                    .with_always_on_top(true),
            ),
        )
        .launch(app);
}

fn app() -> Element {
    let mut text = use_signal(String::new);
    let window = dioxus::desktop::use_window();

    let handle_keydown = move |evt: KeyboardEvent| {
        if evt.key() == Key::Escape {
            on_close(&text.read());
            window.close();
        }
    };

    rsx! {
        // github.com/DioxusLabs/dioxus/blob/c64415c0/examples/01-app-demos/todomvc.rs#L66
        Stylesheet { href: STYLE }
        // github.com/DioxusLabs/dioxus/blob/c64415c0/examples/03-assets-styling/fonts.rs
        style { "@font-face {{ font-family: 'JuliaMono'; src: url('{JULIAMONO}') format('truetype'); font-display: swap; }}" }
        div {
            class: "container",
            onkeydown: handle_keydown,
            textarea {
                class: "editor",
                autofocus: "true",
                value: "{text}",
                oninput: move |evt| text.set(evt.value()),
            }
        }
    }
}

fn on_close(text: &str) {
    // TODO: clipboard
    println!("{}", text);
}
