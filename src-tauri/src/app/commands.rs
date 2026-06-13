use crate::core;

#[tauri::command]
pub fn get_core_summary() -> String {
    core::summary().to_string()
}
