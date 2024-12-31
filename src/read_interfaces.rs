use eyre::Result;
use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio};
pub fn list_interfaces() -> Result<Vec<String>> {
    let ip_link_output = Command::new("ip")
        .arg("link")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to capture ip link output"))?;

    let grep_output = Command::new("grep")
        .arg("-P")
        .arg(r"^[0-9]")
        .stdin(Stdio::from(ip_link_output))
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to capture grep output"))?;

    let reader = BufReader::new(grep_output);
    let interfaces: Vec<String> = reader
        .lines()
        .map(|line| match line {
            Ok(val) => val.split(':').nth(1).unwrap().trim().to_string(),
            Err(_) => String::from(""),
        })
        .filter(|str| !str.is_empty())
        .collect();
    Ok(interfaces)
}
