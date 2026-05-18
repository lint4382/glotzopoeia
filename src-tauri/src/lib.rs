use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    id: String,
    data: BlockData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum BlockData {
    Gloss { cols: Vec<GlossColumn> },
    Paragraph(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GlossColumn {
    id: String,
    name: String,
    content: GlossColumnType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum GlossColumnType {
    Rows(Vec<String>),
    FreeForm(String),
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn save(data: Vec<Block>) {
    dbg!(serde_xml_rs::to_string(&data[0]));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dbg!(serde_json::from_str::<Vec<Block>>(
        r#"[
        {
            "id": "test",
            "data": {
                "type": "Gloss",
                "value": {
                    "cols": [
                        {
                            "id": "source",
                            "name": "Source",
                            "content": {
                                "type": "FreeForm",
                                "value": "Pra ka vajni."
                            }
                        },
                        {
                            "id": "words",
                            "name": "Words",
                            "content": {
                                "type": "Rows",
                                "value": ["pra", "ka", "vajni"]
                            }
                        }
                    ]
                }
            }
        }
    ]"#,
    ));
    dbg!(serde_json::to_string(&GlossColumn {
        id: "a".to_string(),
        name: "b".to_string(),
        content: GlossColumnType::Rows(Vec::new())
    }));
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
