extern crate clap;
extern crate mpd;
extern crate reqwest;
extern crate time;

use clap::{Arg, App};
use mpd::{Client, State};
use std::net::TcpStream;

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
        .get_matches();

    let api_token = matches.value_of("apiToken").unwrap();
    let api_url = matches.value_of("apiUrl").unwrap();
    let version_uid = matches.value_of("versionUid").unwrap();

    let mut conn = Client::connect("127.0.0.1:6600").unwrap();
    let play_status = conn.status().unwrap();

    if play_status.state == State::Play {
        let current_song = conn.currentsong().unwrap().unwrap();

        let text = format!("{} - {}", current_song.title.unwrap(), current_song.tags["Artist"]);
        set_status(api_token, api_url, version_uid, text.as_str(), ":headphones:");
    } else {
        set_status(api_token, api_url, version_uid, "I don't even", ":question:");
    }
}

fn set_status(api_token: &str, api_url: &str, version_uid: &str, status_text: &str, status_emoji: &str) {
   let current_time = time::get_time();
    let inner = "{\"status_text\":\"".to_owned() + status_text + "\", \"status_emoji\":\"" + status_emoji + "\"}";
    let params = [
        ("token", api_token),
        ("profile", inner.as_str()),
    ];
    let client = reqwest::Client::new().unwrap();

    let url: String = api_url.to_owned() + "users.profile.set?_x_id=" + version_uid + "-" + format!("{}", current_time.sec).as_str();
    let res = client.post(url.as_str())
        .form(&params)
        .send()
        .unwrap();

    println!("result: {:#?}", res);
}
