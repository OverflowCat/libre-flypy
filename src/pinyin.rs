use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn build_hashmap() -> HashMap<char, Vec<Pinyin>> {
    let mine = File::open("my_pinyin.txt").unwrap();
    let file = File::open("pinyin-data/pinyin.txt").unwrap();
    let my_reader = BufReader::new(mine);
    let reader = BufReader::new(file);
    let mut map = HashMap::new();
    for line in my_reader.lines().chain(reader.lines()) {
        // dont use u+, use char after #
        let line = line.unwrap();
        let (_, pinyin_and_zi) = line.split_once(": ").unwrap();
        let Some((pinyin, zi)) = pinyin_and_zi.split_once("  # ") else {
            continue;
        };
        let pinyins = pinyin
            .split(",")
            .filter_map(|pinyin| {
                Some(
                    Pinyin::try_from(pinyin).expect(format!("Invalid pinyin: {}", pinyin).as_str()),
                )
            })
            .collect();
        if let Some(zi) = zi.chars().next() {
            map.entry(zi).or_insert_with(|| pinyins);
        }
    }
    map
}

enum Tone {
    Zero,
    First,
    Second,
    Third,
    Fourth,
}

impl TryFrom<usize> for Tone {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Tone::Zero),
            1 => Ok(Tone::First),
            2 => Ok(Tone::Second),
            3 => Ok(Tone::Third),
            4 => Ok(Tone::Fourth),
            _ => Err("Invalid tone"),
        }
    }
}

impl TryFrom<char> for Tone {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'ā' | 'ō' | 'ē' | 'ī' | 'ū' | 'ǖ' => Ok(Tone::First),
            'á' | 'ó' | 'é' | 'í' | 'ú' | 'ǘ' => Ok(Tone::Second),
            'ǎ' | 'ǒ' | 'ě' | 'ǐ' | 'ǔ' | 'ǚ' | 'ň' => Ok(Tone::Third),
            'à' | 'ò' | 'è' | 'ì' | 'ù' | 'ǜ' => Ok(Tone::Fourth),
            // 'a' | 'o' | 'e' | 'i' | 'u' | 'ü' => Ok(Tone::Zero),
            _ => Err("Invalid tone"),
        }
    }
}

impl From<&str> for Tone {
    fn from(value: &str) -> Self {
        for c in value.chars() {
            if let Ok(tone) = Tone::try_from(c) {
                return tone;
            }
        }
        Tone::Zero
    }
}

static TONE_LETTERS: [[char; 5]; 7] = [
    ['ā', 'á', 'ǎ', 'à', 'a'],
    ['ō', 'ó', 'ǒ', 'ò', 'o'],
    ['ē', 'é', 'ě', 'è', 'e'],
    ['ī', 'í', 'ǐ', 'ì', 'i'],
    ['ū', 'ú', 'ǔ', 'ù', 'u'],
    ['ǖ', 'ǘ', 'ǚ', 'ǜ', 'ü'],
    ['无', 'ń', 'ň', 'ǹ', 'n'],
];

pub enum Shengmu {
    ZeroSheng,
    B,
    P,
    M,
    F,
    D,
    T,
    N,
    L,
    G,
    K,
    H,
    J,
    Q,
    X,
    R,
    Y,
    W,
    Z,
    C,
    S,
    ZH,
    CH,
    SH,
}

pub enum Yunmu {
    ZeroYun,
    A,
    O,
    E,
    I,
    U,
    V,
    AI,
    IA,
    EI,
    UI,
    AO,
    OU,
    UO,
    IU,
    IE,
    UE,
    VE,
    ER,
    AN,
    EN,
    IN,
    UN,
    UA,
    UAI,
    UAN,
    IAO,
    IAN,
    ANG,
    ENG,
    ING,
    ONG,
    IONG,
    UANG,
    IANG,
}

impl TryFrom<&str> for Yunmu {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // replace toned letters with untoned letters
        let mut value = value.to_string();
        for (i, tone_letters) in TONE_LETTERS.iter().enumerate() {
            for tone_letter in tone_letters {
                value = value.replace(*tone_letter, TONE_LETTERS[i][4].to_string().as_str());
            }
        }
        match value.as_str() {
            "a" => Ok(Yunmu::A),
            "o" => Ok(Yunmu::O),
            "e" => Ok(Yunmu::E),
            "i" => Ok(Yunmu::I),
            "u" => Ok(Yunmu::U),
            "ü" => Ok(Yunmu::V),
            "ai" => Ok(Yunmu::AI),
            "ia" => Ok(Yunmu::IA),
            "ei" => Ok(Yunmu::EI),
            "er" => Ok(Yunmu::ER),
            "ui" => Ok(Yunmu::UI),
            "ao" => Ok(Yunmu::AO),
            "ou" => Ok(Yunmu::OU),
            "uo" => Ok(Yunmu::UO),
            "iu" => Ok(Yunmu::IU),
            "ie" => Ok(Yunmu::IE),
            "ue" => Ok(Yunmu::UE),
            "üe" => Ok(Yunmu::VE),
            "an" => Ok(Yunmu::AN),
            "en" => Ok(Yunmu::EN),
            "in" => Ok(Yunmu::IN),
            "un" => Ok(Yunmu::UN),
            "ua" => Ok(Yunmu::UA),
            "uai" => Ok(Yunmu::UAI),
            "uan" => Ok(Yunmu::UAN),
            "ian" => Ok(Yunmu::IAN),
            "iao" => Ok(Yunmu::IAO),
            "ang" => Ok(Yunmu::ANG),
            "eng" => Ok(Yunmu::ENG),
            "ing" => Ok(Yunmu::ING),
            "ong" => Ok(Yunmu::ONG),
            "iong" => Ok(Yunmu::IONG),
            "uang" => Ok(Yunmu::UANG),
            "iang" => Ok(Yunmu::IANG),
            "n" | "ng" | "m" | "ḿ" | "\u{304}" | "\u{300}" | "g" | "ế" | "ê̌" | "ề" | "ê\u{304}"
            | "" => Ok(Yunmu::ZeroYun),
            _ => Err(format!("Invalid yunmu {}", value)),
        }
    }
}

pub struct Pinyin {
    shengmu: Shengmu,
    yunmu: Yunmu,
    tone: Tone,
}

impl TryFrom<&str> for Pinyin {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let first_char = value.chars().next().unwrap();
        let mut shengmu_length = 1;
        let shengmu = match first_char {
            'b' => Shengmu::B,
            'p' => Shengmu::P,
            'm' => Shengmu::M,
            'f' => Shengmu::F,
            'd' => Shengmu::D,
            't' => Shengmu::T,
            'n' => Shengmu::N,
            'l' => Shengmu::L,
            'g' => Shengmu::G,
            'k' => Shengmu::K,
            'h' => Shengmu::H,
            'j' => Shengmu::J,
            'q' => Shengmu::Q,
            'x' => Shengmu::X,
            'r' => Shengmu::R,
            'y' => Shengmu::Y,
            'w' => Shengmu::W,
            'z' | 'c' | 's' => {
                let second_char = value.chars().nth(1);
                if let Some('h') = second_char {
                    shengmu_length = 2;
                    match first_char {
                        'z' => Shengmu::ZH,
                        'c' => Shengmu::CH,
                        's' => Shengmu::SH,
                        _ => unreachable!(),
                    }
                } else {
                    match first_char {
                        'z' => Shengmu::Z,
                        'c' => Shengmu::C,
                        's' => Shengmu::S,
                        _ => unreachable!(),
                    }
                }
            }
            _ => {
                shengmu_length = 0;
                Shengmu::ZeroSheng
            }
        };

        let yunmu_part = value.chars().skip(shengmu_length).collect::<String>();
        let tone = Tone::from(yunmu_part.as_str());
        let yunmu = Yunmu::try_from(yunmu_part.as_str())?;
        Ok(Pinyin {
            shengmu,
            yunmu,
            tone,
        })
    }
}

impl Pinyin {
    pub fn into_xnhe(&self) -> String {
        use Shengmu::*;
        use Yunmu::*;
        let mut last = match &self.yunmu {
            IU => 'q',
            EI => 'w',
            E => 'e',
            UAN => 'r',
            UE | VE => 't',
            UN => 'y',
            U => 'u',
            I => 'i',
            UO | O => 'o',
            IE => 'p',
            A => 'a',
            ONG | IONG => 's',
            AI => 'd',
            EN => 'f',
            ENG => 'g',
            ANG => 'h',
            AN => 'j',
            ING | UAI => 'k',
            IANG | UANG => 'l',
            OU => 'z',
            IA | UA => 'x',
            AO => 'c',
            UI | V => 'v',
            IN => 'b',
            IAO => 'n',
            IAN => 'm',
            ER => 'r',
            ZeroYun => '0',
        };
        let first = match &self.shengmu {
            B => 'b',
            P => 'p',
            M => 'm',
            F => 'f',
            D => 'd',
            T => 't',
            N => 'n',
            L => 'l',
            G => 'g',
            K => 'k',
            H => 'h',
            J => 'j',
            Q => 'q',
            X => 'x',
            R => 'r',
            Y => 'y',
            W => 'w',
            Z => 'z',
            C => 'c',
            S => 's',
            ZH => 'v',
            CH => 'i',
            SH => 'u',
            ZeroSheng => match &self.yunmu {
                A | O | E | I | U | V => last,
                ANG => 'a',
                ONG => 'o',
                ENG => 'e',
                AI | EI | AN | EN | AO | OU | ER => {
                    last = match &self.yunmu {
                        AI | EI => 'i',
                        AN | EN => 'n',
                        AO => 'o',
                        OU => 'u',
                        ER => 'r',
                        _ => unreachable!(),
                    };
                    match &self.yunmu {
                        AI | AN | AO => 'a',
                        EI | EN | ER => 'e',
                        OU => 'o',
                        _ => unreachable!(),
                    }
                }
                _ => '0',
            },
        };

        format!("{}{}", first, last)
    }
}
