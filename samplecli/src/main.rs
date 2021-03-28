use clap::Clap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clap, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Your name",
    about = "Super awesome sample RPN calculator"
)]

struct Opts {
    // Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    // #[clap(name = "NUMBER")]
    // num: i32,

    // Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,

}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);

        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);
        }
    }
    
}

// use clap::{App, Arg};

// fn main() {
//     let matches = App::new("My RPN program")
//     .version("1.0.0")
//     .author("t o")
//     .about("Super awesome sample RPN calculator")
//     .arg(
//         Arg::new("formula_file")
//         .about("Formulas written in RPN")
//         .value_name("FILE")
//         .index(1)
//         .required(false),
//     )
//     .arg(
//         Arg::new("verbose")
//         .about("Sets the level of verbosity")
//         .short('v')
//         .long("verbose")
//         .required(false),
//     )
//     .get_matches();

//     match matches.value_of("formula_file") {
//         Some(file) => println!("File speciried: {}", file),
//         None => println!("No file specified."),
//     }
//     let verbose = matches.is_present("verbose");
//     println!("Is verbosity specified?: {}", verbose);
// }