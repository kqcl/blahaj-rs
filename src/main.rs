mod color_schemes;
mod ascii_sharky;

use color_schemes::{color_schemes, ColorArray};
use ascii_sharky::BLAHAJ_ART;

use std::collections::HashMap;
use std::io::{self, BufRead};
use clap::{command, crate_version, Arg, ArgMatches};
use rand::Rng;

fn main() {
    let match_results = cli();
    let schemes = color_schemes();

    if let Some(true) = match_results.get_one::<bool>("list").copied() {
        list_color_schemes(&schemes);
        return;
    }

    let scheme_name = determine_scheme_name(&match_results, &schemes);

    if let Some(true) = match_results.get_one::<bool>("sharky").copied() {
        display_sharky(&schemes, scheme_name);
    } else {
        colorize_input(&schemes, scheme_name);
    }
}

fn cli() -> ArgMatches {
    command!()
        .about("A tool for summoning powerful gay ikea sharks to colorize your terminal :3")
        .version(crate_version!())
        .arg(
            Arg::new("flag-name")
                .short('f')
                .long("flag-name")
                .aliases(["flagname", "fname", "flag"])
                .default_value("trans")
                .help("This specifies the pride flag from the default register")
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .num_args(0)
                .aliases(["ls"])
                .help("Get a list of all the flags in the default register.")
                .conflicts_with("flag-name")
        )
        .arg(
            Arg::new("random")
                .short('r')
                .long("random")
                .num_args(0)
                .aliases(["rand", "rndm", "rnd"])
                .help("Chooses a random flag.")
                .conflicts_with("flag-name")
        )
        .arg(
            Arg::new("sharky")
                .short('s')
                .long("sharky")
                .num_args(0)
                .aliases(["shark"])
                .help("Print a cute blahaj into your console.")
        )
        .get_matches()
}

fn list_color_schemes(schemes: &HashMap<&'static str, ColorArray>) {
    println!("Available color schemes:");
    for scheme in schemes.keys() {
        println!("{}", scheme);
    }
}

fn determine_scheme_name<'a>(matches: &'a ArgMatches, schemes: &'a HashMap<&'static str, ColorArray>) -> &'a str {
    let scheme_name = matches
        .get_one::<String>("flag-name")
        .map(String::as_str)
        .unwrap_or("trans");

    if matches.get_one::<bool>("random").copied().unwrap_or(false) {
        get_random_key(schemes).unwrap_or(scheme_name)
    } else {
        scheme_name
    }
}

fn get_random_key(map: &HashMap<&'static str, ColorArray>) -> Option<&'static str> {
    let keys: Vec<_> = map.keys().cloned().collect();
    if keys.is_empty() {
        None
    } else {
        let random_index = rand::thread_rng().gen_range(0..keys.len());
        Some(keys[random_index])
    }
}

fn display_sharky(schemes: &HashMap<&'static str, ColorArray>, scheme_name: &str) {
    if let Some(colors) = get_color_array(schemes, scheme_name) {
        print_colored_blahaj(colors);
    } else {
        eprintln!("Error: Invalid color scheme specified.");
    }
}

fn get_color_array<'a>(schemes: &'a HashMap<&'a str, ColorArray>, scheme_name: &str) -> Option<&'a ColorArray> {
    schemes.get(scheme_name)
}

fn print_colored_blahaj(colors: &ColorArray) {
    for (color_index, line) in BLAHAJ_ART.lines().enumerate() {
        let color = colors[color_index % colors.len()];
        println!("{}{}\x1b[0m", color, line);
    }
}

fn colorize_input(schemes: &HashMap<&'static str, ColorArray>, scheme_name: &str) {
    let colors = match get_color_array(schemes, scheme_name) {
        Some(colors) => colors,
        None => {
            eprintln!("Error: Invalid color scheme specified.");
            return;
        }
    };

    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    for (line_index, line) in stdin_lock.lines().enumerate() {
        match line {
            Ok(line) => {
                let color = &colors[line_index % colors.len()];
                println!("{}{}\x1b[0m", color, line);
            }
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }
}
