use serde::Serialize;
use uuid::Uuid;

use crate::{
    guid_converter::guid_definition::{GuidInput, GuidResult},
    json_finder::json_finder_helper::JsonData,
};
mod guid_converter;
mod json_finder;

#[derive(Serialize)]
struct BaseApiReturn<T> {
    is_successful: bool,
    message: Option<String>,
    payload: Option<T>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn generate_guid() -> BaseApiReturn<GuidResult> {
    let new_guid = format!("{}", Uuid::new_v4());
    let mut my_guid = GuidInput::new(&new_guid);
    match my_guid.check_guid() {
        true => {
            let gr = GuidResult::new(&my_guid);
            BaseApiReturn {
                is_successful: true,
                message: None,
                payload: Some(gr),
            }
        }
        false => BaseApiReturn {
            is_successful: false,
            message: Some(String::from("Error!")),
            payload: None,
        },
    }
}

#[tauri::command]
fn convert_guid(guid_input: &str) -> BaseApiReturn<GuidResult> {
    let mut my_guid = GuidInput::new(guid_input);
    match my_guid.check_guid() {
        true => {
            let gr = GuidResult::new(&my_guid);
            BaseApiReturn {
                is_successful: true,
                message: None,
                payload: Some(gr),
            }
        }
        false => BaseApiReturn {
            is_successful: false,
            message: Some(String::from("Error!")),
            payload: None,
        },
    }
}

#[tauri::command]
fn parse_and_find_json(input_key: Option<&str>, input_json: &str) -> BaseApiReturn<String> {
    let json_result = JsonData::new(input_json);
    // let v_result: Result<Value, Error> = serde_json::from_str(input_json);
    match json_result {
        Ok(json_data) => {
            let _ = json_data.parse();
            BaseApiReturn {
                is_successful: true,
                message: None,
                payload: Some(format!("{:?}", json_data.value)),
            }
        }
        Err(error) => BaseApiReturn {
            is_successful: false,
            message: Some(error.to_string()),
            payload: None,
        },
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            generate_guid,
            convert_guid,
            parse_and_find_json
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
