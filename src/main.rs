mod ids;
mod roots;

use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
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
        let (character, sequence) = ids::parse_ids(&line);
        todos.push((character, sequence));
    }
    let mut prev_len = 0;
    loop {
        todos = todos
            .into_iter()
            .filter_map(|(character, tree)| {
                let first_part = tree.get_first_leaf();
                let last_part = tree.get_last_leaf();
                let first_code = firsts.get(&first_part);
                let last_code = lasts.get(&last_part);
                if first_code.is_some() && last_code.is_some() {
                    let first_code = first_code.unwrap();
                    let last_code = last_code.unwrap();
                    println!(
                        "{}: {}{} {}{}",
                        character, first_part, last_part, first_code, last_code
                    );
                    done.insert(character, format!("{}{}", first_code, last_code));
                    firsts.insert(character, *first_code);
                    lasts.insert(character, *last_code);
                    None
                } else {
                    // println!("{}: {}{} not found", character, first_part, last_part);
                    Some((character, tree))
                }
            })
            .collect();
        let curr_len = todos.len();
        if curr_len == prev_len {
            break;
        }
        prev_len = curr_len;
    }
    println!("{:?}", done);
    println!("总计: {}", prev_len);
}

fn main() {
    println!("Hello, world!");
    parse_file();
}
