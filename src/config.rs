// ##### Config File Functions #####

// These are the functions that set
// up the config file for the user.
// This is where the Todoist API key
// is stored.

use serde::{Serialize, Deserialize};
use tokio::runtime::Runtime;
use reqwest::{header, Client};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub todoist_key: String,
}

/// Generate default config file
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            todoist_key: String::new(),
        }
    }
}

fn input_key() -> std::string::String {
    let mut key: String = String::new();
    let _input_key = std::io::stdin().read_line(&mut key).unwrap();
    return key
}

async fn validate_key(key: &String) -> Result<bool, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", key).parse().unwrap());

    let res = Client::new()
        .get("https://api.todoist.com/rest/v1/projects")
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;


    if res == String::from("Forbidden") {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub fn config_setup() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = confy::load("coto").expect("Could not load config");

    if config.todoist_key == "" {
        println!("Enter your Todoist API key: ");

        let mut success: bool = false;
        let mut key: String = String::new();

        while success == false {
            let mut key_input: String = input_key();
            key_input = key_input[..key_input.len()-1].to_string();

            success = Runtime::new().expect("Could not validate key")
                .block_on(validate_key(&key_input))
                .unwrap();
            if success == true {
                key = key_input;
            } else {
                println!("Invalid Todoist API key")
            }
        }
        let updated: Config = Config {
            todoist_key: key,
        };

        confy::store("coto", updated).expect("Could not store config");
    }

    Ok(())
}

pub fn remove_key() {
    let config = Config {
        todoist_key: String::from(""),
    };
    confy::store("coto", config).expect("Could not remove previous key");
}
