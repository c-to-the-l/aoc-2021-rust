use anyhow::Result;
use dotenv::dotenv;
use reqwest::{blocking::Client, cookie::Jar, Url};
use std::env;
use std::sync::Arc;
use std::fs;
use std::io::{Read, Write};

pub fn get_input(year: i32, day: i32) -> Result<String> {
    if let Ok(mut file) = fs::File::open(format!(".cache/y{}d{}.txt", year, day)) {
        let mut resp: String = String::new();
        file.read_to_string(&mut resp)?;
        return Ok(resp);
    }
    dotenv()?;
    let aoc_session = env::var("AOC_SESSION")?;
    let cookie = format!("session={}, Domain=.adventofcode.com", aoc_session);
    let domain = "https://adventofcode.com".parse::<Url>()?;
    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &domain);
    let client = Client::builder().cookie_provider(Arc::new(jar)).build()?;
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let resp = client.get(url).send()?.error_for_status()?.text()?;
    fs::create_dir(".cache").ok();
    let mut file = fs::File::create(format!(".cache/y{}d{}.txt", year, day))?;
    if resp.len() != file.write(resp.as_ref())? {
        panic!("Couldn't completely write to cache .cache/y{}d{}.txt", year, day);
    }
    Ok(resp)
}
