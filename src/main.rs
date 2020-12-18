use clap::{App, Arg};

fn main() {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        // .about(clap::crate_description!()) //TODO
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
                .about("Default Target Number")
                .long_about(
"Default Target Number: All rolls will be compared against it. An explicit 
target number declaration in a roll overrides the default Target Number")
        )
        .arg(
            Arg::new("ROLL")
            .required(true)
            .takes_value(true)
            .multiple(true)
            .about("XdY[+A-B#Z]")
            .long_about(
"A dice roll formated as XdY[+A-B#Z]
X is Number of dice to roll
Y is the type of die
A and B are optional increments and decrements, respectively 
Z is a Target Number. A roll will be successful if Result >= TN
Please note that roll fails quietly - Incorrect input will simply be ignored"
            )
        )
        .arg(Arg::new("verbose")
            .short('v')
            .about("Verbose mode")
        )
        .get_matches();
    
    let roll_strs : Vec<&str> = matches.values_of("ROLL").unwrap().collect();
    for r in roll_strs {
        if let Err(e) = roll::run(
            r,
            matches.value_of("default_tn").unwrap_or("0").parse().unwrap(),
            matches.is_present("dice_pool"),
            matches.is_present("verbose"),
        ){
            println!("Failed for {} - {:?}",r,e);
        }

    }
}
