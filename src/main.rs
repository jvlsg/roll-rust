use clap::{Arg,App};

fn main() {
    let opts = App::new("roll")
        .arg(
            Arg::new("dice_pool")
                .short('p')
                .long("pool")
                .about("Roll dice as a dice pool")
                .long_about(
"Dice Pool mode: Each die of a roll is independent of the others")
        )
        .arg(
            Arg::new("default_target")
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
        .arg(Arg::new("v")
            .short('v')
            .multiple(true)
            .about("Sets the level of verbosity")
        )

        .get_matches();

    println!("{:#?}",opts);
}
