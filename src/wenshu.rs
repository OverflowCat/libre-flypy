use ascii::AsAsciiStr;
use core::panic;
use rusqlite::Connection;
use std::collections::HashMap;
pub mod flypy;

#[derive(Debug, Clone)]
pub struct ChaiziRow {
    z: String, // zi
    s: String, // sequence
}

impl ChaiziRow {
    #[inline]
    fn iter(&self) -> std::str::Split<char> {
        self.s.split(SEPARATOR)
    }
    fn count(&self) -> usize {
        self.s.chars().filter(|&x| x == SEPARATOR).count()
    }
    #[inline]
    fn first(&self) -> String {
        self.iter().next().unwrap().to_string()
    }
    #[inline]
    fn last(&self) -> String {
        self.iter().last().unwrap().to_string()
    }
}

const SEPARATOR: char = ',';

fn build_wenshu() -> Vec<ChaiziRow> {
    let conn = Connection::open("db/wenshu.sqlite").unwrap();
    let mut stmt = conn
        .prepare("SELECT ZI, GROUP_CONCAT(CHAI) FROM CHAIZI GROUP BY ZI;")
        .unwrap(); // It's worth noting that the result of using GROUP_CONCAT may be truncated if the result is too long, // and you can use the GROUP_CONCAT(CHAI, separator) and set the separator to join the strings.
    let zis = stmt
        .query_map([], |row| {
            Ok(ChaiziRow {
                z: row.get(0)?,
                s: row.get(1)?,
            })
        })
        .unwrap();
    zis.map(|x| x.unwrap()).collect()
}

fn main() {
    let flypy_data = flypy::parse_codes();
    let flypy_len = flypy_data.len();
    let mut parts = flypy::get_parts(&flypy_data);
    let extras = vec![
        ("⺝", 'o'),
        ("卅", 'f'),
        ("𧾷", 'z'),
        ("𥫗", 'v'),
        ("龰", 'r'),
        ("𫜹", 'e'),
    ];
    for (k, v) in extras.into_iter() {
        parts.insert(k.to_owned(), v);
    }
    println!("{:?}", parts);
    // return;
    let mut flypy: HashMap<String, Vec<String>> = HashMap::new();
    for zi in flypy_data {
        flypy
            .entry(zi.character)
            .or_insert(Vec::with_capacity(1))
            .push(zi.fly_code.slice_ascii(..4).unwrap().to_string());
    }

    let wenshu = build_wenshu();
    let mut wenshu_map = HashMap::new();
    for x in wenshu.iter() {
        wenshu_map.insert(x.z.clone(), x.s.clone());
    }

    fn find_first(
        zir: &ChaiziRow,
        parts: &HashMap<String, char>,
        wenshu: &HashMap<String, String>,
    ) -> Option<String> {
        let mut zi = zir.clone();

        let mut loop_counter = 0;
        loop {
            if loop_counter > 10 {
                print!("🟡 {} -> {} 递归 || ", zir.z, zi.z);
                break None;
            }
            loop_counter += 1;
            let part = &zi.first();
            if let Some(x) = parts.get(part) {
                break Some((&x).to_string());
            }
            if let Some(o) = wenshu.get(part) {
                if part == o {
                    break None;
                } // 防止死循环
                zi = ChaiziRow {
                    z: zi.first().clone(),
                    s: o.clone(),
                };
            } else {
                break None;
            }
        }
    }

    fn find_last(
        zir: &ChaiziRow,
        parts: &HashMap<String, char>,
        wenshu: &HashMap<String, String>,
    ) -> Option<String> {
        let mut zi = zir.clone();
        loop {
            let part = &zi.last();
            if let Some(x) = parts.get(part) {
                break Some(x.to_string());
            }
            if let Some(o) = wenshu.get(part) {
                if part == o {
                    break None;
                } // 防止死循环
                zi = ChaiziRow {
                    z: zi.last().clone(),
                    s: o.clone(),
                };
            } else {
                break None;
            }
        }
    }

    let (mut correct, mut incorrect, mut cannot_build, mut extra) = (0, 0, 0, 0);
    let all_count = wenshu.len();
    let now = std::time::Instant::now(); // benchmark
    for zi in wenshu {
        let xingma;
        match (
            find_first(&zi, &parts, &wenshu_map),
            find_last(&zi, &parts, &wenshu_map),
        ) {
            (Some(first), Some(last)) => {
                xingma = format!("{}{}", first, last);
            }
            _ => {
                cannot_build += 1;
                continue;
            }
        }
        if let Some(codes) = flypy.get(&zi.z) {
            let mut flag = false;
            for code in codes {
                if code.ends_with(&xingma) {
                    print!("✅ {} {} == {} || ", zi.z, xingma, code);
                    correct += 1;
                    flag = true;
                    break;
                }
            }
            // dbg!(&zi.z, &codes, &xingma);
            if !flag {
                incorrect += 1;
                print!("⛔ {} {} != {} || ", zi.z, xingma, codes[0]);
            }
        } else {
            // not in flypy scheme
            // println!("➕ {}, wenshu built {}, not in flypy official scheme.", zi.z, xingma);
            extra += 1;
        }
    }
    println!("Elapsed: {:.2?}", now.elapsed());
    println!(
        "\nAs of {all_count} characters, {correct} correct, {incorrect} incorrect, {extra} new, {cannot_build} cannot be built."
    );
    let correct_rate = correct as f32 / (correct + incorrect) as f32;
    let cov_rate = correct as f32 / flypy_len as f32;
    println!(
        "可生成的小鹤音形表内字的正确率为 {:.2}%，相当于覆盖了表内字的 {:.2}%",
        correct_rate * 100.,
        cov_rate * 100.
    );
}
