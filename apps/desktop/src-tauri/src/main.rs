#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn security_boundary_version() -> &'static str {
    "foundation-v1"
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![security_boundary_version])
        .run(tauri::generate_context!())
        .expect("error while running SiteWarden Mail");
}
