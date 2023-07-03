mod ids;
mod roots;

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
};

use ids::Tree;

pub fn parse_file() {
    let (mut firsts, mut lasts) = roots::generate();
    let mut done: HashMap<char, String> = HashMap::new();
    let mut todos: Vec<(char, Tree)> = Vec::new();
    // read ./ids/ids_lv2.txt line by line
    let file = std::fs::File::open("./ids/ids_lv2.txt").expect("File not found.");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Line not found.");
        let (character, sequence) = ids::parse_ids(line);
        todos.push((character, sequence));
    }
    let mut prev_len = 0;
    loop {
        todos = todos
            .into_iter()
            .rev()
            .filter_map(|(character, tree)| {
                let first_part = tree.get_first_leaf();
                let last_part = tree.get_last_leaf();
                let first_code = firsts.get(&first_part);
                let last_code = lasts.get(&last_part);
                if first_code.is_some() && last_code.is_some() {
                    let mut first_code = (*first_code.unwrap()).clone();
                    let mut last_code = (*last_code.unwrap()).clone();
                    println!(
                        "{}: {}{} {}{}",
                        character, first_part, last_part, first_code, last_code
                    );
                    done.insert(character, format!("{}{}", first_code, last_code));
                    if first_part == '辶' || first_part == '廴' {
                        (first_code, last_code) = (last_code, first_code);
                    }
                    if !firsts.contains_key(&character) {
                        firsts.insert(character, first_code.clone());
                    }
                    if !lasts.contains_key(&character) {
                        lasts.insert(character, last_code.clone());
                    }
                    None
                } else {
                    if first_code.is_some() && !firsts.contains_key(&character) {
                        firsts.insert(character, first_code.unwrap().clone());
                    }
                    if last_code.is_some() && !lasts.contains_key(&character) {
                        lasts.insert(character, last_code.unwrap().clone());
                    }
                    // println!("{}: {}{} not found", character, first_part, last_part);
                    Some((character, tree))
                }
            })
            .collect();
        let curr_len = done.len();
        if curr_len == prev_len {
            break;
        }
        prev_len = curr_len;
    }
    println!("{:?}", done);
    println!("{:?}", todos.len());
    println!("总计: {}", prev_len);

    let output = std::fs::File::create("./output.txt").expect("File not found.");
    let mut writer = std::io::BufWriter::new(output);
    for (character, code) in done {
        writer
            .get_mut()
            .write(format!("{}\t{}\n", character, code).as_bytes())
            .expect("Write failed.");
    }
    let missing = std::fs::File::create("./missing.txt").expect("File not found.");
    let mut writer = std::io::BufWriter::new(missing);
    for (character, tree) in todos {
        writer
            .get_mut()
            .write(format!("{}\t{:?}\n", character, tree).as_bytes())
            .expect("Write failed.");
    }
}

fn main() {
    parse_file();
}
