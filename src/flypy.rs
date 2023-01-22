use ascii::AsciiChar;
use ascii::AsciiStr;
use serde::de::{self, Visitor};
use serde::Deserialize;
use serde_json::Deserializer;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;

type FlyCode = [AsciiChar; 4];
pub struct FlyCodeVisitor {}

/* impl<'de> Visitor<'de> for FlyCodeVisitor {
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Space separated 4-letter codes")
    }

    fn visit_str<E>(self, value: &str) -> Result<Vec<FlyCode>, E>
    where
        E: de::Error,
    {
        Ok(vec![[
            AsciiChar::P,
            AsciiChar::P,
            AsciiChar::P,
            AsciiChar::P,
        ]])
    }

    type Value = Vec<FlyCode>;
}

impl<'de> Deserialize<'de> for Vec<FlyCode> {
    fn deserialize<D>(deserializer: D) -> Result<i32, ()>
    where
        D: Deserialize<'de>,
    {
        deserializer.deserialize_str(FlyCodeVisitor {})
    }
} */

#[derive(Debug, Deserialize)]
pub struct FlypyChar {
    pub character: String,
    level: String, // "1" or "2"
    pub fly_code: String,
    order: String,
    pub first_part: String,
    first_py: String,
    pub last_part: String,
    last_py: String,
}

impl FlypyChar {
    fn codes(&self) -> Vec<&str> {
        self.fly_code.split_ascii_whitespace().collect()
    }
}

pub fn check_fly_code(data: &Vec<FlypyChar>) {
    assert!(data.iter().any(|ch| { ch.fly_code != "3" }));
}

pub fn parse_codes() -> Vec<FlypyChar> {
    std::env::set_var("RUST_BACKTRACE", "1");
    let reader = fs::File::open("./flypy/小鹤音形单字全码码表.json").unwrap();
    let data: Vec<FlypyChar> =
        serde_json::from_reader(reader).expect("JSON was not well-formatted");
    data
}

pub fn get_separated_parts(
    data: &Vec<FlypyChar>,
) -> (HashMap<String, String>, HashMap<String, String>) {
    let mut first_parts = HashMap::new();
    let mut last_parts = HashMap::new();
    for zi in data {
        assert_eq!(
            zi.first_py.clone(),
            *first_parts
                .entry(zi.first_part.clone())
                .or_insert(zi.first_py.clone())
        );
        assert_eq!(
            zi.last_py.clone(),
            *last_parts
                .entry(zi.last_part.clone())
                .or_insert(zi.last_py.clone())
        );
        {
            /*   {
              "character": "壬",
              "level": "1",
              "fly_code": "rfpu",
              "order": "丿　士",
              "first_part": "丿",
              "first_py": "p",
              "last_part": "士",
              "last_py": "u"
            },
            {
              "character": "壬",
              "level": "1",
              "fly_code": "rfqa",
              "order": "千　一",
              "first_part": "千",
              "first_py": "q",
              "last_part": "一",
              "last_py": "a"
            }, */
        }
    }
    (first_parts, last_parts)
    // ({"匚": "k", "乌": "w", "业": "y", "乂": "x", "丽": "l", "与": "y", "木": "m", "酉": "y", "百": "b", "刀": "d", "虍": "h", "亻": "r", "日": "o", "冂": "k", "气": "q", "龵": "u", "西": "x", "艹": "c", "爿": "p", "几": "j", "臣": "i", "生": "u", "我": "w", "比左": "b", "氵": "d", "饣": "u", "久": "j", "口": "k", "文": "w", "永": "y", "丘": "q", "黑": "h", "夫": "f", "血": "x", "非": "f", "了": "l", "不": "b", "艮": "g", "中": "v", "乡": "x", "户": "h", "耳": "e", "巳": "s", "忄": "x", "女": "n", "具上": "q", "雨": "y", "立": "l", "囗": "k", "乃": "n", "朱": "v", "丙": "b", "才": "c", "上": "u", "疒": "b", "巴": "b", "犬": "q", "彳": "i", "白": "b", "川": "i", "禺": "y", "石": "u", "七": "q", "儿": "e", "缶": "f", "出": "i", "又": "y", "舟": "v", "王": "w", "田": "t", "⻊": "z", "午": "w", "门": "m", "夭": "y", "内": "n", "广": "g", "革": "g", "己": "j", "屯": "t", "人": "r", "工": "g", "扌": "f", "辶": "z", "弋": "y", "火": "h", "龹": "j", "求": "q", "丨": "l", "斤": "j", "瓜": "g", "厂": "i", "太": "t", "束": "u", "下": "x", "夕": "x", "其上": "q", "更": "g", "丁": "d", "习": "x", "尹": "y", "世": "u", "彐": "e", "弗": "f", "牛": "n", "自": "z", "犭": "q", "止": "v", "牙": "y", "壬": "r", "勹": "b", "士": "u", "目": "o", "氏": "u", "带上": "f", "羊": "y", "卵": "l", "于": "y", "千": "q", "纟": "s", "龙": "l", "讠": "y", "二": "e", "八": "b", "必": "b", "巾": "j", "即左": "g", "丿": "p", "夂": "w", "虫": "i", "厶": "s", "丷": "b", "半": "b", "乍": "v", "阝": "e", "幺": "y", "禾": "h", "歹": "d", "矢": "u", "戈": "g", "⺍": "x", "天": "t", "乛": "v", "甫": "f", "也": "y", "申": "u", "⺶": "y", "五": "w", "亠": "w", "兀": "w", "冖": "b", "戌": "x", "车": "i", "万": "w", "钅": "j", "丰": "f", "韦": "w", "⺈": "d", "小": "x", "寿上": "f", "且": "q", "⺌": "x", "戋": "j", "骨": "g", "衤": "p", "⺮": "v", "九": "j", "由": "y", " 十": "u", "𠂇": "u", "方": "f", "叉": "i", "大": "d", "尢": "y", "龴": "s", "米": "m", "里": "l", "开": "k", "廿": "n", "卩": "e", "身": "u", "豸": "v", "丶": "d", "冉": "r", "果": "g", "冫": "d", "礻": "p", "月": "o", "鸟": "n", "刃": "r", "匕": "b", "亚": "y", "正": "v", "册": "c", "鬼": "g", "罒": "s", "子": "z", "入": "r", "廴": "z", "干": "g", "丕": "p", "甘": "g", "鱼": "a", "一": "a", "戊": "w", "片": "p", "弓": "g", "山": "e", "龶": "f", "水": "u", "成": "i", "夹": "j", "凹": "a", "耒": "l", "少": "u", "马": "m", "氐": "d", "丝": "s", "勿": "w", "宀": "b", "臼": "j", "串": "i", "亡": "w", "⺷": "y", "金": "j", "未": "w", "民": "m", "毛": "m", "耂": "l", "甲": "j", "东": "d", "而": "e", "毌": "g", "面": "m", "肃": "s", "土": "t", "⺧": "n", " 爪": "v", "尸": "u", "尺": "i", "央": "y", "豕": "u", "产": "i", "禹": "y", "丹": "d", "力": "l", "来": "l"}, {"韦": "w", "击": "j", "幺": "y", "二": "e", "巨": "j", "厂": "i", "己": "j", "冫": "d", "电": "d", "乏": "f", "小": "x", "兀": "w", "禹": "y", "册": "c", "灬": "h", "大": "d", "才": "c", "斥": "i", "血": "x", "臣": "i", "亻": "r", "末": "m", "女": "n", "勺": "u", "矢": "u", "申": "u", "金": "j", "耒": "l", "升": "u", "了": "l", "肃": "s", "于": "y", "戍": "u", "歹": "d", "乐": "l", "木": "m", "止": "v", "戊": "w", "求": "q", "车": "i", "廾": "c", "毛": "m", "子": "z", "半": "b", "术": "u", "⺗": "x", "史": "u", "巳": "s", "七": "q", "门": "m", "少": "u", "主": "v", "尢": "y", "乍": "v", "不": "b", "业": "y", "夹": "j", "臼": "j", "𧘇": "y", "出": "i", "一": "a", "瓦": "w", "东": "d", "文": "w", "丙": "b", "手": "u", "缶": "f", "皿": "m", "韭": "j", " 余下": "h", "之": "v", "儿": "e", "下": "x", "八": "b", "与": "y", "弗": "f", "刀": "d", "耳": "e", "叉": "i", "日": "o", "刃": "r", "黑": "h", "乂": "x", "刂": "d", "朱": "v", "乎": "h", "夫": "f", "氺": "u", "尹": "y", "朩": "m", "人": "r", "三": "s", "丰": "f", "丹": "d", "羌": "q", "丁": "d", "民": "m", "见下": "e", "白": "b", "永": "y", "彐": "e", "吏": "l", "中": "v", "里": "l", "又": "y", "鱼": "a", "尸": "u", "农": "n", "丽": "l", "为": "w", "王": "w", "丈": "v", "乃": "n", "千": "q", "本": "b", "卩": "e", "豕": "u", "具上": "q", "雨": "y", "用": "y", "两": "l", "重": "v", "丝": "s", "屯": "t", "垂": "i", "丸": "w", "冉": "r", "井": "j", "亏": "k", "卞": "b", "开": "k", "厶": "s", "生": "u", "专": "v", "丕": "p", "玉": "y", "太": "t", "柬": "j", "果": "g", "田": "t", "石": "u", "曲": "q", "乛": "v", "夭": "y", "口": "k", "工": "g", "禾": "h", "氏": "u", "龶": "f", "斤": "j", "母": "m", "失": "u", "百": "b", "必": "b", "虫": "i", "亡": "w", "月": "o", "攵": "w", "舟": "v", "刁": "d", "巴": "b", "办": "b", "水": "u", "𠂇": "u", "聿": "y", "广": "g", "天": "t", "成": "i", "夕": "x", "乌": "w", "廴": "z", "匚": "k", "壬": "r", "平": "p", "龴": "s", "阝": "e", "气": "q", "世": "u", "互": "h", "瓜": "g", "心": "x", "丨": "l", "良": "l", "弋": "y", "习": "x", "戋": "j", "凡": "f", "来": "l", "山": "e", "非": "f", "卜": "b", "丐": "g", "羊": "y", "骨": "g", "士": "u", "丫": "y", "无": "w", "严": "y", "及": "j", "凵": "k", "力": "l", "㠯": "k", "我": "w", "州": "v", "义": "y", "乞": "q", "爪": "v", "巾": "j", "束": "u", "曳": "y", "万": "w", "串": "i", "面": "m", "犬": "q", "牙": "y", "丏": "m", "未": "w", " 甲": "j", "夂": "w", "丿": "p", "土": "t", "马": "m", "亍": "i", "艮": "g", "正": "v", "鸟": "n", "久": "j", "由": "y", "自": "z", "产": "i", "禺": "y", "且": "q", "丘": "q", "鬼": "g", "毋": "w", "ㄟ": "n", "牛": "n", "辶": "z", "尺": "i", "夬": "g", "匕": "b", "长": "i", "尤": "y", "几": "j", "丑": "i", "川": "i", "内": "n", "更": "g", "革": "g", "午": "w", "西": "x", "火": "h", "艹": "c", "臾": "y", "十": "u", "米": "m", "勿": "w", "龙": "l", "寸": "c", "上": "u", "弓": "g", "丶": "d", "氐": "d", "方": "f", "央": "y", "亚": "y", "酉": "y", "戈": "g", "目": "o", "九": "j", "也": "y", "立": "l", "干": "g", "甫": "f", "夷": "y", "五": "w", "么": "m", "乡": "x", "曰": "o", "而": "e", "户": "h", "甘": "g"})
}

pub fn get_parts(data: &Vec<FlypyChar>) -> HashMap<String, String> {
    let mut parts = HashMap::new();
    for zi in data {
        assert_eq!(
            zi.first_py.clone(),
            *parts
                .entry(zi.first_part.clone())
                .or_insert(zi.first_py.clone())
        );
        assert_eq!(
            zi.last_py.clone(),
            *parts
                .entry(zi.last_part.clone())
                .or_insert(zi.last_py.clone())
        );
    }
    parts
}

fn main() {
    let data = parse_codes();
    check_fly_code(&data);
    get_parts(&data);
}
