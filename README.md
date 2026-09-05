# CaCO $_{3}$ Shell

This is a silly little bash recreation(~ish) I'm making to learn rust. It's mostly an amalgamation of random things I searched on the internet. It is not designed to be a complete replacement to something like [bash](https://www.gnu.org/software/bash/).

## Planned Features

- [x] echo
- [ ] pwd
- [ ] ls
- [ ] cd
- [ ] cat
- [ ] grep (maybe)
- [x] clear
- [x] exit
- [ ] basic binary execution
... and maybe more.

## Building

To build this program follow the below...

### Requirements

- [Rust/Cargo](https://rust-lang.org/)

### Instructions

1. clone this repository: `git clone https://github.com/UnfamiliarFork/CaCO3_sh.git`
2. Change directory into cargo project: `cd CaCO3_sh/caco3_sh`
3. Run `cargo build` to build the project, or `cargo run` to run the project directly.
4. To execute the program (if you did `cargo build`), run `./target/debug/caco3_sh`

Note: You can run `cargo build --release` if you want to enable release optimisations. The binary will be located at `/target/release/caco3_sh`

## Licensing

It's licensed under Apache 2.0
