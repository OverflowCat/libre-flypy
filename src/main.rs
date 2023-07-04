mod ids;
pub mod pinyin;
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
    let ids_lv2 = std::fs::File::open("./ids/ids_lv2.txt").expect("File not found.");
    let lv2_reader = BufReader::new(ids_lv2);
    let my_ids = std::fs::File::open("./my_ids.txt").expect("File not found.");
    let my_reader = BufReader::new(my_ids);
    for line in lv2_reader.lines().chain(my_reader.lines()) {
        let mut line = line.expect("Line not found.");
        if let Some(i) = line.find("//") {
            if i == 0 {
                continue;
            }
            line.truncate(i);
        }
        let (character, sequence) = ids::parse_ids(line);
        todos.push((character, sequence));
    }
    let mut prev_len = 0;
    loop {
        todos = todos
            .into_iter()
            .rev()
            .filter_map(|(character, tree)| {
                if done.contains_key(&character) {
                    return None;
                }
                let first_part = tree.get_first_leaf();
                let last_part = tree.get_last_leaf();
                let first_code = firsts.get(&first_part);
                let last_code = lasts.get(&last_part);
                if first_code.is_some() && last_code.is_some() {
                    let mut first_code = (*first_code.unwrap()).clone();
                    let mut last_code = (*last_code.unwrap()).clone();
                    // println!("{}: {}{} {}{}", character, first_part, last_part, first_code, last_code);
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
    print!("未成功：{} // 成功: {} // ", todos.len(), prev_len);

    let pinyin_data = pinyin::build_hashmap();
    let mut done_with_pinyin = Vec::with_capacity(done.len());
    for (character, code) in done {
        if let Some(pinyin) = pinyin_data.get(&character) {
            for x in pinyin {
                done_with_pinyin.push((character, format!("{}{}", x.into_xnhe(), code)));
            }
        } else {
            // no pinyin data found
            done_with_pinyin.push((character, format!("__{}", code)));
            continue;
        }
    }
    println!("全部成功: {}", done_with_pinyin.len());

    let output = std::fs::File::create("./output.txt").expect("File not found.");
    let mut writer = std::io::BufWriter::new(output);
    for (character, code) in done_with_pinyin {
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
    let time = std::time::Instant::now();
    parse_file();
    let elapsed = time.elapsed();
    println!(
        "Elapsed: {}.{:03} seconds",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );
}
