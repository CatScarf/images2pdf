// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod pdf;

#[tauri::command]
fn convert(paths: Vec<String>, output: String, quality: u8) -> String{
    match convert_implement(paths, output, quality) {
        Ok(_) => "Success".to_string(),
        Err(e) => format!("Error: {}", e)
    }
}


fn convert_implement(paths: Vec<String>, output: String, quality: u8) -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = pdf::Pdf::new();
    doc.read_imgs(&paths);
    doc.create(output, quality)
}



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
