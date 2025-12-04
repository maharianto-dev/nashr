use std::{
    error::Error,
    io::{self, Write},
};

use serde::Serialize;
use serde_json::Value;

#[derive(Debug)]
enum ValueType {
    Array,
    Object,
    Str,
    Num,
    Boolean,
    Null,
}

enum StrFormatType {
    NewLineOpen,
    NewLineClose,
    NonNewLine,
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

    pub fn parse(&self) -> String {
        let val = key_traversal("", &self.value, None);
        println!("last result: {}", val);
        let formatted = format_json_string(val);
        // dbg!(&formatted);
        // print!("{}", &formatted);
        // io::stdout().flush().unwrap(); // Flush the output buffer
        formatted
    }
}

fn key_traversal(key: &str, value: &Value, level: Option<i32>) -> String {
    // 1. this value is an array => loop this
    // 2. this value is an object => loop the keys
    // 3. this value is str, num, boolean => get the value
    // 4. this value is null => get the value

    let mut new_json_str = String::new();
    let curr_level = level.unwrap_or_else(|| 1);
    dbg!(key);
    dbg!(get_value_type(value));
    match get_value_type(value) {
        ValueType::Array => {
            new_json_str = format!("[");

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
            if value["Key"] != Value::Null {
                // non-standard object consists like this { "Key": "a", "Value": "b" }
                // will be converted to object { "a": "b" }
                let frm = format!("{}", value["Value"]);
                let json_ob: Value = serde_json::from_str(&frm).unwrap();
                dbg!(&json_ob);
                dbg!(json_string_check(&json_ob));
                match json_string_check(&json_ob) {
                    ValueType::Null => {
                        format!("{{\"{}\": null}}", value["Key"].as_str().unwrap())
                    }
                    ValueType::Str => format!(
                        "{{\"{}\": \"{}\"}}",
                        value["Key"].as_str().unwrap(),
                        value["Value"].as_str().unwrap()
                    ),
                    ValueType::Array => format!(
                        "{{{}}}",
                        key_traversal(
                            value["Key"].as_str().unwrap(),
                            &value["Value"],
                            Some(curr_level + 1)
                        )
                    ),
                    ValueType::Object => format!(
                        "{{{}}}",
                        key_traversal(
                            value["Key"].as_str().unwrap(),
                            &value["Value"],
                            Some(curr_level + 1)
                        )
                    ),
                    ValueType::Num => format!(
                        "{{\"{}\": {}}}",
                        value["Key"].as_str().unwrap(),
                        value["Value"].as_number().unwrap()
                    ),
                    ValueType::Boolean => format!(
                        "{{\"{}\": {}}}",
                        value["Key"].as_str().unwrap(),
                        value["Value"].as_bool().unwrap()
                    ),
                }
            } else {
                new_json_str = format!("{{");

                for (index, (o_key, o_value)) in value.as_object().unwrap().iter().enumerate() {
                    let frm = format!("{}", o_value);
                    let json_ob: Value = serde_json::from_str(&frm).unwrap();
                    match get_value_type(&json_ob) {
                        ValueType::Null => {
                            new_json_str = format!("{}\"{}\": null", new_json_str, o_key.as_str());
                        }
                        ValueType::Str => {
                            new_json_str = format!(
                                "{}\"{}\": \"{}\"",
                                new_json_str,
                                o_key.as_str(),
                                o_value.as_str().unwrap()
                            );
                        }
                        ValueType::Array => {
                            new_json_str = format!(
                                "{}\"{}\":{}",
                                new_json_str,
                                o_key.as_str(),
                                key_traversal(o_key, o_value, Some(curr_level + 1))
                            );
                        }
                        ValueType::Object => {
                            new_json_str = format!(
                                "{}\"{}\": {}",
                                new_json_str,
                                o_key.as_str(),
                                key_traversal(o_key, o_value, Some(curr_level + 1))
                            );
                        }
                        ValueType::Num => {
                            new_json_str = format!(
                                "{}\"{}\": {}",
                                new_json_str,
                                o_key.as_str(),
                                o_value.as_number().unwrap()
                            );
                        }
                        ValueType::Boolean => {
                            new_json_str = format!(
                                "{}\"{}\": {}",
                                new_json_str,
                                o_key.as_str(),
                                o_value.as_bool().unwrap()
                            );
                        }
                    }
                    if index != value.as_object().unwrap().len() - 1 {
                        new_json_str = format!("{},", new_json_str);
                    }
                }
                new_json_str = format!("{}}}", new_json_str);
                new_json_str
            }
        }
        ValueType::Str => {
            dbg!(get_value_type(
                &serde_json::from_str(value.as_str().unwrap()).unwrap()
            ));
            match get_value_type(&serde_json::from_str(value.as_str().unwrap()).unwrap()) {
                ValueType::Object | ValueType::Array => {
                    format!("\"{}\":{}", key, value.as_str().unwrap())
                }
                _ => format!("\"{}\":\"{}\"", key, value.as_str().unwrap()),
            }
        }
        ValueType::Num => format!("\"{}\":{}", key, value.as_number().unwrap()),
        ValueType::Boolean => format!("\"{}\":{}", key, value.as_bool().unwrap()),
        ValueType::Null => format!("\"{}\": null", key),
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

fn json_string_check(value: &Value) -> ValueType {
    let frm = value.as_str().unwrap();
    let json_ob = serde_json::from_str(frm);
    match json_ob {
        Ok(val) => get_value_type(&val),
        Err(_) => ValueType::Str,
    }
}

fn format_json_string(json_string: String) -> String {
    let mut retval = String::new();
    let mut indent_level = 0;
    let mut previous_char: char = ' ';
    for char in json_string.chars().into_iter() {
        match char {
            '{' | '[' => {
                indent_level += 1;
                let indent = "\t".repeat(indent_level);
                retval = str_formatter(&retval, char, Some(indent), StrFormatType::NewLineOpen);
            }
            '}' | ']' => {
                indent_level -= 1;
                let indent = "\t".repeat(indent_level);
                retval = str_formatter(&retval, char, Some(indent), StrFormatType::NewLineClose);
            }
            ',' => {
                let indent = "\t".repeat(indent_level);
                match previous_char {
                    '\"' | ']' | '}' => {
                        retval =
                            str_formatter(&retval, char, Some(indent), StrFormatType::NewLineOpen)
                    }
                    _ => retval = str_formatter(&retval, char, None, StrFormatType::NonNewLine),
                }
            }
            _ => retval = str_formatter(&retval, char, None, StrFormatType::NonNewLine),
        }
        previous_char = char;
    }
    retval
}

fn str_formatter(
    prefix_str: &str,
    char_to_input: char,
    indent: Option<String>,
    format_type: StrFormatType,
) -> String {
    match format_type {
        StrFormatType::NewLineOpen => {
            format!("{}{}\n{}", prefix_str, char_to_input, indent.unwrap())
        }
        StrFormatType::NonNewLine => format!("{}{}", prefix_str, char_to_input),
        StrFormatType::NewLineClose => {
            format!("{}\n{}{}", prefix_str, indent.unwrap(), char_to_input)
        }
    }
}
