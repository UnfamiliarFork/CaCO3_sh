use std::io::{self, Write};
use std::env;
// Library that lets me learn more about you
use whoami;
// inbuilt functions
mod echo;

fn main() -> std::io::Result<()> {
    // Get info about the user and computer
    let user_name = whoami::account().unwrap_or_else(|_| "<unknown>".to_string());
    let computer_name= whoami::devicename().unwrap_or_else(|_| "<unknown>".to_string());


    // main loop
    loop{
        let mut current_path = env::current_dir()?;
        print!("{user_name} @ {computer_name} {} ", current_path.display());
        io::stdout().flush();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Dude, I think the program just crashed. WHAT DID YOU ENTER?");

        // echo::echo(input.to_owned());
        let tokens: Vec<&str> = input.trim().split(" ").collect();
        
        if tokens[0] == "echo" {
            // Check that the user didnt do something nefarious
            if tokens.len() == 1 {
                println!("echo couldn't run because no input was given")
            }
            else{
                echo::echo(tokens[1..].join(" ").to_string());
            }
        }
        else {
            println!("That function was not found.")
        }
        continue;
    }
    Ok(())
}
