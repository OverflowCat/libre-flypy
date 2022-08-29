use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

struct Structure {
    symbol: char,
    count: u8,
}

impl Structure {
    fn new(symbol: char, count: u8) -> Structure {
        Structure { symbol, count }
    }
}

fn main() {
    let file = File::open("ids/ids_lv2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    let mut parsed = 0;
    reader.lines().for_each(|line| match line {
        Ok(entry) => {
            let res = parse(&entry);
            if res.len() > 3 {
                parsed += 1;
                println!("{:?}", res);
            }
            total += 1;
        }
        Err(msg) => {
            println!("Err: {:?}", msg);
            total += 1;
        }
    });
    println!("Parsed {} entries in {}.", parsed, total);
}

fn parse(entry: &str) -> String {
    let structures = vec![
        Structure::new('⿰', 2),
        Structure::new('⿱', 2),
        Structure::new('⿳', 3),
    ];

    let mut columns: VecDeque<&str> = entry.split('\t').collect();
    if columns.len() < 2 {
        return "".to_string();
    }
    let character = columns.pop_front().unwrap();
    let v: Vec<String> = columns
        .iter()
        .filter_map(|&s| {
            let s = if s.contains(";") {
                let v: Vec<&str> = s.split(";").collect();
                v[0]
            } else {
                s
            };
            let chars: Vec<char> = s.chars().collect();
            let mut flag = false;
            for structure in &structures {
                if structure.symbol == chars[0] && chars.len() == structure.count as usize + 1 {
                    flag = true;
                    break;
                }
            }
            if flag {
                Some(s.to_string())
            } else {
                None
            }
        })
        .collect();
    if v.len() > 0 {
        format!("{}: {}", character, v.join(" / "))
    } else {
        "".to_string()
    }
}
