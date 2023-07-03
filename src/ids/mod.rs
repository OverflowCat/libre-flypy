mod pn;

pub use self::pn::Tree;

fn split_ids(s: &str) -> Vec<&str> {
    s.split(&['\t', ';'][..])
        .map(|x| {
            if x.ends_with(')') {
                &x[..x
                    .rfind('(')
                    .expect("Only found closing parenthesis. Expect opening parenthesis.")]
            } else {
                x
            }
        })
        .collect()
}

fn remove_parenthesis(s: String) -> String {
    let mut t = String::with_capacity(s.len());
    let mut in_parenthesis = false;
    for c in s.chars() {
        if c == '(' || c == '[' || c == '{' {
            in_parenthesis = true;
        } else if c == ')' || c == ']' || c == '}' {
            in_parenthesis = false;
        } else if c == '↷' {
            continue;
        } else if !in_parenthesis {
            t.push(c);
        }
    }
    t
}

pub fn parse_ids(s: String) -> (char, Tree) {
    let s = remove_parenthesis(s);
    let (character, ids) = s.split_once('\t').expect("No tab found.");
    let ids = split_ids(ids);
    (
        character.chars().next().expect("No character found."),
        Tree::from(ids[0]),
    )
}
