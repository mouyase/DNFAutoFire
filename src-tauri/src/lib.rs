mod core;


#[tauri::command]
async fn start_auto_fire() -> Result<(), String> {
    println!("start_auto_fire");
    Ok(())
}

#[tauri::command]
async fn stop_auto_fire() -> Result<(), String> {
    println!("stop_auto_fire");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_auto_fire, stop_auto_fire])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
