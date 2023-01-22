use pinyin::{ToPinyin, ToPinyinMulti};
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
    ids();
}

fn ids() {
    let file = File::open("ids/ids_lv2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    let mut parsed = 0;
    reader.lines().for_each(|line| match line {
        Ok(entry) => {
            let res = parse(&entry);
            if res.len() > 3 {
                let mut flag = false;
                for 多音 in res.chars().next().unwrap().to_pinyin_multi() {
                    for 音 in 多音 {
                        flag = true;
                        print!("{} ", 音.with_tone_num());
                    }
                }
                if flag {
                    parsed += 1;
                }
                println!("{}", res);
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
            structures
                .iter()
                .any(|structure| {
                    chars[0] == structure.symbol && chars.len() == structure.count as usize + 1
                })
                .then_some(s.to_string())
        })
        .collect();
    if v.len() > 0 {
        format!("{}: {}", character, v.join(" / "))
    } else {
        "".to_string()
    }
}

/* fn pinyin_to_pbyb(p: &str) -> String {
    let mut pbyb = String::new();
    let 声母 = if p.starts_with("sh") {
        'u'
    } else if p.starts_with("ch") {
        'i'
    } else if p.starts_with("zh") {
        'v'
    } else {
        p.chars().next().unwrap()
    };
}
 */
