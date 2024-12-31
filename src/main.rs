use clap::{Arg, Command};
mod mac_generate;
mod read_interfaces;
use mac_generate::{mac_from_input, mac_from_list, rand_mac};
use nix::unistd::geteuid;
use read_interfaces::list_interfaces;

use std::process;
fn main() {
    let dana = geteuid();
    if dana == 0.into() {
        let interfaces = list_interfaces().unwrap();
        let args_parsed = Command::new("conceal_me")
        .version("1.0")
        .author("dhmnztr")
        .about("mac changer but useful")
        .arg(
            Arg::new("first")
                .short('f')
                .long("first")
                .value_parser(|s: &str| {
                    if s.starts_with("list=") || s =="rng" || s == "input" {
                        Ok(s.to_string()) 
                    } else {
                        Err("Invalid value for 'first'. Use 'rng', 'input', or 'list=<value>'.")
                    }
                })
                .required(true),
        )
        .arg(
            Arg::new("clone")
                .short('c')
                .long("clone")
                .exclusive(true)
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("second")
                .short('s')
                .long("second")
                .required(true)
                .value_parser(clap::builder::PossibleValuesParser::new(["rng", "input"]))
                .help(
                    "rng -> it generates last 3 octects automatically\ninput -> reads your input",
                ),
        )
        .arg(Arg::new("interface")
            .short('i')
            .long("interface")
            .required(true)
            .value_parser(|interface: &str| {
                Ok::<String,String>(interface.to_string())
            })
        )
        .get_matches();
        let interface = args_parsed.get_one::<String>("interface").unwrap();
        let first_value = match args_parsed.get_one::<String>("first").unwrap() {
            value if value.starts_with("list=") => mac_from_list(&value[5..]).unwrap(),
            value if value == "rng" => rand_mac(),
            value if value == "input" => mac_from_input(),
            _ => unreachable!(),
        };
        let second_half: String = match args_parsed.get_one::<String>("second").unwrap() {
            value if value == "rng" => rand_mac(),
            value if value == "input" => mac_from_input(),
            _ => unreachable!(),
        };

        let mac = format!("{first_value}:{second_half}");
        if interfaces.contains(interface) {
            match process::Command::new("sudo")
                .arg("ip")
                .arg("set")
                .arg(interface)
                .arg("down")
                .output()
            {
                Ok(_) => {
                    match process::Command::new("sudo")
                        .arg("ip")
                        .arg("link")
                        .arg("set")
                        .arg("dev")
                        .arg(interface)
                        .arg("address")
                        .arg(mac)
                        .status()
                    {
                        Ok(_) => println!("Happy Hacking"),
                        Err(_) => println!("didnt workey"),
                    }
                }
                Err(_) => {
                    println!("couldnt change ip address? Make sure you can use ip link command")
                }
            };
        } else {
            println!("bad interface")
        }
    } else {
        println!("Run me as root")
    }
}
