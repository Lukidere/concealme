use clap::{arg, builder::TypedValueParser, command, Arg, Command};
use hex;
mod mac_generate;
use mac_generate::{mac_from_input, mac_from_list, rand_mac};
fn main() {
    let args_parsed = Command::new("conceal_me")
        .version("1.0")
        .author("dhmnztr")
        .about("mac changer but useful")
        .arg(
            Arg::new("first")
                .short('f')
                .long("first")
                .value_parser(|s: &str| {
                    if s.starts_with("list=") {
                        Ok(s.to_string()) // Allow 'list=<value>' format
                    } else if s == "rng" || s == "input" {
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
        .get_matches();
    let rng = String::from("rng");
    let generated_mac = String::new();
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
    println!("{first_value}:{second_half}")
}
