use std::env;
use std::io::{self, Write};
// Library that lets me learn more about you
use whoami;
// YAY! Colors now exist!
use owo_colors::OwoColorize;
// inbuilt functions
mod echo; // The echo command
mod pwd; // The pwd command
mod cd;

fn main() -> std::io::Result<()> {
    // Get info about the user and computer
    let user_name = whoami::account().unwrap_or_else(|_| "<unknown>".to_string());
    let computer_name = whoami::devicename().unwrap_or_else(|_| "<unknown>".to_string());

    // Funny Ascii-art
    println!("\t{}", "Welcome to...".red());
    println!("{}", r"         ___           ___    _____    ___ 
        (  _`\        (  _`\ (  _  ) /'_  )
        | ( (_)   _ _ | ( (_)| ( ) |(_)_) |
        | |  _  /'_` )| |  _ | | | | _(_ < 
        | (_( )( (_| || (_( )| (_) |( )_) |
        (____/'`\__,_)(____/'(_____)`\____)".green());
    println!("{}\n", "  The world's worst shell! Made by UnfamilarFork".blue());

    // main loop
    loop {
        // get the user's current directory
        let current_path = env::current_dir()?;

        // Prints the user's name, computer name, and current directory
        print!("{user_name} @ {computer_name} {} ", current_path.display());
        let _ = io::stdout().flush(); // Actually print the thing

        // Part where I get what the user wants to do
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Dude, I think the program just crashed. WHAT DID YOU ENTER?");
        
        // Split what the user entered
        let input: Vec<&str> = input.trim().split(" ").collect();

       let input_0 = input[0].to_lowercase();
       match input_0.as_str() {
        // echo: repeats the user
        "echo" => {
            if input.len() == 1 { // Checks that echo can actually echo something
                println!("{}", "echo couldn't run because no input was given".red());
            } else {
                echo::echo(input[1..].join(" ").to_string());
            }},
        // pwd: Prints current directory
        "pwd" => println!("{}", pwd::pwd()),
        // cd:change directory
        "cd" => {
            let cd_result = cd::change_directory(input[1..].join(" ").to_string(), current_path.display().to_string());
            if cd_result == 0 || cd_result == 1 || cd_result == 2 {
                continue;
            }
            else {
                println!("{}", "The directory you are trying to change into is invalid or couldn't be found by cd.".red())
            }
        }
        // exit: closes CaCO3
        "exit" => break,
        // clear: clears terminal screen
        "clear" => print!("\x1B[2J\x1B[1;1H"), // Strange terminal code that clears it. Found off stack exchange
        "" => continue, // Empty text just continues the loop
        // Just in case anything else happened
        other => println!("\"{}\" {}", other.red(), "does not seem to be the name for any function currently avaliable.".red())
       }
    }
    println!("See ya next time!");
    Ok(())
}
