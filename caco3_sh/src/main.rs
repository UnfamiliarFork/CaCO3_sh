use std::env;
use std::io::{self, Write};
// Library that lets me learn more about you
use whoami;
// inbuilt functions
mod echo; // The echo command
mod pwd; // The pwd command

fn main() -> std::io::Result<()> {
    // Get info about the user and computer
    let user_name = whoami::account().unwrap_or_else(|_| "<unknown>".to_string());
    let computer_name = whoami::devicename().unwrap_or_else(|_| "<unknown>".to_string());

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

        // Match what the user wants to do with functions
        if input[0].to_lowercase() == "echo" {
            // Check that the user didnt do something nefarious
            if input.len() == 1 {
                println!("echo couldn't run because no input was given");
            } else {
                echo::echo(input[1..].join(" ").to_string());
            }
        }
        else if input[0].to_lowercase() == "pwd" {
            // This literally just prints the working directory
            println!("{}", pwd::pwd());
        }
        else if input[0].to_lowercase() == "exit" {
            // Break the loop, causing program to exit
            break;
        } else if input[0].to_lowercase() == "clear" {
            // Strange terminal code that clears the screen
            print!("\x1B[2J\x1B[1;1H");
        } else {
            println!("That function was not found.")
        }
    }
    Ok(())
}
