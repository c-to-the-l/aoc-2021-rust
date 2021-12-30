// build.rs
use anyhow::Result;
use dotenv::dotenv;
use reqwest::{blocking::Client, cookie::Jar, Url};
use std::env;
use std::sync::Arc;
use std::fs;
use std::io::{Write, Read};
use std::path::Path;

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

pub fn day24() -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("day24-gen.rs");
    let mut f = fs::File::create(dest_path)?;
    let input = get_input(2021, 24)?;
    let mut stg = 1;

    for line in input.lines() {
        let ins = line.split_whitespace().next().unwrap();
        let lhs = line.split_whitespace().nth(1).unwrap();
        let rhs = line.split_whitespace().nth(2);

        match ins {
            "inp" => {
                if stg != 1 {
                    writeln!(f, "(x, y, z)")?;
                    writeln!(f, "}}")?;
                }
                if stg != 14 {
                    writeln!(f, "fn alu_step{}(w: i64, x: i64, y: i64, z: i64) -> (i64, i64, i64) {{", stg)?;
                } else {
                    writeln!(f, "fn alu_step{}(w: i64, x: i64, y: i64, z: i64) -> i64 {{", stg)?;
                }
                stg += 1;
            }
            "add" => writeln!(f, "let {} = {} + {};", lhs, lhs, rhs.unwrap())?,
            "mul" => writeln!(f, "let {} = {} * {};", lhs, lhs, rhs.unwrap())?,
            "div" => writeln!(f, "let {} = {} / {};", lhs, lhs, rhs.unwrap())?,
            "mod" => writeln!(f, "let {} = {} % {};", lhs, lhs, rhs.unwrap())?,
            "eql" => writeln!(f, "let {} = ({} == {}) as i64;", lhs, lhs, rhs.unwrap())?,
            _ => {},
        }

    }
    writeln!(f, "z")?;
    writeln!(f, "}}")?;

    Ok(())

}


fn main() {
    day24().unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
