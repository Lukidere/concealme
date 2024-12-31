use csv::ReaderBuilder;
use eyre::Result;
use rand::{thread_rng, Rng};
use regex::Regex;
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use std::{fs, io};
pub fn rand_mac() -> String {
    let mut ready = String::new();
    for number in 0..6 {
        if [2, 4].contains(&number) {
            ready.push(':');
        }

        ready.push(
            format!("{:X}", thread_rng().gen_range(0..16))
                .chars()
                .next()
                .unwrap(),
        );
    }
    ready
}

pub fn mac_from_input() -> String {
    println!("Provide 3 octets of your desired mac address");
    println!("It should have this format FF:FF:FF where FF are letters from A to F");
    let regex = Regex::new(r"([0-9a-fA-F]{2}):([0-9a-fA-F]{2}):([0-9a-fA-F]{2})").unwrap();
    let mut mac_do_sprawdzenia = String::new();
    io::stdin()
        .read_line(&mut mac_do_sprawdzenia)
        .expect("Couldnt read input");
    let _ = mac_do_sprawdzenia.trim();
    if let Some(captures) = regex.captures(&mac_do_sprawdzenia) {
        captures.iter().for_each(|capture| println!("{capture:#?}"));
    }
    return String::from("Cos");
}

#[derive(Serialize, Deserialize)]
struct Dana {
    Assignment: String,
}
pub fn mac_from_list(path: &str) -> Result<String> {
    let content = fs::read_to_string(path)?;
    // uncomment these if you know what you are doing
    // let macs = "https://standards-oui.ieee.org/oui/oui.csv"; |
    // let response = get(macs)?;
    // let content = response.text()?;
    let mut reader = ReaderBuilder::new().from_reader(content.as_bytes());
    let data: Result<Vec<Dana>, csv::Error> = reader
        .deserialize()
        .map(|result| {
            result.map(|mut line: Dana| {
                line.Assignment.insert(2, ':');
                line.Assignment.insert(5, ':');
                line
            })
        })
        .collect();
    let data = data.unwrap();
    Ok(data
        .get(thread_rng().gen_range(0..data.len()))
        .unwrap()
        .Assignment
        .clone())
}
