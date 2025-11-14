use std::error::Error;

use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug)]
enum ValueType {
    Array,
    Object,
    Str,
    Null,
}

#[derive(Serialize)]
pub struct JsonData {
    pub value: Value,
}

impl JsonData {
    pub fn new(input: &str) -> Result<Self, Box<dyn Error>> {
        let v_result: Value = serde_json::from_str(input)?;
        Ok(JsonData { value: v_result })
    }

    pub fn parse(&self) -> Result<(), Box<dyn Error>> {
        let mut counter = 1;
        for (key, value) in self.value.as_object().unwrap() {
            key_traversal(key, value, None);
            counter = counter + 1;
        }
        Ok(())
    }
}

pub fn key_traversal(key: &str, value: &Value, level: Option<i32>) {
    // 1. this value is an array loop this
    // 2. this value is an object loop the keys
    // 3. this value is str, get the value
    // 4. this value is null, get the value

    let curr_level = level.unwrap_or_else(|| 1);

    match get_value_type(value) {
        ValueType::Array => {
            println!("level: {} => {:#?}: Array", curr_level, key);
            for item in value.as_array().unwrap() {
                let mut ii = 0;
                if item["Key"] != json!(null) {
                    key_traversal(
                        item["Key"].as_str().unwrap(),
                        &item["Value"],
                        Some(curr_level + 1),
                    );
                } else {
                    key_traversal(&ii.to_string(), item, Some(curr_level + 1));
                }
                ii = ii + 1;
            }
        }
        ValueType::Object => {
            println!("level: {} => {:#?}: Object", curr_level, key);
            for (o_key, o_value) in value.as_object().unwrap() {
                key_traversal(o_key, o_value, Some(curr_level + 1));
            }
        }
        ValueType::Str => {
            let _res: Result<Value, serde_json::Error> =
                serde_json::from_str(value.as_str().unwrap());
            match _res {
                Ok(val) => {
                    if val != json!(null) {
                        key_traversal(key, &val, Some(curr_level + 1));
                    } else {
                        println!("level: {} => {:#?}: {:#?}", curr_level, key, val);
                    }
                }
                Err(_err) => {
                    println!(
                        "level: {} => {:#?}: {:#?}",
                        curr_level,
                        key,
                        value.as_str().unwrap()
                    );
                }
            }
        }
        ValueType::Null => {
            println!("level: {} => {:#?}: {:#?}", curr_level, key, value);
        }
    }
}

fn get_value_type(value: &Value) -> ValueType {
    if value.as_array() != None {
        ValueType::Array
    } else if value.as_object() != None {
        ValueType::Object
    } else if value.as_str() != None {
        ValueType::Str
    } else {
        ValueType::Null
    }
}
