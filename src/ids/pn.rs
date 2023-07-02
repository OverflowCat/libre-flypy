use std::fmt::Debug;

#[derive(PartialEq, Eq, Hash)]
pub enum Idc {
    LeftToRight,
    AboveToBelow,
    LeftToMiddleAndRight,
    AboveToMiddleAndBelow,
    FullSurround,
    SurroundFromAbove,
    SurroundFromBelow,
    SurroundFromLeft,
    SurroundFromUpperLeft,
    SurroundFromUpperRight,
    SurroundFromLowerLeft,
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

pub enum Tree {
    Node { idc: Idc, children: Vec<Tree> },
    Leaf { value: char },
}

impl From<&str> for Tree {
    fn from(s: &str) -> Self {
        // '⿰', '⿱', '⿲', '⿳', '⿴', '⿵', '⿶', '⿷', '⿸', '⿹', '⿺', '⿻'
        let mut stack = Vec::with_capacity(3);
        for c in s.chars().rev() {
            if let Ok(idc) = Idc::try_from(c) {
                let children = (0..idc.get_children_count())
                    .map(|_| stack.pop().expect("Not enough children."))
                    .collect();
                let node = Tree::Node { idc, children };
                stack.push(node);
            } else {
                stack.push(Tree::Leaf { value: c });
            }
        }
        stack.pop().expect("No root found.")
    }
}

impl Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Tree::Node { idc, children } => {
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
            Tree::Node { idc: _, children } => children[0].get_first_leaf(),
            Tree::Leaf { value } => *value,
        }
    }

    pub fn get_last_leaf(&self) -> char {
        match self {
            Tree::Node { idc, children } => {
                // if *idc == Idc::SurroundFromLowerLeft {
                //     let last_leaf = children[0].get_last_leaf();
                //     if last_leaf != self.get_first_leaf() {
                //         println!("== {}: {}", last_leaf, self.get_first_leaf());
                //         return last_leaf;
                //     }
                // }
                children[children.len() - 1].get_last_leaf()
            }
            Tree::Leaf { value } => *value,
        }
    }

    /*     pub fn traverse_first(&self, f: &dyn Fn(&Self) -> Option<char>) -> char {
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
