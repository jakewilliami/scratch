#![cfg(target_os = "macos")]

use dioxus::desktop::{
    self,
    tao::{
        event_loop::EventLoopBuilder,
        platform::macos::{ActivationPolicy, EventLoopExtMacOS},
    },
};
use objc2_app_kit::NSApplication;
use objc2_foundation::MainThreadMarker;

// Ensure the application runs in the background
pub fn configure_event_loop(cfg: desktop::Config) -> desktop::Config {
    let mut event_loop = EventLoopBuilder::with_user_event().build();
    event_loop.set_activation_policy(ActivationPolicy::Accessory);
    event_loop.set_dock_visibility(false);
    event_loop.set_activate_ignoring_other_apps(false);
    cfg.with_event_loop(event_loop)
}

// Restore focus to previous application
pub fn restore_focus() {
    unsafe {
        let mtm = MainThreadMarker::new_unchecked();
        let app = NSApplication::sharedApplication(mtm);
        app.hide(None);
    }
}
