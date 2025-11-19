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

fn key_traversal(key: &str, value: &Value, level: Option<i32>) -> String {
    // 1. this value is an array => loop this
    // 2. this value is an object => loop the keys
    // 3. this value is str, num, boolean => get the value
    // 4. this value is null => get the value

    let mut new_json_str = String::new();
    let curr_level = level.unwrap_or_else(|| 1);

    // println!("{:#?}", key);
    // println!("{:#?}", get_value_type(value));

    match get_value_type(value) {
        ValueType::Array => {
            println!("level: {} => {:#?}: Array", curr_level, key);

            new_json_str = format!("[");

            println!("new_json: {:#?}", new_json_str);

            let mut ii = 0;
            for item in value.as_array().unwrap() {
                new_json_str = format!(
                    "{}{}",
                    new_json_str,
                    key_traversal(&ii.to_string(), item, Some(curr_level + 1))
                );
                if ii != value.as_array().unwrap().len() - 1 {
                    new_json_str = format!("{},", new_json_str);
                }
                ii = ii + 1;
            }
            new_json_str = format!("{}]", new_json_str);
            new_json_str
        }
        ValueType::Object => {
            println!("level: {} => {:#?}: Object", curr_level, key);

            if value["Key"] != Value::Null {
                // non-standard object consists like this { "Key": "a", "Value": "b" }
                // will be converted to "a": "b"
                let frm = format!("{}", value["Value"]);
                let json_ob: Value = serde_json::from_str(&frm).unwrap();
                match json_string_check(json_ob) {
                    ValueType::Null => format!("{}: null", value["Key"]),
                    ValueType::Str => format!("{}: {}", value["Key"], value["Value"]),
                    ValueType::Array => todo!(),
                    ValueType::Object => todo!(),
                    ValueType::Num => todo!(),
                    ValueType::Boolean => todo!(),
                }
            } else {
                new_json_str = format!("{{");

                new_json_str = format!("{}}}", new_json_str);
                new_json_str
            }

            // for (o_key, o_value) in value.as_object().unwrap() {
            //     let new_obj = key_traversal(o_key, o_value, Some(curr_level + 1));
            //     new_json
            //         .as_object_mut()
            //         .unwrap()
            //         .insert(o_key.to_string(), new_obj);
            // }
        }
        ValueType::Str => todo!(),
        ValueType::Num => todo!(),
        ValueType::Boolean => todo!(),
        ValueType::Null => todo!(),
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

fn json_string_check(value: Value) -> ValueType {
    let frm = value.as_str().unwrap();
    println!("json_string_check::frm: {}", frm);
    let json_ob = serde_json::from_str(frm);
    match json_ob {
        Ok(val) => get_value_type(&val),
        Err(_) => ValueType::Str,
    }
}
