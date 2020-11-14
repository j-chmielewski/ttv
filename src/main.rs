use std::collections::HashMap;
use std::env;
use clap::{App, Arg, ArgMatches};
use colored::*;
use serde::Deserialize;


static API_URL: &str = "https://api.twitch.tv/helix/streams?user_login=";
static AUTH_URL: &str = "https://id.twitch.tv/oauth2/token";

#[derive(Deserialize, Debug)]
struct Token {
  access_token: String,
  expires_in: u32,
  token_type: String
}

#[derive(Deserialize, Debug)]
struct User {
    id: String,
    user_id: String,
    user_name: String,
    game_id: String,
    game_name: String,
    title: String,
    viewer_count: u32,
    started_at: String,
    language: String,
    thumbnail_url: String,
}

#[derive(Deserialize, Debug)]
struct Streams {
    data: Vec<User>,
}

fn configure_clap<'a>() -> ArgMatches<'a> {
    App::new("ttv")
        .author("Jacek Chmielewski <jchmielewski@teonite.com>")
        .version(env!("CARGO_PKG_VERSION"))
        .args(&[
            Arg::with_name("channels")
            .multiple(true)
            .required(true)
            .help("Channels to check.")
       ])
        .get_matches()
}

fn get_token() -> Token {
    let mut request = HashMap::new();
    request.insert("client_id", env::var("TTV_CLIENT_ID").expect("TTV_CLIENT_ID environment variable not set."));
    request.insert("client_secret", env::var("TTV_CLIENT_SECRET").expect("TTV_CLIENT_SECRET environment variable not set."));
    request.insert("grant_type", String::from("client_credentials"));
    let client = reqwest::blocking::Client::new();
    let response = client.post(AUTH_URL)
        .json(&request)
        .send()
        .unwrap();
    response.json().unwrap()
}

fn main() {
    let matches = configure_clap();
    let token = get_token();
    for channel in matches.values_of("channels").unwrap() {
        let client = reqwest::blocking::Client::new();
        let streams: Streams = client.get(&format!("{}{}", API_URL, channel))
        .header("Client-Id", env::var("TTV_CLIENT_ID").unwrap())
        .header("Authorization", format!("Bearer {}", token.access_token))
        .send()
        .unwrap()
        .json()
        .unwrap();

        if streams.data.len() > 0 {
            println!("{:30} {}", channel, "Online".bright_green())
        } else  {
            println!("{:30} {}", channel, "Offline".bright_red())
        }
    }
}
