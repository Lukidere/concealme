use clap::{Arg, Command};
mod net_grab;
mod mac_generate;
mod read_interfaces;
use net_grab::*;
use mac_generate::{mac_from_input, mac_from_list, rand_mac, mac_from_net};
use nix::unistd::geteuid;
use read_interfaces::list_interfaces;
use eyre::Result;
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
                .required(true)
                .long("first")
                .value_parser(|s: &str| {
                    if s.starts_with("list=") || ["rng","input","net"].contains(&s) {
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
                .conflicts_with_all(["first","second"])
                .action(clap::ArgAction::SetTrue)
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
        if args_parsed.get_flag("clone") {
            let interface = args_parsed.get_one::<String>("interface").unwrap();
            let choosen = choose_mac(interface.clone()).unwrap();
            setting_the_mac(choosen, interface.to_owned(), interfaces);
        }
        else {
        let interface = args_parsed.get_one::<String>("interface").unwrap();
        let first_value = match args_parsed.get_one::<String>("first") {
            Some(val) => val,
            _ => unreachable!()
        };
        let mut first_value = match first_value {
            value if value.starts_with("list=") => mac_from_list(&value[5..]).unwrap(),
            value if value == "rng" => rand_mac(),
            value if value == "input" => mac_from_input(),
            value if value == "net" => mac_from_net().unwrap(),
            _ => unreachable!(),
        };

        let second_half = match args_parsed.get_one::<String>("second") {
            Some(val) => val,
            _ => unreachable!()
        };
        let second_half: String = match second_half {
            value if value == "rng" => rand_mac(),
            value if value == "input" => mac_from_input(),
            _ => unreachable!(),
        };
        let first_value = "0".to_owned() + &first_value[1..];
        let mac = format!("{first_value}:{second_half}");
        setting_the_mac(mac, interface.to_owned(), interfaces);
        }
    } else {
        println!("Run me as root")
    }
}

fn setting_the_mac(mac:String,interface:String,interfaces:Vec<String>) -> Result<()> {
        if interfaces.contains(&interface.clone()) {
            match process::Command::new("sudo")
                .arg("ip")
                .arg("link")
                .arg("set")
                .arg("dev")
                .arg(interface.clone())
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
                        .arg(mac.trim())
                        .status()
                    {
                        Ok(_) => println!("Happy Hacking"),
                        Err(_) => println!("didnt workey"),
                    }
                }
                Err(_) => {
                    println!("couldnt change ip address? Make sure you can use ip link command")
                }
            }
            }
         else {
            println!("bad interface")
        }
    Ok(())
}
