use copypasta_ext::{prelude::*, x11_fork::ClipboardContext};
use dioxus::{
    desktop::{self, Config, HotKeyState, LogicalSize, WindowBuilder, tao::dpi::PhysicalPosition},
    prelude::*,
};
use std::env;

const WINDOW_TITLE: &str = "scratch";
const INIT_WINDOW_W: u16 = 400;
const INIT_WINDOW_H: u16 = 100;
const STYLE: Asset = asset!("/assets/style.css");
const JULIAMONO: Asset = asset!("/assets/fonts/julia-mono.woff2");
const SHORTCUT: &str = "cmd+quote";

fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title(WINDOW_TITLE)
                    .with_inner_size(LogicalSize::new(INIT_WINDOW_W, INIT_WINDOW_H))
                    .with_always_on_top(true)
                    .with_visible(false)
                    .with_position(PhysicalPosition::new(0, 0)),
            ),
        )
        .launch(app);
}

fn app() -> Element {
    let mut text = use_signal(String::new);
    let mut show = use_signal(|| false);

    // Set shortcut to show the window
    _ = desktop::use_global_shortcut(SHORTCUT, move |state| {
        if state == HotKeyState::Pressed {
            show.toggle();
        }
    });

    // Handle focus
    let window = desktop::use_window();
    use_effect(move || {
        if show() {
            window.set_visible(true);
            window.set_focus();
        } else {
            window.set_visible(false);
        }
    });

    // When escape is pressed, close window (copy to clipboard and clear buffer)
    let handle_keydown = move |evt: KeyboardEvent| {
        if evt.key() == Key::Escape {
            on_close(&text.read());
            text.set(String::new());
            show.set(false);
        }
    };

    // Primary app template
    rsx! {
        Stylesheet { href: STYLE }
        style { "@font-face {{ font-family: 'JuliaMono'; src: url('{JULIAMONO}') format('truetype'); font-display: swap; }}" }
        div {
            class: "container",
            onkeydown: handle_keydown,
            textarea {
                class: "editor",
                autofocus: "true",
                autocorrect: "off",
                autocapitalize: "off",
                spellcheck: "false",
                value: "{text}",
                oninput: move |evt| text.set(evt.value()),
            }
        }
    }
}

// Stolen from github.com/jakewilliami/cb/blob/e2506051/src/main.rs#L124-L156
fn on_close(s: &str) {
    // Try set clipboard for WSL or SSH first, falling back to `clipboard` if unavailable
    let set_res = clipboard_anywhere::set_clipboard(s);
    let get_res = clipboard_anywhere::get_clipboard();

    // Possible errors:
    //   1. Something has gone wrong if we can neither set nor get the clipboard
    let clipboard_unresponsive = set_res.is_err() && get_res.is_err();
    //   2. If we are not using SSH, get_res should be okay
    let local_clipboard_get_err = env::var("SSH_CLIENT").is_err() && get_res.is_err();
    //   3. We might be able to get the result from clipboard but it could be empty
    let clipboard_not_populated = get_res.is_ok() && get_res.unwrap().is_empty();

    // Clipboard should be populated, but if any of the above edge cases are true,
    // then we need additional handling for possible errors or a final attempt
    // at setting the clipboard.
    if clipboard_unresponsive || local_clipboard_get_err || clipboard_not_populated {
        // If the clipboard is empty, then we failed to set the clipboard using
        // clipboard_anywhere; as such, let's try setting the clipboard using an
        // X11-aware clipboard manager
        let result = std::panic::catch_unwind(|| {
            let mut ctx = ClipboardContext::new().unwrap();
            ctx.set_contents(s.to_string())
                .expect("Failed to set contents of clipboard");
        });

        if result.is_err() {
            eprintln!("Text could not be copied to clipboard");
        }
    }
}
