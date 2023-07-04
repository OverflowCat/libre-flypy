use std::fmt::Debug;

#[derive(PartialEq, Eq, Hash)]
/// Represents an ideographic description character.
pub enum Idc {
    /// ⿰ - Left-to-right
    LeftToRight,
    /// ⿱ - Above-to-below
    AboveToBelow,
    /// ⿲ - Left-to-middle-and-right
    LeftToMiddleAndRight,
    /// ⿳ - Above-to-middle-and-below
    AboveToMiddleAndBelow,
    /// ⿴ - Full surround
    FullSurround,
    /// ⿵ - Surround from above
    SurroundFromAbove,
    /// ⿶ - Surround from below
    SurroundFromBelow,
    /// ⿷ - Surround from left
    SurroundFromLeft,
    /// ⿸ - Surround from upper left
    SurroundFromUpperLeft,
    /// ⿹ - Surround from upper right
    SurroundFromUpperRight,
    /// ⿺ - Surround from lower left
    SurroundFromLowerLeft,
    /// ⿻ - Overlaid
    Overlaid,
}

impl TryFrom<char> for Idc {
    fn try_from(c: char) -> Result<Idc, ()> {
        match c {
            '⿰' => Ok(Idc::LeftToRight),
            '⿱' => Ok(Idc::AboveToBelow),
            '⿲' => Ok(Idc::LeftToMiddleAndRight),
            '⿳' => Ok(Idc::AboveToMiddleAndBelow),
            '⿴' => Ok(Idc::FullSurround),
            '⿵' => Ok(Idc::SurroundFromAbove),
            '⿶' => Ok(Idc::SurroundFromBelow),
            '⿷' => Ok(Idc::SurroundFromLeft),
            '⿸' => Ok(Idc::SurroundFromUpperLeft),
            '⿹' => Ok(Idc::SurroundFromUpperRight),
            '⿺' => Ok(Idc::SurroundFromLowerLeft),
            '⿻' => Ok(Idc::Overlaid),
            _ => Err(()),
        }
    }

    type Error = ();
}

impl ToString for Idc {
    fn to_string(&self) -> String {
        String::from(match self {
            Idc::LeftToRight => "⿰",
            Idc::AboveToBelow => "⿱",
            Idc::LeftToMiddleAndRight => "⿲",
            Idc::AboveToMiddleAndBelow => "⿳",
            Idc::FullSurround => "⿴",
            Idc::SurroundFromAbove => "⿵",
            Idc::SurroundFromBelow => "⿶",
            Idc::SurroundFromLeft => "⿷",
            Idc::SurroundFromUpperLeft => "⿸",
            Idc::SurroundFromUpperRight => "⿹",
            Idc::SurroundFromLowerLeft => "⿺",
            Idc::Overlaid => "⿻",
        })
    }
}

impl Debug for Idc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

impl Idc {
    fn get_children_count(&self) -> usize {
        match self {
            Idc::LeftToMiddleAndRight | Idc::AboveToMiddleAndBelow => 3,
            _ => 2,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Tree {
    Node {
        idc: Idc,
        children: Vec<Tree>,
        root: bool,
    },
    Leaf {
        value: char,
    },
}

impl From<&str> for Tree {
    fn from(s: &str) -> Self {
        // '⿰', '⿱', '⿲', '⿳', '⿴', '⿵', '⿶', '⿷', '⿸', '⿹', '⿺', '⿻'
        let mut stack = Vec::with_capacity(3);
        for c in s.chars().rev() {
            if let Ok(idc) = Idc::try_from(c) {
                // if idc == Idc::SurroundFromBelow {
                //     println!("<< {}: {:?} >>", s, idc);
                // }
                let children = (0..idc.get_children_count())
                    .map(|_| stack.pop().expect("Not enough children."))
                    .collect();
                let node = Tree::Node {
                    idc,
                    children,
                    root: false,
                };
                stack.push(node);
            } else {
                stack.push(Tree::Leaf { value: c });
            }
        }
        match stack.pop().expect("No root.") {
            Tree::Node { idc, children, .. } => Tree::Node {
                idc,
                children,
                root: true,
            },
            Tree::Leaf { value } => Tree::Node {
                idc: Idc::LeftToRight,
                children: vec![Tree::Leaf { value }, Tree::Leaf { value }],
                root: true,
            },
        }
    }
}

impl Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Tree::Node { idc, children, .. } => {
                write!(f, "{:?}(", idc)?;
                for child in children {
                    write!(f, "{:?}", child)?;
                }
                write!(f, ")")
            }
            Tree::Leaf { value } => write!(f, "{}", value),
        }
    }
}

impl Tree {
    pub fn get_first_leaf(&self) -> char {
        match self {
            Tree::Node {
                idc: _, children, ..
            } => {
                let mut child = &children[0];
                match child {
                    Tree::Leaf { value, .. } => {
                        if ['凵', '𠃊'].contains(value) {
                            child = &children[1];
                        }
                    }
                    _ => {}
                }
                child.get_first_leaf()
            }
            Tree::Leaf { value } => *value,
        }
    }

    pub fn get_last_leaf(&self) -> char {
        match self {
            Tree::Node {
                idc,
                children,
                root,
            } => {
                if *idc == Idc::SurroundFromLowerLeft && !root {
                    match children.first() {
                        Some(Tree::Leaf { value }) => {
                            if *value == '辶' {
                                // println!("{:?}", children);
                                return '辶';
                            } else if *value == '廴' {
                                // println!("{:?}", children);
                                return '廴';
                            }
                        }
                        _ => {}
                    }
                }
                children[children.len() - 1].get_last_leaf()
            }
            Tree::Leaf { value } => *value,
        }
    }

    /*
    pub fn traverse_first(&self, f: &dyn Fn(&Self) -> Option<char>) -> char {
        if let Some(value) = f(&self) {
            value
        } else {
            match self {
                Tree::Node { idc: _, children } => children[0].traverse_first(f),
                Tree::Leaf { value } => *value,
            }
        }
    }

    pub fn traverse_last(&self, f: &dyn Fn(&Self) -> Option<char>) -> char {
        if let Some(value) = f(&self) {
            value
        } else {
            match self {
                Tree::Node { idc: _, children } => {
                    children[children.len() - 1].traverse_last(f)
                }
                Tree::Leaf { value } => *value,
            }
        }
    } */
}
