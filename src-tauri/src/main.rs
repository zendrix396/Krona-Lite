// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, RunEvent, AppHandle, State};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use std::fs;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AppData {
    title: String,
    hotkey: String,
    todos: serde_json::Value,
}

struct AppState {
    current_hotkey: Mutex<String>,
    registered_shortcut: Mutex<Option<(Modifiers, Code)>>,
}

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

#[tauri::command]
fn update_hotkey(app: AppHandle, state: State<AppState>, new_hotkey: String) -> Result<(), String> {
    // Parse the new hotkey combination
    let (modifiers, code) = parse_hotkey_combination(&new_hotkey)
        .or_else(|| {
            // Fallback for single keys (legacy format)
            parse_hotkey_code(&new_hotkey).map(|c| (Modifiers::empty(), c))
        })
        .ok_or("Invalid hotkey combination")?;
    
    // Update the state
    {
        let mut current_hotkey = state.current_hotkey.lock().unwrap();
        *current_hotkey = new_hotkey.clone();
        
        let mut registered_shortcut = state.registered_shortcut.lock().unwrap();
        *registered_shortcut = Some((modifiers, code));
    }
    
    // Unregister all shortcuts first
    app.global_shortcut().unregister_all().map_err(|e| format!("Failed to unregister shortcuts: {}", e))?;
    
    // Register the new shortcut
    app.global_shortcut()
        .register(Shortcut::new(Some(modifiers), code))
        .map_err(|e| format!("Failed to register new hotkey: {}", e))?;
    
    Ok(())
}

fn parse_hotkey_code(hotkey: &str) -> Option<Code> {
    match hotkey {
        "End" => Some(Code::End),
        "Home" => Some(Code::Home),
        "Insert" => Some(Code::Insert),
        "Delete" => Some(Code::Delete),
        "F1" => Some(Code::F1),
        "F2" => Some(Code::F2),
        "F3" => Some(Code::F3),
        "F4" => Some(Code::F4),
        "F5" => Some(Code::F5),
        "F6" => Some(Code::F6),
        "F7" => Some(Code::F7),
        "F8" => Some(Code::F8),
        "F9" => Some(Code::F9),
        "F10" => Some(Code::F10),
        "F11" => Some(Code::F11),
        "F12" => Some(Code::F12),
        "Space" => Some(Code::Space),
        "Enter" => Some(Code::Enter),
        "Escape" => Some(Code::Escape),
        "Tab" => Some(Code::Tab),
        "Backspace" => Some(Code::Backspace),
        "KeyA" => Some(Code::KeyA),
        "KeyB" => Some(Code::KeyB),
        "KeyC" => Some(Code::KeyC),
        "KeyD" => Some(Code::KeyD),
        "KeyE" => Some(Code::KeyE),
        "KeyF" => Some(Code::KeyF),
        "KeyG" => Some(Code::KeyG),
        "KeyH" => Some(Code::KeyH),
        "KeyI" => Some(Code::KeyI),
        "KeyJ" => Some(Code::KeyJ),
        "KeyK" => Some(Code::KeyK),
        "KeyL" => Some(Code::KeyL),
        "KeyM" => Some(Code::KeyM),
        "KeyN" => Some(Code::KeyN),
        "KeyO" => Some(Code::KeyO),
        "KeyP" => Some(Code::KeyP),
        "KeyQ" => Some(Code::KeyQ),
        "KeyR" => Some(Code::KeyR),
        "KeyS" => Some(Code::KeyS),
        "KeyT" => Some(Code::KeyT),
        "KeyU" => Some(Code::KeyU),
        "KeyV" => Some(Code::KeyV),
        "KeyW" => Some(Code::KeyW),
        "KeyX" => Some(Code::KeyX),
        "KeyY" => Some(Code::KeyY),
        "KeyZ" => Some(Code::KeyZ),
        _ => None,
    }
}

fn parse_hotkey_combination(hotkey: &str) -> Option<(Modifiers, Code)> {
    let parts: Vec<&str> = hotkey.split('+').collect();
    if parts.is_empty() {
        return None;
    }

    let mut modifiers = Modifiers::empty();
    let mut main_key = None;

    for part in &parts {
        match *part {
            "Ctrl" => modifiers |= Modifiers::CONTROL,
            "Shift" => modifiers |= Modifiers::SHIFT,
            "Alt" => modifiers |= Modifiers::ALT,
            "Meta" => modifiers |= Modifiers::SUPER,
            key => {
                if main_key.is_none() {
                    main_key = parse_key_code(key);
                }
            }
        }
    }

    main_key.map(|code| (modifiers, code))
}

fn parse_key_code(key: &str) -> Option<Code> {
    match key {
        "End" => Some(Code::End),
        "Home" => Some(Code::Home),
        "Insert" => Some(Code::Insert),
        "Delete" => Some(Code::Delete),
        "F1" => Some(Code::F1),
        "F2" => Some(Code::F2),
        "F3" => Some(Code::F3),
        "F4" => Some(Code::F4),
        "F5" => Some(Code::F5),
        "F6" => Some(Code::F6),
        "F7" => Some(Code::F7),
        "F8" => Some(Code::F8),
        "F9" => Some(Code::F9),
        "F10" => Some(Code::F10),
        "F11" => Some(Code::F11),
        "F12" => Some(Code::F12),
        "Space" => Some(Code::Space),
        "Enter" => Some(Code::Enter),
        "Escape" => Some(Code::Escape),
        "Tab" => Some(Code::Tab),
        "Backspace" => Some(Code::Backspace),
        "PageUp" => Some(Code::PageUp),
        "PageDown" => Some(Code::PageDown),
        "ArrowUp" => Some(Code::ArrowUp),
        "ArrowDown" => Some(Code::ArrowDown),
        "ArrowLeft" => Some(Code::ArrowLeft),
        "ArrowRight" => Some(Code::ArrowRight),
        "KeyA" | "A" => Some(Code::KeyA),
        "KeyB" | "B" => Some(Code::KeyB),
        "KeyC" | "C" => Some(Code::KeyC),
        "KeyD" | "D" => Some(Code::KeyD),
        "KeyE" | "E" => Some(Code::KeyE),
        "KeyF" | "F" => Some(Code::KeyF),
        "KeyG" | "G" => Some(Code::KeyG),
        "KeyH" | "H" => Some(Code::KeyH),
        "KeyI" | "I" => Some(Code::KeyI),
        "KeyJ" | "J" => Some(Code::KeyJ),
        "KeyK" | "K" => Some(Code::KeyK),
        "KeyL" | "L" => Some(Code::KeyL),
        "KeyM" | "M" => Some(Code::KeyM),
        "KeyN" | "N" => Some(Code::KeyN),
        "KeyO" | "O" => Some(Code::KeyO),
        "KeyP" | "P" => Some(Code::KeyP),
        "KeyQ" | "Q" => Some(Code::KeyQ),
        "KeyR" | "R" => Some(Code::KeyR),
        "KeyS" | "S" => Some(Code::KeyS),
        "KeyT" | "T" => Some(Code::KeyT),
        "KeyU" | "U" => Some(Code::KeyU),
        "KeyV" | "V" => Some(Code::KeyV),
        "KeyW" | "W" => Some(Code::KeyW),
        "KeyX" | "X" => Some(Code::KeyX),
        "KeyY" | "Y" => Some(Code::KeyY),
        "KeyZ" | "Z" => Some(Code::KeyZ),
        "Digit1" | "1" => Some(Code::Digit1),
        "Digit2" | "2" => Some(Code::Digit2),
        "Digit3" | "3" => Some(Code::Digit3),
        "Digit4" | "4" => Some(Code::Digit4),
        "Digit5" | "5" => Some(Code::Digit5),
        "Digit6" | "6" => Some(Code::Digit6),
        "Digit7" | "7" => Some(Code::Digit7),
        "Digit8" | "8" => Some(Code::Digit8),
        "Digit9" | "9" => Some(Code::Digit9),
        "Digit0" | "0" => Some(Code::Digit0),
        _ => None,
    }
}

fn load_hotkey_from_file() -> String {
    let mut path = dirs::document_dir().unwrap_or_default();
    path.push("Krona");
    path.push("todos.json");
    
    if path.exists() {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(app_data) = serde_json::from_str::<AppData>(&content) {
                return app_data.hotkey;
            }
        }
    }
    
    "F4".to_string() // Default fallback
}

fn main() {
    let initial_hotkey = load_hotkey_from_file();
    
    // Parse initial hotkey
    let initial_shortcut = parse_hotkey_combination(&initial_hotkey)
        .or_else(|| parse_hotkey_code(&initial_hotkey).map(|c| (Modifiers::empty(), c)));
    
    let app_state = AppState {
        current_hotkey: Mutex::new(initial_hotkey.clone()),
        registered_shortcut: Mutex::new(initial_shortcut),
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if let (ShortcutState::Pressed, Some(window)) =
                        (event.state(), app.get_webview_window("main"))
                    {
                        // Check if the pressed shortcut matches the current registered hotkey
                        if let Ok(registered) = app.state::<AppState>().registered_shortcut.lock() {
                            if let Some((modifiers, code)) = *registered {
                                if shortcut.matches(modifiers, code) {
                                    if let Ok(true) = window.is_visible() {
                                        let _ = window.hide();
                                    } else {
                                        let _ = window.show();
                                        let _ = window.set_focus();
                                    }
                                }
                            }
                        }
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![save_todos, load_todos, update_hotkey])
        .setup(|app| {
            let hotkey = load_hotkey_from_file();
            let shortcut_combo = parse_hotkey_combination(&hotkey)
                .or_else(|| parse_hotkey_code(&hotkey).map(|c| (Modifiers::empty(), c)));
            
            if let Some((modifiers, code)) = shortcut_combo {
                app.global_shortcut()
                    .register(Shortcut::new(Some(modifiers), code))?;
            }
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
