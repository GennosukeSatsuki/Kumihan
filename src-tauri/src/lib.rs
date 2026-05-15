use std::fs::{self, File};
use std::io::{Read, Write, Cursor};
use zip::write::FileOptions;
use walkdir::WalkDir;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn process_file(txt_path: String) -> Result<String, String> {
    let txt_content = fs::read_to_string(&txt_path)
        .map_err(|e| format!("TXTファイルの読み込みに失敗しました: {}", e))?;

    // TODO: テンプレートファイルの場所を決定する。
    // 今回は実行ファイルと同じディレクトリの template.docx を探すか、
    // あるいは組み込みリソースから出す。
    // ここではデモ用に、txt_path と同じディレクトリに output.docx を作る想定。
    
    let exe_path = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe_dir = exe_path.parent().ok_or("実行ファイルのパス取得に失敗しました")?;
    let template_path = exe_dir.join("template.docx");
    
    if !template_path.exists() {
        return Err(format!("テンプレートファイルが見つかりません: {}\nアプリと同じフォルダに template.docx を配置してください。", template_path.display()));
    }

    let file = File::open(&template_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;

    let mut new_archive_data = Vec::new();
    {
        let mut writer = zip::ZipWriter::new(Cursor::new(&mut new_archive_data));

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
            let name = file.name().to_string();
            
            let options = FileOptions::default()
                .compression_method(file.compression())
                .unix_permissions(file.unix_mode().unwrap_or(0o755));

            writer.start_file(name.clone(), options).map_err(|e| e.to_string())?;

            if name == "word/document.xml" {
                let mut content = String::new();
                file.read_to_string(&mut content).map_err(|e| e.to_string())?;
                
                // プレースホルダー {{CONTENT}} を置換
                // 簡易的な実装: XMLエスケープ処理が必要
                let escaped_txt = txt_content
                    .replace("&", "&amp;")
                    .replace("<", "&lt;")
                    .replace(">", "&gt;")
                    .replace("\n", "<w:br/>");
                
                let new_content = content.replace("{{CONTENT}}", &escaped_txt);
                writer.write_all(new_content.as_bytes()).map_err(|e| e.to_string())?;
            } else {
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
                writer.write_all(&buffer).map_err(|e| e.to_string())?;
            }
        }
        writer.finish().map_err(|e| e.to_string())?;
    }

    let output_path = txt_path.replace(".txt", "_converted.docx");
    fs::write(&output_path, new_archive_data).map_err(|e| e.to_string())?;

    Ok(format!("変換が完了しました: {}", output_path))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![process_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
