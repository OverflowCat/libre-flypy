use std::collections::HashMap;

pub fn generate() -> (HashMap<char, char>, HashMap<char, char>) {
    let first = HashMap::from([
        ('凹', 'a'),
        ('一', 'a'),
        ('𰀁', 'a'),
        ('𠀎', 'a'),
        ('𫠠', 'a'),
        ('𢦑', 'a'),
        ('彧', 'a'),
        ('𰀉', 'a'),
        ('镸', 'a'),
        ('惠', 'a'),
        ('朿', 'a'),
        ('鱼', 'a'),
        ('八', 'b'),
        ('百', 'b'),
        ('匕', 'b'),
        ('𠤎', 'b'),
        ('疒', 'b'),
        ('巴', 'b'),
        ('白', 'b'),
        ('宀', 'b'),
        ('穴', 'b'),
        ('冖', 'b'),
        ('丷', 'b'),
        ('半', 'b'),
        ('必', 'b'),
        ('丙', 'b'),
        ('勹', 'b'),
        ('不', 'b'),
        ('艹', 'c'),
        ('册', 'c'),
        ('才', 'c'),
        ('歹', 'd'),
        ('丁', 'd'),
        ('丹', 'd'),
        ('东', 'd'),
        ('脊', 'd'),
        ('丶', 'd'),
        ('冫', 'd'),
        ('氵', 'd'),
        ('⺈', 'd'),
        ('刀', 'd'),
        ('㡀', 'd'),
        ('大', 'd'),
        ('氐', 'd'),
        ('山', 'e'),
        ('而', 'e'),
        ('兆', 'e'),
        ('耳', 'e'),
        ('卩', 'e'),
        ('𠁳', 'e'),
        ('彐', 'e'),
        ('ヨ', 'e'),
        ('⺕', 'e'),
        ('儿', 'e'),
        ('二', 'e'),
        ('阝', 'e'),
        ('龶', 'f'),
        ('夫', 'f'),
        ('𡗗', 'f'),
        ('麦', 'f'),
        // ('麥', 'f'),
        // ('寿上', 'f'),
        ('方', 'f'),
        ('弗', 'f'),
        ('非', 'f'),
        ('扌', 'f'),
        ('甫', 'f'),
        ('缶', 'f'),
        ('丰', 'f'),
        ('卅', 'f'),
        ('艮', 'g'),
        ('骨', 'g'),
        ('干', 'g'),
        ('广', 'g'),
        ('鬼', 'g'),
        ('革', 'g'),
        ('毌', 'g'),
        ('戈', 'g'),
        ('甘', 'g'),
        ('工', 'g'),
        ('艮', 'g'),
        ('更', 'g'),
        ('弓', 'g'),
        ('瓜', 'g'),
        ('果', 'g'),
        ('户', 'h'),
        ('虍', 'h'),
        ('火', 'h'),
        ('黑', 'h'),
        ('禾', 'h'),
        ('叉', 'i'),
        ('产', 'i'),
        ('厂', 'i'),
        ('串', 'i'),
        ('尺', 'i'),
        ('出', 'i'),
        ('虫', 'i'),
        ('车', 'i'),
        ('川', 'i'),
        ('臣', 'i'),
        ('成', 'i'),
        ('彳', 'i'),
        ('龹', 'j'),
        ('甲', 'j'),
        ('斤', 'j'),
        ('九', 'j'),
        ('臼', 'j'),
        ('鼠', 'j'),
        ('钅', 'j'),
        ('金', 'j'),
        ('己', 'j'),
        ('久', 'j'),
        ('几', 'j'),
        ('巾', 'j'),
        ('夹', 'j'),
        ('戋', 'j'),
        ('口', 'k'),
        ('冂', 'k'),
        ('⺆', 'k'),
        ('冃', 'k'),
        ('盥', 'k'),
        ('开', 'k'),
        ('匚', 'k'),
        ('囗', 'k'),
        ('立', 'l'),
        ('丽', 'l'),
        ('来', 'l'),
        ('卵', 'l'),
        ('丨', 'l'),
        ('⺊', 'l'),
        ('北', 'l'),
        ('里', 'l'),
        ('了', 'l'),
        ('龙', 'l'),
        ('力', 'l'),
        ('耂', 'l'),
        ('耒', 'l'),
        ('木', 'm'),
        ('马', 'm'),
        ('馬', 'm'),
        ('民', 'm'),
        ('毛', 'm'),
        ('米', 'm'),
        ('面', 'm'),
        ('门', 'm'),
        ('内', 'n'),
        ('女', 'n'),
        ('鸟', 'n'),
        ('廿', 'n'),
        ('牛', 'n'),
        ('𠂒', 'n'),
        ('⺧', 'n'),
        ('乃', 'n'),
        ('月', 'o'),
        ('⺝', 'o'),
        ('日', 'o'),
        ('目', 'o'),
        ('鼎', 'o'),
        ('礻', 'p'),
        ('丿', 'p'),
        ('㇀', 'p'),
        ('㇁', 'p'),
        ('𠂆', 'p'),
        ('𫜵', 'p'),
        ('𦥑', 'p'),
        ('𦥔', 'p'),
        ('𠂎', 'p'),
        ('𠂊', 'p'),
        ('卬', 'p'),
        ('卑', 'p'),
        ('爿', 'p'),
        ('片', 'p'),
        ('制', 'p'),
        ('衤', 'p'),
        ('示', 'p'),
        ('丕', 'p'),
        ('犭', 'q'),
        ('丘', 'q'),
        // ('具上', 'q'),
        ('气', 'q'),
        ('犬', 'q'),
        ('求', 'q'),
        ('其', 'q'),
        ('七', 'q'),
        ('且', 'q'),
        ('千', 'q'),
        ('入', 'r'),
        ('人', 'r'),
        ('亻', 'r'),
        ('刃', 'r'),
        ('壬', 'r'),
        ('冉', 'r'),
        ('罒', 's'),
        ('丝', 's'),
        ('肃', 's'),
        ('厶', 's'),
        ('纟', 's'),
        ('巳', 's'),
        ('龴', 's'),
        // ('予', 's'),
        // ('矛', 's'),
        ('天', 't'),
        ('太', 't'),
        ('田', 't'),
        ('土', 't'),
        ('屯', 't'),
        ('身', 'u'),
        ('水', 'u'),
        ('氏', 'u'),
        ('𠂇', 'u'),
        ('上', 'u'),
        ('生', 'u'),
        ('尸', 'u'),
        ('饣', 'u'),
        ('束', 'u'),
        ('龵', 'u'),
        ('石', 'u'),
        ('矢', 'u'),
        ('士', 'u'),
        ('世', 'u'),
        ('豕', 'u'),
        ('申', 'u'),
        ('十', 'u'),
        ('𢦏', 'u'),
        ('尢', 'u'),
        ('少', 'u'),
        ('乍', 'v'),
        ('舟', 'v'),
        ('止', 'v'),
        ('齒', 'v'),
        ('齿', 'v'),
        ('叚', 'v'),
        ('爪', 'v'),
        ('⺮', 'v'),
        ('𥫗', 'v'),
        ('巛', 'v'),
        ('巜', 'v'),
        ('𡿨', 'v'),
        ('中', 'v'),
        ('正', 'v'),
        ('豸', 'v'),
        ('𠃋', 'v'),
        ('㇇', 'v'),
        ('𠂈', 'v'),
        ('乛', 'v'),
        ('⺄', 'v'),
        ('𠃍', 'v'),
        ('𠃌', 'v'),
        ('𠄎', 'v'),
        ('乙', 'v'),
        ('ユ', 'v'),
        ('彑', 'v'),
        ('肀', 'v'),
        ('乜', 'v'),
        ('癶', 'v'),
        ('𠃜', 'v'),
        ('隶', 'v'),
        ('朱', 'v'),
        ('尹', 'v'),
        ('戊', 'w'),
        ('乌', 'w'),
        ('勿', 'w'),
        ('五', 'w'),
        ('韦', 'w'),
        ('午', 'w'),
        ('兀', 'w'),
        ('夂', 'w'),
        ('我', 'w'),
        ('亠', 'w'),
        ('亥', 'w'),
        ('未', 'w'),
        ('亡', 'w'),
        ('王', 'w'),
        ('𪡅', 'w'),
        ('万', 'w'),
        ('文', 'w'),
        ('臧', 'w'),
        ('⺍', 'x'),
        ('𭕄', 'x'),
        ('⺌', 'x'),
        ('𡭙', 'x'),
        ('小', 'x'),
        ('戌', 'x'),
        ('西', 'x'),
        ('覀', 'x'),
        ('乡', 'x'),
        ('夕', 'x'),
        ('乂', 'x'),
        ('𠂭', 'x'),
        ('𠚍', 'x'),
        ('下', 'x'),
        ('习', 'x'),
        ('忄', 'x'),
        ('血', 'x'),
        ('⺷', 'y'),
        ('又', 'y'),
        ('也', 'y'),
        ('禺', 'y'),
        ('牙', 'y'),
        ('禹', 'y'),
        ('酉', 'y'),
        ('央', 'y'),
        ('弋', 'y'),
        ('畿', 'y'),
        ('亚', 'y'),
        ('讠', 'y'),
        ('尹', 'y'),
        ('幺', 'y'),
        ('尢', 'y'),
        ('于', 'y'),
        ('夭', 'y'),
        ('永', 'y'),
        ('⺶', 'y'),
        ('与', 'y'),
        ('业', 'y'),
        ('羌', 'y'),
        ('羊', 'y'),
        ('𦍌', 'y'),
        ('雨', 'y'),
        ('由', 'y'),
        ('⻊', 'z'),
        ('𧾷', 'z'),
        ('廴', 'z'),
        ('辶', 'z'),
        ('子', 'z'),
        ('自', 'z'),
    ]);
    let last = HashMap::from([
        ('一', 'a'),
        ('戢', 'a'),
        ('堇', 'a'),
        ('豆', 'a'),
        ('亟', 'a'),
        ('鱼', 'a'),
        ('不', 'b'),
        ('丙', 'b'),
        ('巴', 'b'),
        ('卜', 'b'),
        ('必', 'b'),
        ('八', 'b'),
        ('其', 'b'),
        ('半', 'b'),
        ('本', 'b'),
        ('冖', 'b'),
        ('办', 'b'),
        ('卞', 'b'),
        ('匕', 'b'),
        ('𠤎', 'b'),
        ('百', 'b'),
        ('白', 'b'),
        ('寸', 'c'),
        ('才', 'c'),
        ('册', 'c'),
        ('艹', 'c'),
        ('廾', 'c'),
        ('大', 'd'),
        ('歹', 'd'),
        ('东', 'd'),
        ('氐', 'd'),
        ('丹', 'd'),
        ('刀', 'd'),
        ('丁', 'd'),
        ('丶', 'd'),
        ('冫', 'd'),
        ('𡿨', 'd'),
        ('羲', 'd'),
        ('刁', 'd'),
        ('赤', 'd'),
        ('禸', 'd'),
        ('彧', 'd'),
        ('刂', 'd'),
        ('兆', 'd'),
        ('发', 'd'),
        ('电', 'd'),
        // ('亦', 'd'),
        ('电', 'd'),
        ('𡗞', 'd'),
        ('阝', 'e'),
        ('儿', 'e'),
        ('山', 'e'),
        ('二', 'e'),
        ('卩', 'e'),
        ('耳', 'e'),
        ('而', 'e'),
        ('见', 'e'),
        ('彐', 'e'),
        ('ヨ', 'e'),
        ('缶', 'f'),
        ('非', 'f'),
        ('方', 'f'),
        ('甫', 'f'),
        ('龶', 'f'),
        ('夫', 'f'),
        ('乏', 'f'),
        ('凡', 'f'),
        ('嬴', 'f'),
        ('蠃', 'f'),
        ('赢', 'f'),
        ('丰', 'f'),
        ('隺', 'f'),
        ('弗', 'f'),
        ('更', 'g'),
        ('弓', 'g'),
        ('骨', 'g'),
        ('戈', 'g'),
        ('丐', 'g'),
        ('瓜', 'g'),
        ('革', 'g'),
        ('干', 'g'),
        ('果', 'g'),
        ('鬼', 'g'),
        ('工', 'g'),
        ('甘', 'g'),
        ('艮', 'g'),
        ('夬', 'g'),
        ('广', 'g'),
        ('乎', 'h'),
        ('禾', 'h'),
        ('火', 'h'),
        ('户', 'h'),
        ('灬', 'h'),
        ('爲', 'h'),
        ('互', 'h'),
        ('朩', 'h'),
        ('黑', 'h'),
        ('虫', 'i'),
        ('出', 'i'),
        ('川', 'i'),
        ('成', 'i'),
        ('长', 'i'),
        ('尺', 'i'),
        ('垂', 'i'),
        ('串', 'i'),
        ('产', 'i'),
        ('叉', 'i'),
        ('斥', 'i'),
        ('车', 'i'),
        ('亍', 'i'),
        ('厂', 'i'),
        ('臣', 'i'),
        ('臧', 'i'),
        ('丑', 'i'),
        ('臼', 'j'),
        ('己', 'j'),
        ('九', 'j'),
        ('久', 'j'),
        ('金', 'j'),
        ('夹', 'j'),
        ('几', 'j'),
        ('及', 'j'),
        ('戋', 'j'),
        ('柬', 'j'),
        ('井', 'j'),
        ('甲', 'j'),
        ('巾', 'j'),
        ('巨', 'j'),
        ('斤', 'j'),
        ('击', 'j'),
        ('韭', 'j'),
        ('㠯', 'k'),
        ('亏', 'k'),
        ('匚', 'k'),
        ('コ', 'k'),
        ('凵', 'k'),
        ('凶', 'k'),
        ('开', 'k'),
        ('口', 'k'),
        ('周', 'k'),
        ('龙', 'l'),
        ('了', 'l'),
        ('予', 'l'),
        ('立', 'l'),
        ('丨', 'l'),
        ('亅', 'l'),
        ('芈', 'l'),
        ('㐄', 'l'),
        ('𰀁', 'l'),
        ('𠬤', 'l'),
        ('𬎾', 'l'),
        ('丽', 'l'),
        ('力', 'l'),
        ('良', 'l'),
        ('食', 'l'),
        ('两', 'l'),
        ('来', 'l'),
        ('丌', 'l'),
        ('单', 'l'),
        ('革', 'l'),
        ('争', 'l'),
        ('齑', 'l'),
        ('巿', 'l'),
        ('吏', 'l'),
        ('耒', 'l'),
        ('乐', 'l'),
        ('里', 'l'),
        ('丏', 'm'),
        ('木', 'm'),
        ('米', 'm'),
        ('皿', 'm'),
        ('门', 'm'),
        ('民', 'm'),
        ('么', 'm'),
        ('马', 'm'),
        ('馬', 'm'),
        ('面', 'm'),
        ('末', 'm'),
        ('毛', 'm'),
        ('母', 'm'),
        ('乃', 'n'),
        ('女', 'n'),
        ('农', 'n'),
        ('𧰨', 'n'),
        ('内', 'n'),
        ('象', 'n'),
        ('鸟', 'n'),
        ('鳥', 'n'),
        ('ㄟ', 'n'),
        ('乀', 'n'),
        ('隶', 'n'),
        ('乑', 'n'),
        ('朿', 'n'),
        ('兼', 'n'),
        ('𧘇', 'n'),
        ('𰀠', 'n'),
        ('𠂢', 'n'),
        ('菐', 'n'),
        ('兼', 'n'),
        ('夜', 'n'),
        ('牛', 'n'),
        ('目', 'o'),
        ('日', 'o'),
        ('曰', 'o'),
        ('⺝', 'o'),
        ('⺼', 'o'),
        ('月', 'o'),
        ('肖', 'o'),
        ('丕', 'p'),
        ('𬺻', 'p'),
        ('平', 'p'),
        ('爿', 'p'),
        ('片', 'p'),
        ('尹', 'p'),
        ('丿', 'p'),
        ('㇀', 'p'),
        ('㇁', 'p'),
        ('彡', 'p'),
        ('七', 'q'),
        ('犬', 'q'),
        ('且', 'q'),
        ('直', 'q'),
        ('羌', 'q'),
        ('曲', 'q'),
        ('乞', 'q'),
        ('气', 'q'),
        ('求', 'q'),
        ('具', 'q'),
        ('丘', 'q'),
        ('千', 'q'),
        ('冉', 'r'),
        ('刃', 'r'),
        ('亻', 'r'),
        ('人', 'r'),
        ('颎', 'r'),
        ('颍', 'r'),
        ('𤴓', 'r'),
        ('龰', 'r'),
        ('疌', 'r'),
        ('壬', 'r'),
        ('肃', 's'),
        ('三', 's'),
        ('丝', 's'),
        ('巳', 's'),
        ('厶', 's'),
        ('龴', 's'),
        ('屯', 't'),
        ('田', 't'),
        ('天', 't'),
        ('太', 't'),
        ('土', 't'),
        ('坐', 't'),
        ('升', 'u'),
        ('𠂇', 'u'),
        ('氺', 'u'),
        ('水', 'u'),
        ('手', 'u'),
        ('生', 'u'),
        ('豕', 'u'),
        ('彖', 'u'),
        ('辛', 'u'),
        ('少', 'u'),
        ('史', 'u'),
        ('十', 'u'),
        ('斗', 'u'),
        ('卑', 'u'),
        ('士', 'u'),
        ('失', 'u'),
        ('𰦒', 'u'),
        ('石', 'u'),
        ('申', 'u'),
        ('勺', 'u'),
        ('戍', 'u'),
        ('束', 'u'),
        ('尸', 'u'),
        ('上', 'u'),
        ('术', 'u'),
        ('氏', 'u'),
        ('矢', 'u'),
        ('世', 'u'),
        ('重', 'v'),
        ('中', 'v'),
        ('朱', 'v'),
        ('舟', 'v'),
        ('乍', 'v'),
        ('之', 'v'),
        ('爪', 'v'),
        ('匃', 'v'),
        ('乛', 'v'),
        ('旡', 'v'),
        ('乙', 'v'),
        ('𪜊', 'v'),
        ('巛', 'v'),
        ('巜', 'v'),
        ('𡿨', 'v'),
        ('甚', 'v'),
        ('乚', 'v'),
        ('㇉', 'v'),
        ('㇍', 'v'),
        ('⺄', 'v'),
        ('𠃍', 'v'),
        ('𠃊', 'v'),
        ('〇', 'v'),
        ('㇂', 'v'),
        ('𠃋', 'v'),
        ('乙', 'v'),
        ('㇄', 'v'),
        ('鼎', 'v'),
        ('尢', 'v'),
        ('丐', 'v'),
        ('丏', 'v'),
        ('羌', 'v'),
        ('冘', 'v'),
        ('免', 'v'),
        ('𫶧', 'v'),
        ('乜', 'v'),
        ('龟', 'v'),
        ('㔾', 'v'),
        ('鼠', 'v'),
        ('专', 'v'),
        ('州', 'v'),
        ('主', 'v'),
        ('丈', 'v'),
        ('止', 'v'),
        ('正', 'v'),
        ('鼎', 'v'),
        ('勿', 'w'),
        ('毋', 'w'),
        ('未', 'w'),
        ('为', 'w'),
        ('丸', 'w'),
        ('韦', 'w'),
        ('五', 'w'),
        ('无', 'w'),
        ('夂', 'w'),
        ('王', 'w'),
        ('文', 'w'),
        ('午', 'w'),
        ('亡', 'w'),
        ('万', 'w'),
        ('兀', 'w'),
        ('我', 'w'),
        ('乌', 'w'),
        ('攵', 'w'),
        ('戊', 'w'),
        ('瓦', 'w'),
        ('乂', 'x'),
        ('心', 'x'),
        ('西', 'x'),
        ('血', 'x'),
        ('习', 'x'),
        ('⺗', 'x'),
        ('㣺', 'x'),
        ('乡', 'x'),
        ('小', 'x'),
        ('下', 'x'),
        ('夕', 'x'),
        ('用', 'y'),
        ('又', 'y'),
        ('皮', 'y'),
        ('反', 'y'),
        ('义', 'y'),
        // ('縠', 'y'),
        ('曳', 'y'),
        ('于', 'y'),
        ('与', 'y'),
        ('禺', 'y'),
        ('聿', 'y'),
        ('由', 'y'),
        ('亚', 'y'),
        ('雨', 'y'),
        ('尢', 'y'),
        ('羊', 'y'),
        ('禹', 'y'),
        ('永', 'y'),
        ('夷', 'y'),
        ('央', 'y'),
        ('夭', 'y'),
        ('尹', 'y'),
        ('幺', 'y'),
        ('酉', 'y'),
        ('𧘇', 'y'),
        ('玉', 'y'),
        ('牙', 'y'),
        ('也', 'y'),
        ('丫', 'y'),
        ('严', 'y'),
        ('臾', 'y'),
        ('业', 'y'),
        ('弋', 'y'),
        ('尤', 'y'),
        ('自', 'z'),
        ('廴', 'z'),
        ('子', 'z'),
        ('辶', 'z'),
    ]);
    (first, last)
}
