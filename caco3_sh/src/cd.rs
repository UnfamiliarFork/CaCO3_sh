use std::path::Path;
use std::env;

pub fn change_directory(path: String, cur_path: String) -> u8 {
    let edit_path = path.trim_start_matches("\"").trim_end_matches("\"");
    let join_path = format!("{}{}", cur_path.to_owned(), edit_path);
    let slash_path = format!("{}{}{}", cur_path.to_owned(), "/", edit_path);

    if Path::new(edit_path).exists() == true {
        let _change = env::set_current_dir(edit_path);
        return 0;
    }
    else if Path::new(join_path.as_str()).exists() == true {
        let _change = env::set_current_dir(edit_path);
        return 1;
    }
    else if Path::new(slash_path.as_str()).exists() == true {
            let _change = env::set_current_dir(edit_path);
        return 2;
    }
    else {
        return 3;
    }

}