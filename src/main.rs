use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

extern crate clap;
use clap::{Arg, App};

extern crate toml;
use toml::Value as Toml;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize)]
struct Config {
    emoji: String,
    text: String,
}

fn main() {
    let matches = App::new("process-slack-status")
        .arg(Arg::with_name("apiToken")
             .long("api-token")
             .value_name("TOKEN")
             .takes_value(true)
             .required(true)
             .help("Your Slack API token"))
        .arg(Arg::with_name("apiUrl")
             .long("api-url")
             .value_name("URL")
             .takes_value(true)
             .required(true)
             .help("Full URL to API path for the Slack team"))
        .arg(Arg::with_name("versionUid")
             .long("version-uid")
             .value_name("UID")
             .takes_value(true)
             .required(true)
             .help("The Slack version uid"))
        .arg(Arg::with_name("tick")
             .long("tick")
             .value_name("DUR")
             .takes_value(true)
             .required(true)
             .help("Duration to refresh status"))
        .arg(Arg::with_name("config")
             .long("config")
             .value_name("PATH")
             .takes_value(true)
             .required(false)
             .help("Config path (default: ./config.toml)"))
        .get_matches();

    let config = matches.value_of("config").unwrap_or("config.toml");
    println!("Value for config: {}", config);

    let mut input = String::new();
    File::open(config).and_then(|mut f| {
        f.read_to_string(&mut input)
    }).unwrap();

    let parsed = input.parse().map(parse_config).unwrap();
    println!("{:#?}", parsed);
}

fn parse_config(toml: Toml) -> HashMap<String, Config> {
    let mut map: HashMap<String, Config> = HashMap::new();
    match toml {
        Toml::Table(table) => {
            for (k, v) in table {
                let emoji = v["emoji"].as_str().unwrap();
                let text = v["text"].as_str().unwrap();

                map.insert(k, Config{emoji: emoji.into(), text: text.into()});
            }
        }
        _ => println!("skipped"),
    }
    return map;
}
