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

    pub fn to_parameter_string(&self) -> String {
        let mut parameter_string: String = "?".to_owned();

        let mut first_item = true;
        if let Some(id) = &self.id {
            if !first_item {
                parameter_string.push_str("&");
            }
            first_item = false;
            parameter_string.push_str("id=");
            parameter_string.push_str(id);
        }
        if let Some(platform) = &self.platform {
            if !first_item {
                parameter_string.push_str("&");
            }
            first_item = false;
            parameter_string.push_str("platform=");
            parameter_string.push_str(platform);
        }
        if let Some(giveaway_type) = &self.giveaway_type {
            if !first_item {
                parameter_string.push_str("&");
            }
            first_item = false;
            parameter_string.push_str("type=");
            parameter_string.push_str(giveaway_type);
        }
        if let Some(sort_by) = &self.sort_by {
            if !first_item {
                parameter_string.push_str("&");
            }
            parameter_string.push_str("sort-by=");
            parameter_string.push_str(sort_by);
        }

        parameter_string
    }
}

pub fn run(query: Query) -> Result<(), Box<dyn Error>> {
    let host_url = "https://gamerpower.com/api/giveaways";
    println!("{:#?}", &query);

    let parameter_string = query.to_parameter_string();
    let url = format!("{host_url}{parameter_string}");

    println!("Full url string: {}", &url);

    let body = reqwest::blocking::get(url)?.text()?;
    let json = json::parse(&body)?;

    print_query_json(json);
    Ok(())
}

fn print_query_json(json: JsonValue) {
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
}
