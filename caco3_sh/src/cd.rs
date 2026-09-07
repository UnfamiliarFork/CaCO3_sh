use std::env;
use std::path::Path;

pub fn change_directory(path: String, cur_path: String) -> u8 {
    let edit_path = path.trim_start_matches("\"").trim_end_matches("\"");
    let join_path = format!("{}{}", cur_path.to_owned(), edit_path);
    let slash_path = format!("{}{}{}", cur_path.to_owned(), "/", edit_path);

    if Path::new(edit_path).exists() == true {
        let change = env::set_current_dir(edit_path);
        let _change_match = match change {
            Ok(_ok) => return 0,
            Err(_error) => return 2,
        };
    }
    if Path::new(join_path.as_str()).exists() == true {
        let change = env::set_current_dir(join_path);
        let _change_match = match change {
            Ok(_ok) => return 1,
            Err(_error) => return 2,
        };
    }
    if Path::new(slash_path.as_str()).exists() == true {
        let change = env::set_current_dir(edit_path);
        let _change_match = match change {
            Ok(_ok) => return 1,
            Err(_error) => return 2,
        };
    }
    else {
        return 2;
    }
}

