use std::path::Path;

use reqwest;
use toml::Table;
use anyhow::Result;


fn get_cookie() -> String {
    let cookie_file = std::fs::read_to_string("cookie.toml")
        .expect("Error reading cookie.toml");
    let cookie_table = cookie_file.parse::<Table>()
        .expect("Error parsing cookie.toml");
    cookie_table.get("cookie").unwrap().to_string()
}


/// Get the input for `day` from the advent of code website and cache it as a txt.
fn get_puzzle_input_from_website(day: i32, part: i32) ->Result<String> {
    let cookie = format!("session={}", get_cookie());

    let url = match part {
        1 => format!("https://adventofcode.com/2023/day/{day}/input"),
        _ => panic!("Invalid part number: {}", part),
    }.parse::<reqwest::Url>()?;

    let builder = reqwest::blocking::ClientBuilder::new();
    let jar = reqwest::cookie::Jar::default();
    jar.add_cookie_str(cookie.as_str(), &url);

    let client = builder.cookie_provider(jar.into()).build()?;
    let result = client.get(url).send()?.text()?;

    std::fs::write(format!("inputs/day_{day}.txt").as_str(), result.clone()).unwrap();
    Ok(result)
}

/// Get the cached file for `day`. If it's not found, get it from the website and cache it.
pub fn get_puzzle_input(day: i32, part: i32) -> String{
    let path_str = format!("inputs/day_{day}.txt");
    let local_path = Path::new(path_str.as_str());
    if local_path.is_file() {
        return std::fs::read_to_string(local_path).expect("Error reading local cached input file");
    };
    get_puzzle_input_from_website(day, part).expect("Error reading input file form website")

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_puzzle_input_for_day_1() {
        let input = get_puzzle_input(1, 1);
        assert!(input.len() > 0);
        assert!(!input.contains("Puzzle inputs differ by user."))
    }

    #[test]
    fn test_get_puzzle_input_for_days() {
        let curr_day = 2;
        (1..=curr_day).into_iter().for_each(|d| {
            let input = get_puzzle_input(d, 1);
            assert!(input.len() > 0);
            assert!(!input.contains("Puzzle inputs differ by user."))
        })
    }
}
