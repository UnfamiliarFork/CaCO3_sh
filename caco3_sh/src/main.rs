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

       let input_0 = input[0].to_lowercase();
       match input_0.as_str() {
        // echo: repeats the user
        "echo" => {
            if input.len() == 1 { // Checks that echo can actually echo something
                println!("echo couldn't run because no input was given");
            } else {
                echo::echo(input[1..].join(" ").to_string());
            }},
        // pwd: Prints current directory
        "pwd" => println!("{}", pwd::pwd()),
        // exit: closes CaCO3
        "exit" => break,
        // clear: clears terminal screen
        "clear" => print!("\x1B[2J\x1B[1;1H"), // Strange terminal code that clears it. Found off stack exchange
        // Just in case anythiung else happened
        _other => println!("That function was not found.")
       }
    }
    println!("See ya next time!");
    Ok(())
}
