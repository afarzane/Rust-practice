
use text_colorizer::*; // the ::* helps import everything from text-colorizer
use regex::Regex; // Will help us create replace/find fucntions
use std::env; // Used for inspecting environmental variables, process arguments, and etc.
use std::fs; // Used for reading and writing files

#[derive(Debug)] // Helps print the variables in a struct in a debugger-friendly way
struct Arguments{
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn parse_arguments() -> Arguments{

    let args: Vec<String> = env::args().skip(1).collect(); // Grab arguments from user

    if args.len() != 4 { // If there isnt exactly 4 inputs...
        print_usage();
        eprintln!("{} wrong number of arguments: expected {} arguments, got {}", "Error: ".red().bold(), "4".green(), ((args.len()).to_string()).red());
        std::process::exit(1);
    }

    Arguments { target: args[0].clone(), 
        replacement: args[1].clone(), 
        filename: args[2].clone(), 
        output: args[3].clone() 
    }

}

fn print_usage(){
    eprintln!("{} - change occurrences of one string into another", "quickreplace".green()); // the ".green()" will print the string in green 
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?; // Provide target to Regex
    Ok(regex.replace_all(text, replacement).to_string())
}

fn main() {
    let args = parse_arguments();
    let data = match fs::read_to_string(&args.filename){ // Read data from file
            Ok(v) => v,
            Err(e) => {eprintln!("{} failed to read from file '{}': {:?}", "Error:".red().bold(), args.filename, e);
            std::process::exit(1);}
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data){
        Ok(v) => v,
        Err(e) => {eprintln!("{} failed to replace text: {:?}", "Error:".red().bold(), e);
        std::process::exit(1);}
    };

    match fs::write(&args.output, &replaced_data){
        Ok(_) => {},
        Err(e) => {eprintln!("{} failed to write file '{}': {:?}", "Error: ".red().bold(), args.filename, e);
        std::process::exit(1);}
    };

    println!("{:?}", args)
}

// ---------------------------- Notes ---------------------------- //
// When declaring dependencies, using "1" as a version number will grab the latest version of a crate before 2.0

// {:?} is used in functions like println! or format! to output the values of variables on the command line

// .skip(n) iterates over n elements of iter

// .collect() produces a vector of the inputted arguments 

// The "diff" command allows you to compare two files and see differences
