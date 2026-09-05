use std::env;

pub fn pwd() -> String {
    // Literally all this does is return the current directory...
    let dir = env::current_dir(); // Get current directory as PathBuf
    let dir = dir.unwrap().as_os_str().to_str().unwrap().to_string(); //Convert to String

    // Remove those double/single quotes
    let dir = dir.trim_start_matches("\"").trim_end_matches("\""); 
    let dir = dir.trim_start_matches("\'").trim_end_matches("\'");

    return dir.to_string();
}