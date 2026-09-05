use std::env;

pub fn pwd() -> std::io::Result<()> {
    // Literally all this does is return the current directory...
    let dir = env::current_dir()?;
    return dir;
}