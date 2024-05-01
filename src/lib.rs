use colored::Colorize;
use json::JsonValue;
use std::error::Error;

#[derive(Debug)]
pub struct Query {
    id: Option<String>,
    platform: Option<String>,
    giveaway_type: Option<String>,
    sort_by: Option<String>,
}

impl Query {
    fn empty() -> Self {
        Self {
            id: None,
            platform: None,
            giveaway_type: None,
            sort_by: None,
        }
    }

    pub fn new(args: &[String]) -> Self {
        let mut query = Query::empty();

        for arg in args {
            let lower_arg = &arg.to_lowercase();
            if lower_arg.contains("id=") {
                let arg_setting_as_string = arg[3..].to_string().clone();
                query.id = Some(arg_setting_as_string);
            } else if lower_arg.contains("platform=") {
                let arg_setting_as_string = arg[9..].to_string().clone();
                query.platform = Some(arg_setting_as_string);
            } else if lower_arg.contains("type=") {
                let arg_setting_as_string = arg[5..].to_string().clone();
                query.giveaway_type = Some(arg_setting_as_string);
            } else if lower_arg.contains("sort-by=") {
                let arg_setting_as_string = arg[8..].to_string().clone();
                query.sort_by = Some(arg_setting_as_string);
            }
        }

        query
    }

    fn is_id_query(&self) -> bool {
        !self.id.is_none()
            && self.platform.is_none()
            && self.sort_by.is_none()
            && self.giveaway_type.is_none()
    }

    pub fn to_parameterized_url(&self) -> String {
        let mut parameterized_url: String = "https://gamerpower.com/api/giveaway".to_owned();
        if !self.is_id_query() {
            parameterized_url.push_str("s?");
        } else {
            parameterized_url.push_str("?");
        }

        let mut first_item = true;
        if let Some(id) = &self.id {
            if !first_item {
                parameterized_url.push_str("&");
            }
            first_item = false;
            parameterized_url.push_str("id=");
            parameterized_url.push_str(id);
        }
        if let Some(platform) = &self.platform {
            if !first_item {
                parameterized_url.push_str("&");
            }
            first_item = false;
            parameterized_url.push_str("platform=");
            parameterized_url.push_str(platform);
        }
        if let Some(giveaway_type) = &self.giveaway_type {
            if !first_item {
                parameterized_url.push_str("&");
            }
            first_item = false;
            parameterized_url.push_str("type=");
            parameterized_url.push_str(giveaway_type);
        }
        if let Some(sort_by) = &self.sort_by {
            if !first_item {
                parameterized_url.push_str("&");
            }
            parameterized_url.push_str("sort-by=");
            parameterized_url.push_str(sort_by);
        }

        parameterized_url
    }
}

pub fn run(query: Query) -> Result<(), Box<dyn Error>> {
    let parameterized_url = query.to_parameterized_url();
    //    println!("{:#?}", &query);

    //    println!("Full url string: {}", &parameterized_url);

    let body = reqwest::blocking::get(parameterized_url)?.text()?;
    let json = json::parse(&body)?;

    print_query_json(json);
    Ok(())
}

fn print_query_json(json: JsonValue) {
    if json.is_array() {
        for i in 0..json.len() {
            if json[i].is_null() {
                continue;
            }
            println!(
                "{}, {}, {}",
                &json[i]["title"].to_string().bold(),
                &json[i]["worth"].to_string().green(),
                &json[i]["gamerpower_url"].to_string().purple()
            );
        }
    } else {
        println!(
            "{}, {}, {}",
            &json["title"].to_string().bold(),
            &json["worth"].to_string().green(),
            &json["gamerpower_url"].to_string().purple()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_query() {
        let query = Query {
            id: Some("525".to_owned()),
            platform: None,
            giveaway_type: None,
            sort_by: None,
        };

        let parameterized_url = query.to_parameterized_url();
        let json_text = reqwest::blocking::get(parameterized_url)
            .unwrap()
            .text()
            .unwrap();
        let json = json::parse(&json_text).unwrap();
        assert!(!json["title"].is_null());
    }

    #[test]
    fn list_query() {
        let query = Query {
            id: None,
            platform: Some("steam".to_owned()),
            giveaway_type: Some("loot".to_owned()),
            sort_by: Some("value".to_owned()),
        };

        let parameterized_url = query.to_parameterized_url();
        let json_text = reqwest::blocking::get(parameterized_url)
            .unwrap()
            .text()
            .unwrap();
        let json = json::parse(&json_text).unwrap();
        assert!(json.is_array());
    }
}
