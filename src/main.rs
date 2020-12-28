use clap::{App, Arg};
use std::process;

fn build_app<'a>() -> clap::App<'a> {
    App::new(clap::crate_name!())
    .version(clap::crate_version!())
    .author(clap::crate_authors!())
    .about(clap::crate_description!())
    .arg(
        Arg::new("dice_pool")
            .short('p')
            .long("pool")
            .about("Roll dice as a dice pool")
            .long_about(
"Dice Pool mode: Each die of a roll is independent of the others")
    )
    .arg(
        Arg::new("default_tn")
            .short('t')
            .long("target")
            .takes_value(true)
            .about("Default Target Number (overriden by [#target])")
            .long_about(
"Default Target Number: All rolls will be compared against it. An explicit 
target number declaration in a roll overrides the default Target Number")
    )
    .arg(
        Arg::new("ROLL")
        .required(true)
        .takes_value(true)
        .multiple(true)
        .about("XdY[+i-d...][#target]")
        .long_about(
"XdY[+i-d...][#t] is the roll format, where:
X is Number of dice to roll
Y is the type of die
i and d are optional increments and decrements, respectively. 
t is a Target Number. A roll result is successful if >= t"
        )
    )
    .arg(Arg::new("verbose")
        .short('v')
        .about("Verbose mode")
    )
    .after_help(
"--help for further details

EXAMPLES:
roll 2d20+12-3+1-1-2+4#10
roll -p 3d6+1
roll 1d20
"
    )
}

fn main() {
    let mut return_code = 0;
    let matches = build_app().get_matches();
    let is_verbose = matches.is_present("verbose");
    let roll_strs : Vec<&str> = matches.values_of("ROLL").unwrap().collect();
    let default_tn = match matches.value_of("default_tn").unwrap_or("0").parse::<isize>() {
        Err(_e) => {
            return_code = 1;
            if is_verbose { eprintln!("[!] Invalid default target number - defaulting to zero") }
            0
        }
        Ok(t) => t
    };

    for r in roll_strs {
        if let Err(e) = roll::run(
            r,
            default_tn,
            matches.is_present("dice_pool"),
            is_verbose,
        ){
            if is_verbose { eprintln!("[!] {}\n",e) };
            return_code = 1;
        }
    }
    process::exit(return_code);
}
