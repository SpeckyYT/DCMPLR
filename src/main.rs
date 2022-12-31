use std::fs::File;
use std::io::Read;

use compiler::Compiler;
use crossterm::style::Stylize;
use processing::Tree;

mod compiler;
mod load_savefile;
mod object;
mod parse_level;
mod processing;
mod trigger;

fn main() {
    let path = load_savefile::local_levels_path();
    let mut file = File::open(path).expect("what");
    let mut file_content = vec![];

    file.read_to_end(&mut file_content)
        .expect("Failed reading file");

    let full_level = load_savefile::get_level_string(file_content, Some(&"test".into()))
        .expect("Failed to get level");

    let (_, level) = full_level
        .split_once("kA11,0;")
        .expect("Failed to get level");

    // println!("{}", level);

    let level = parse_level::parse_level(level);

    let mut tree = Tree::default();

    tree.build(&level.objects);

    let mut compiler = Compiler::default();

    println!("{}", compiler.compile(tree));

    // let (spwn, other) = level.split_spwn();

    // let spwn = processing::FeetAss::new(spwn);
    // let other = processing::FeetAss::new(other);

    // // STATIC OBJECTS
    // if spwn.static_objects.len() + other.static_objects.len() > 0 {
    //     let spwn_static = compiler::compile(spwn.static_objects);
    //     let other_static = compiler::compile(other.static_objects);

    //     println!("{}", "extract obj_props\n".red());
    //     println!("{}", spwn_static.green());
    //     println!("{}", other_static.magenta());
    // }

    // // dynamic
    // let spwn_dynamic = spwn.groups;
    // let other_dynamic = other.groups;
    // // ^ not sure what to do with these mfs
}
