use std::ops::Deref;

use quick_xml::{events::Event, Reader};
use serde::{Deserialize, Serialize};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize)]
struct Range {
    from: usize,
    to: usize,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum XmlNode {
    Text {
        #[serde(flatten)]
        range: Range,
        content: String,
    },
    Element {
        open: XmlOpenTag,
        content: Vec<XmlNode>,
        close: XmlCloseTag,
    },
    SelfClosingElement {
        tag: XmlOpenTag,
    },
}

#[derive(Serialize)]
struct XmlOpenTag {
    #[serde(flatten)]
    range: Range,
    name: String,
    attributes: Vec<XmlAttribute>,
}

#[derive(Serialize)]
struct XmlAttribute {
    key: String,
    value: String,
}

#[derive(Serialize)]
struct XmlCloseTag {
    #[serde(flatten)]
    range: Range,
}

#[tauri::command]
fn parse(xml: &str) -> Result<Vec<XmlNode>, ()> {
    let mut reader = Reader::from_str(xml);
    let mut pos_utf16 = 0;
    let mut pos_utf8 = 0;
    let mut open_tags = Vec::new();
    let mut contents = vec![Vec::new()];
    loop {
        match reader.read_event() {
            Ok(event) => {
                let to_utf8 = reader.buffer_position() as usize;
                let to_utf16 = pos_utf16 + xml[pos_utf8..to_utf8].encode_utf16().count();
                let range = Range {
                    from: pos_utf16,
                    to: to_utf16,
                };
                match event {
                    Event::Start(start) => {
                        let mut attributes = Vec::new();
                        for attr in start.attributes() {
                            let attr = attr.or(Err(()))?;
                            attributes.push(XmlAttribute {
                                key: String::from_utf8(attr.key.0.to_vec()).or(Err(()))?,
                                value: String::from_utf8(attr.value.to_vec()).or(Err(()))?,
                            });
                        }
                        open_tags.push(XmlOpenTag {
                            range,
                            name: String::from_utf8(start.name().0.to_vec()).or(Err(()))?,
                            attributes,
                        });
                        contents.push(Vec::new());
                    }
                    Event::End(_) => {
                        let tag = open_tags.pop().ok_or(())?;
                        let content = contents.pop().unwrap();
                        contents.last_mut().unwrap().push(XmlNode::Element {
                            open: tag,
                            content,
                            close: XmlCloseTag { range },
                        });
                    }
                    Event::Empty(empty) => {
                        let mut attributes = Vec::new();
                        for attr in empty.attributes() {
                            let attr = attr.or(Err(()))?;
                            attributes.push(XmlAttribute {
                                key: String::from_utf8(attr.key.0.to_vec()).or(Err(()))?,
                                value: String::from_utf8(attr.value.to_vec()).or(Err(()))?,
                            });
                        }
                        contents
                            .last_mut()
                            .unwrap()
                            .push(XmlNode::SelfClosingElement {
                                tag: XmlOpenTag {
                                    range,
                                    name: String::from_utf8(empty.name().0.to_vec()).or(Err(()))?,
                                    attributes,
                                },
                            });
                    }
                    Event::Text(text) => {
                        contents.last_mut().unwrap().push(XmlNode::Text {
                            range,
                            content: String::from_utf8(text.to_vec()).or(Err(()))?,
                        });
                    }
                    Event::Eof => {
                        return if contents.len() == 1 {
                            Ok(contents.pop().unwrap())
                        } else {
                            Err(())
                        }
                    }
                    _ => todo!(),
                }
                pos_utf8 = to_utf8;
                pos_utf16 = to_utf16;
            }
            Err(_) => return Err(()),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![parse])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
