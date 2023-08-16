use serde::Deserialize;
use reqwest;

use std::collections::HashMap;
use std::cmp::Reverse;

#[derive(Deserialize)]
struct PyPiResponse {
    releases: HashMap<String, Vec<ReleaseInfo>>,
}

#[derive(Deserialize)]
struct ReleaseInfo {}

fn version_key(version: &String) -> (Vec<i32>, Vec<char>) {
    let parts: Vec<&str> = version.split('.').collect();
    let mut numbers = Vec::new();
    let mut chars = Vec::new();

    for part in parts {
        let mut number = String::new();
        for ch in part.chars() {

            if ch.is_numeric() {
                number.push(ch);
            } else {
                chars.push(ch);
            }
        }
        if let Ok(num) = number.parse::<i32>() {
            numbers.push(num);
        }
    }

    (numbers, chars)
}


pub fn version_query(package_name: &str) -> Result<Vec<String>, reqwest::Error> { 
    let url = format!("https://pypi.org/pypi/{}/json", package_name);
    let response: PyPiResponse = reqwest::blocking::get(&url)?.json()?;

    let mut versions: Vec<String> = response.releases.keys().cloned().collect();
    versions.sort_by_key(|v| Reverse(version_key(&v))); 
    Ok(versions.into_iter().take(1).collect()) 
}
