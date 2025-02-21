use eyre::Result;
use glob::glob;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::io::stdin;
use std::iter::zip;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
#[derive(Debug, Deserialize)]
struct Bssid {
    mac: String,
    first_time_seen: String,
    last_time_seen: String,
    channel: String,
    speed: String,
    privacy: String,
    cipher: String,
    authentication: String,
    power: String,
    beacons: String,
    iv: String,
    lan_ip: String,
    id_length: String,
    essid: String,
    key: String,
}

#[derive(Debug, Deserialize)]
struct Station {
    mac: String,
    first_time_seen: String,
    last_time_seen: String,
    power: String,
    packets: String,
    bssid: String,
    probed_essids: String,
}
pub trait showcase {
    fn info(&self) -> String;
    fn mac(&self) -> String;
}

impl showcase for Station {
    fn info(&self) -> String {
        format!("station_mac:{} | stations_bssid:{}", &self.mac, &self.bssid)
    }
    fn mac(&self) -> String {
        return (&self.mac).to_owned();
    }
}
impl showcase for Bssid {
    fn info(&self) -> String {
        format!("Bssid:{} | Essid:{}", &self.mac, &self.essid)
    }
    fn mac(&self) -> String {
        return (&self.mac).to_owned();
    }
}
fn option<T: showcase>(list: Vec<T>) -> String {
    println!("Choose one option:");
    list.iter()
        .enumerate()
        .for_each(|(numer, siec)| println!("{}. {}", numer, siec.info()));
    let mut answer: String = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    let number = match answer.trim().parse::<usize>() {
        Ok(val) => val,
        Err(_) => 0,
    };
    if number < list.len() {
        return list[number].mac();
    } else {
        return list.iter().next().unwrap().mac();
    }
}
pub fn choose_mac(interface: String) -> Result<String> {
    let (bssids, stations) = scan_networks(interface)?;
    println!("Type either \"network\" or \"station\" to choose what to mimic");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;
    let choosen_mac = match answer.trim() {
        "network" => option(bssids),
        "station" => option(stations),
        _ => option(bssids),
    };
    Ok(choosen_mac)
}
pub fn scan_networks(interface: String) -> Result<(Vec<Bssid>, Vec<Station>)> {
    let output_name = "temporary_output";
    delete_leftovers("temporary_output");
    let mut monitor_mode = Command::new("sudo")
        .args(["airmon-ng", "start", &interface])
        .stdout(Stdio::piped())
        .spawn()?;
    thread::sleep(Duration::from_secs(1));
    monitor_mode.kill()?;

    let interface_mon = format!("{interface}mon");
    let mut stations: Vec<Station> = vec![];
    let mut bssids: Vec<Bssid> = vec![];
    let mut execute = Command::new("sudo")
        .args([
            "airodump-ng",
            "--output-format",
            "csv",
            &interface_mon,
            "-w",
            output_name,
        ])
        .stdout(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("couldnt spawn process");
    thread::sleep(Duration::from_secs(5));
    execute.kill().expect("failed to kill a child");
    let file = File::open(format!("{output_name}-01.csv"))?;
    let mut monitor_mode2 = Command::new("sudo")
        .args(["airmon-ng", "stop", &interface_mon])
        .stdout(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    thread::sleep(Duration::from_secs(1));
    monitor_mode2.kill()?;
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(file);

    for result in rdr.records() {
        let record = result?;
        match record.len() {
            15 => {
                let dana: Bssid = record.deserialize(None)?;
                bssids.push(dana)
            }
            7 => {
                let dana: Station = record.deserialize(None)?;
                if dana.mac.contains(":") {
                    stations.push(dana)
                } else {
                }
            }
            _ => unreachable!(),
        }
    }
    delete_leftovers("temporary_output");
    Ok((bssids, stations))
}

fn delete_leftovers(name: &str) {
    let pattern = format!("{name}*.csv");
    for entry in glob(&pattern).expect("Failed to read pattern") {
        match entry {
            Ok(path) => if let Err(e) = fs::remove_file(&path) {},
            Err(e) => {}
        }
    }
}
