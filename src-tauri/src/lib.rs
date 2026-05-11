pub mod generator;
pub mod layout;
pub mod settings;

use settings::LayoutSettings;
use std::fs;
use std::path::Path;

#[tauri::command]
async fn convert_text_to_docx(file_path: String, settings: LayoutSettings) -> Result<(), String> {
    let content = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;

    let path = Path::new(&file_path);
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    let stem = path
        .file_stem()
        .ok_or("Invalid filename")?
        .to_str()
        .ok_or("Invalid filename")?;
    let output_path = parent.join(format!("{}.docx", stem));

    let docx = generator::create_docx(&content, &settings);

    let file = fs::File::create(output_path).map_err(|e| e.to_string())?;
    docx.build().pack(file).map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![convert_text_to_docx])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
