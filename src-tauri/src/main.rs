// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, RunEvent};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use std::fs;

#[tauri::command]
fn save_todos(todos: String) -> Result<(), String> {
    let mut path = dirs::document_dir().ok_or("Could not find documents directory")?;
    path.push("Krona");
    
    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| format!("Failed to create directory: {}", e))?;
    }
    
    path.push("todos.json");
    fs::write(path, todos).map_err(|e| format!("Failed to write file: {}", e))?;
    Ok(())
}

#[tauri::command]
fn load_todos() -> Result<String, String> {
    let mut path = dirs::document_dir().ok_or("Could not find documents directory")?;
    path.push("Krona");
    path.push("todos.json");
    
    if path.exists() {
        fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))
    } else {
        Ok(String::new())
    }
}

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
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![save_todos, load_todos])
        .setup(|app| {
            app.global_shortcut()
                .register(Shortcut::new(None, Code::End))?;
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| {
            if let RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });
}
