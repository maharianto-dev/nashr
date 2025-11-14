use regex::Regex;
use serde::Serialize;

pub enum GuidType {
    Uuid,
    Hex,
}

pub struct GuidInput {
    pub guid_input: String,
    pub guid_type: Option<GuidType>,
}

#[derive(Serialize)]
pub struct GuidResult {
    pub guid_standard: String,
    pub guid_bracketed: String,
    pub guid_oracle_raw16_format: String,
    pub guid_oracle_hextoraw: String,
    pub guid_sqlserver_using_oracle_byte_order: String,
}

impl GuidInput {
    pub fn new(guid_input: &str) -> Self {
        let mut clean_input = guid_input.trim();
        if clean_input.starts_with("{") && clean_input.ends_with("}") {
            let mut chars = clean_input.chars();
            chars.next();
            chars.next_back();
            clean_input = chars.as_str();
        }
        Self {
            guid_input: clean_input.to_string(),
            guid_type: None,
        }
    }

    pub fn check_guid(&mut self) -> bool {
        let regex_uuid_pattern =
            Regex::new("^[{]?[0-9a-fA-F]{8}-([0-9a-fA-F]{4}-){3}[0-9a-fA-F]{12}[}]?$").unwrap();
        let regex_hex_pattern = Regex::new("^[{]?[0-9a-fA-F]{32}[}]?").unwrap();

        if regex_uuid_pattern.is_match(&self.guid_input) {
            self.guid_type = Some(GuidType::Uuid);
            return true;
        }

        if regex_hex_pattern.is_match(&self.guid_input) {
            self.guid_type = Some(GuidType::Hex);
            return true;
        }

        return false;
    }
}

impl GuidResult {
    pub fn new(gi: &GuidInput) -> Self {
        let gs = get_standard_uuid(gi).to_uppercase();
        let gb = get_bracketed_guid(gi).to_uppercase();
        let gor = get_oracle_raw16_format(gi).to_uppercase();
        let goh = get_oracle_hextoraw(gi).to_uppercase();
        let gss = get_sqlserver_using_oracle_byte_order(gi);

        GuidResult {
            guid_standard: gs,
            guid_bracketed: gb,
            guid_oracle_raw16_format: gor,
            guid_oracle_hextoraw: goh,
            guid_sqlserver_using_oracle_byte_order: gss,
        }
    }
}

fn swap_byte(input: &str) -> String {
    format!(
        "{}{}{}{}-{}{}-{}{}-{}-{}",
        &input[6..8],
        &input[4..6],
        &input[2..4],
        &input[0..2],
        &input[10..12],
        &input[8..10],
        &input[14..16],
        &input[12..14],
        &input[16..20],
        &input[20..]
    )
    .to_string()
}

fn clean_dashes(input: &str) -> String {
    input.replace("-", "")
}

fn get_standard_uuid(gi: &GuidInput) -> String {
    match gi.guid_type.as_ref().unwrap() {
        GuidType::Uuid => gi.guid_input.clone(),
        GuidType::Hex => swap_byte(&gi.guid_input),
    }
}

fn get_bracketed_guid(gi: &GuidInput) -> String {
    match gi.guid_type.as_ref().unwrap() {
        GuidType::Uuid => format!("{{{}}}", gi.guid_input),
        GuidType::Hex => format!("{{{}}}", get_standard_uuid(gi)),
    }
}

fn get_oracle_raw16_format(gi: &GuidInput) -> String {
    match gi.guid_type.as_ref().unwrap() {
        GuidType::Uuid => {
            let clean = clean_dashes(&gi.guid_input);
            let res = swap_byte(&clean);
            clean_dashes(&res)
        }
        GuidType::Hex => gi.guid_input.clone(),
    }
}

fn get_oracle_hextoraw(gi: &GuidInput) -> String {
    match gi.guid_type.as_ref().unwrap() {
        GuidType::Uuid => format!("HEXTORAW({})", get_oracle_raw16_format(gi)),
        GuidType::Hex => format!("HEXTORAW({})", gi.guid_input),
    }
}

fn get_sqlserver_using_oracle_byte_order(gi: &GuidInput) -> String {
    match gi.guid_type.as_ref().unwrap() {
        GuidType::Uuid => format!("0x{}", get_oracle_raw16_format(gi).to_uppercase()),
        GuidType::Hex => format!("0x{}", gi.guid_input.to_uppercase()),
    }
}
