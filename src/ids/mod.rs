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

pub fn parse_ids(s: &str) -> (char, Tree) {
    let (character, ids) = s.split_once('\t').expect("No tab found.");
    let ids = split_ids(ids);
    (
        character.chars().next().expect("No character found."),
        Tree::from(ids[0]),
    )
}
