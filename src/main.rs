extern crate clap;
use clap::{Arg, App};

extern crate reqwest;

extern crate time;

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

    let current_time = time::get_time();
    let inner = "{\"status_text\":\"".to_owned() + "foo" + "\", \"status_emoji\":\"" + ":question:" + "\"}";
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
