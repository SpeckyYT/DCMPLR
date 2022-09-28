use std::fs::File;
use std::io::Read;

use crossterm::style::Stylize;

mod object;
mod load_savefile;
mod parse_level;
mod compiler;

fn main() {
    let path = load_savefile::local_levels_path();
    let mut file = File::open(path).expect("what");
    let mut file_content = vec![];

    file.read_to_end(&mut file_content).expect("Failed reading file");

    let full_level = load_savefile::get_level_string(file_content, None).expect("Failed to get level");

    let (_, level) = full_level.split_once("kA11,0;").expect("Failed to get level");

    println!("{}", level);

    let level = parse_level::parse_level(level);

    let (spwn, other) = level.split_spwn();

    let spwn = compiler::compile(spwn);
    let other = compiler::compile(other);

    println!("{}", spwn.green());
    println!("{}", other.magenta());
}
