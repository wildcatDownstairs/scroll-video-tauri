#[tauri::command]
async fn save_video(app: tauri::AppHandle, data: Vec<u8>) -> Result<String, String> {
    use tauri_plugin_dialog::DialogExt;

    let file_path = app
        .dialog()
        .file()
        .add_filter("WebM Video", &["webm"])
        .set_file_name("scroll-caption.webm")
        .blocking_save_file();

    match file_path {
        Some(path) => {
            let path_str = path.to_string();
            std::fs::write(&path_str, &data).map_err(|e| e.to_string())?;
            Ok(path_str)
        }
        None => Err("canceled".to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![save_video])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
