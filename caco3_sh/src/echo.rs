pub fn echo(input: String) {
    let echo_string = input.trim_start_matches("\"").trim_end_matches("\"");
    let echo_string = echo_string.trim_start_matches("\'").trim_end_matches("\'");
    println!("{}", echo_string.trim());
}