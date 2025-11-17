use std::error::Error;

use serde::Serialize;
use serde_json::{json, Value};

type JsonLinkedList = ();

#[derive(Debug)]
enum ValueType {
    Array,
    Object,
    Str,
    Num,
    Boolean,
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
        let mut new_json: Value;
        // let mut counter = 1;
        // for (key, value) in self.value.as_object().unwrap() {
        //     key_traversal(key, value, None);
        //     counter = counter + 1;
        // }

        let val = key_traversal("", &self.value, None);

        println!("last result: {:#?}", val);

        Ok(())
    }
}

pub fn key_traversal(key: &str, value: &Value, level: Option<i32>) -> Value {
    // 1. this value is an array => loop this
    // 2. this value is an object => loop the keys
    // 3. this value is str, num, boolean => get the value
    // 4. this value is null => get the value

    let mut new_json: Value = json!(null);
    let curr_level = level.unwrap_or_else(|| 1);

    // println!("{:#?}", key);
    // println!("{:#?}", get_value_type(value));

    match get_value_type(value) {
        ValueType::Array => {
            println!("level: {} => {:#?}: Array", curr_level, key);

            new_json = serde_json::from_str("[]").unwrap();

            println!("new_json: {:#?}", new_json);

            let mut ii = 0;
            for item in value.as_array().unwrap() {
                if item["Key"] != json!(null) {
                    new_json.as_array_mut().unwrap().push(key_traversal(
                        item["Key"].as_str().unwrap(),
                        &item["Value"],
                        Some(curr_level + 1),
                    ));
                } else {
                    key_traversal(&ii.to_string(), item, Some(curr_level + 1));
                }
                ii = ii + 1;
            }
            new_json
        }
        ValueType::Object => {
            println!("level: {} => {:#?}: Object", curr_level, key);

            new_json = serde_json::from_str("{}").unwrap();

            for (o_key, o_value) in value.as_object().unwrap() {
                let new_obj = key_traversal(o_key, o_value, Some(curr_level + 1));
                new_json
                    .as_object_mut()
                    .unwrap()
                    .insert(o_key.to_string(), new_obj);
            }
            new_json
        }
        ValueType::Str => {
            let _res: Result<Value, serde_json::Error> =
                serde_json::from_str(value.as_str().unwrap());
            match _res {
                Ok(val) => {
                    if val != json!(null) {
                        key_traversal(key, &val, Some(curr_level + 1));
                        new_json
                    } else {
                        println!("level: {} => {:#?}: {:#?}", curr_level, key, val);
                        let frm = format!("{:#?}: null", key);
                        println!("frm: {}", frm);
                        return serde_json::from_str(&frm).unwrap();
                    }
                }
                Err(_err) => {
                    println!(
                        "level: {} => {:#?}: {:#?}",
                        curr_level,
                        key,
                        value.as_str().unwrap()
                    );
                    return serde_json::from_str(
                        format!("{:#?}: {:#?}", key, value.as_str().unwrap()).as_str(),
                    )
                    .unwrap();
                }
            }
        }
        ValueType::Num => {
            println!(
                "level: {} => {:#?}: {:#?}",
                curr_level,
                key,
                value.as_number().unwrap().to_string()
            );
            let val = value.as_number().unwrap().as_f64().unwrap();
            let frm = format!("{:#?}: {:#?}", key, val);
            println!("frm: {}", frm);
            return serde_json::from_str(&frm).unwrap();
        }
        ValueType::Boolean => {
            println!(
                "level: {} => {:#?}: {:#?}",
                curr_level,
                key,
                value.as_bool().unwrap().to_string()
            );
            return serde_json::from_str(
                format!("{:#?}: {:#?}", key, value.as_bool().unwrap()).as_str(),
            )
            .unwrap();
        }
        ValueType::Null => {
            println!("level: {} => {:#?}: {:#?}", curr_level, key, value);
            return serde_json::from_str(format!("{:#?}: null", key).as_str()).unwrap();
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
    } else if value.as_number() != None {
        ValueType::Num
    } else if value.as_bool() != None {
        ValueType::Boolean
    } else {
        ValueType::Null
    }
}
