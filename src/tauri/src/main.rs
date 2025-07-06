use tauri::{Manager, RunEvent};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if let (ShortcutState::Pressed, Some(window)) =
                        (event.state(), app.get_webview_window("main"))
                    {
                        if shortcut.matches(Modifiers::empty(), Code::End) {
                            if let Ok(true) = window.is_visible() {
                                let _ = window.hide();
                            } else {
                                // ... existing code ...
                            }
                        }
                    }
                })
        )
} 