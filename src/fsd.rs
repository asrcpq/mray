use std::collections::HashMap;
use crate::graphic_object::GraphicObjects;
use lazy_static::lazy_static;

pub fn fsd(c: char) -> GraphicObjects {
    macro_rules! default_color {
        () => {
            "0 0 1 0.7 "
        };
    };
    macro_rules! default_border_color {
        () => {
            "0 1 0 0.7 "
        };
    };
    lazy_static! {
        static ref SEGMENTS: Vec<GraphicObjects> = vec![
            GraphicObjects::from_strs(vec![concat!(
                "P ",
                default_border_color!(), default_color!(),
                "0.2 0.1 0.26 0.2 0.74 0.2 0.8 0.1"
            )]),
            GraphicObjects::from_strs(vec![concat!(
                "P ",
                default_border_color!(), default_color!(),
                "0.74 0.2 0.8 0.1 0.8 0.5 0.74 0.47"
            )]),
            GraphicObjects::from_strs(vec![concat!(
                "P ",
                default_border_color!(), default_color!(),
                "0.74 0.53 0.74 0.8 0.8 0.9 0.8 0.5"
            )]),
            GraphicObjects::from_strs(vec![concat!(
                "P ",
                default_border_color!(), default_color!(),
                "0.8 0.9 0.2 0.9 0.26 0.8 0.74 0.8"
            )]),
            GraphicObjects::from_strs(vec![concat!(
                "P ",
                default_border_color!(), default_color!(),
                "0.2 0.9 0.26 0.8 0.26 0.53 0.2 0.5"
            )]),
            GraphicObjects::from_strs(vec![concat!(
                "P ",
                default_border_color!(), default_color!(),
                "0.26 0.47 0.26 0.2 0.2 0.1 0.2 0.5"
            )]),
            GraphicObjects::from_strs(vec![concat!(
                "P ",
                default_border_color!(), default_color!(),
                "0.2 0.5 0.26 0.47 0.47 0.47 0.5 0.5 0.47 0.53 0.26 0.53"
            )]),
            GraphicObjects::from_strs(vec![concat!(
                "P ",
                default_border_color!(), default_color!(),
                "0.53 0.53 0.5 0.5 0.53 0.47 0.74 0.47 0.8 0.5 0.74 0.53"
            )]),
            GraphicObjects::from_strs(vec![concat!(
                "P ",
                default_border_color!(), default_color!(),
                "0.26 0.2 0.5 0.375 0.5 0.5 0.26 0.325"
            )]),
            GraphicObjects::from_strs(vec![concat!(
                "P ",
                default_border_color!(), default_color!(),
                "0.47 0.2 0.53 0.2 0.53 0.47 0.5 0.5 0.47 0.47"
            )]),
            GraphicObjects::from_strs(vec![concat!(
                "P ",
                default_border_color!(), default_color!(),
                "0.74 0.2 0.5 0.375 0.5 0.5 0.74 0.325"
            )]),
            GraphicObjects::from_strs(vec![concat!(
                "P ",
                default_border_color!(), default_color!(),
                "0.26 0.8 0.26 0.675 0.5 0.5 0.5 0.625"
            )]),
            GraphicObjects::from_strs(vec![concat!(
                "P ",
                default_border_color!(), default_color!(),
                "0.47 0.53 0.5 0.5 0.53 0.53 0.53 0.8 0.47 0.8"
            )]),
            GraphicObjects::from_strs(vec![concat!(
                "P ",
                default_border_color!(), default_color!(),
                "0.74 0.8 0.74 0.675 0.5 0.5 0.5 0.625"
            )]),
        ];
        static ref CHAR_MAP: HashMap<char, Vec<usize>> = (0..=255u8)
            .map(char::from)
            .map(|c| (
                c,
                match c {
                    '!' => vec![3, 9, 11, 13],
                    '"' => vec![5, 9],
                    '#' => vec![1, 2, 3, 6, 7, 9, 12],
                    '$' => vec![0, 2, 3, 5, 6, 7, 9, 12],
                    '%' => vec![2, 5, 10, 11],
                    '&' => vec![0, 3, 4, 6, 8, 10, 13],
                    '\'' => vec![9],
                    '(' => vec![10, 13],
                    ')' => vec![8, 11],
                    '*' => vec![8, 9, 10, 11, 12, 13],
                    '+' => vec![6, 7, 9, 12],
                    ',' => vec![11],
                    '-' => vec![6, 7],
                    '.' => vec![4],
                    '/' => vec![10, 11],
                    '0' => vec![0, 1, 2, 3, 4, 5, 10, 11],
                    '1' => vec![1, 2, 10],
                    '2' => vec![0, 1, 3, 4, 6, 7],
                    '3' => vec![0, 1, 2, 3, 7],
                    '4' => vec![1, 2, 5, 6, 7],
                    '5' => vec![0, 2, 3, 5, 6, 7],
                    '6' => vec![0, 2, 3, 4, 5, 6, 7],
                    '7' => vec![0, 10, 12],
                    '8' => vec![0, 1, 2, 3, 4, 5, 6, 7],
                    '9' => vec![0, 1, 2, 5, 6, 7],
                    ':' => vec![9, 13],
                    ';' => vec![9, 11],
                    '<' => vec![6, 10, 13],
                    '=' => vec![3, 6, 7],
                    '>' => vec![7, 8, 11],
                    '?' => vec![0, 5, 10, 12],
                    '@' => vec![0, 1, 2, 3, 4, 5, 8, 10, 11, 13],
                    'A' => vec![0, 1, 2, 4, 5, 6, 7],
                    'B' => vec![0, 1, 2, 3, 7, 9, 12],
                    'C' => vec![0, 3, 4, 5],
                    'D' => vec![0, 1, 2, 3, 9, 12],
                    'E' => vec![0, 3, 4, 5, 6, 7],
                    'F' => vec![0, 4, 5, 6, 7],
                    'G' => vec![0, 2, 3, 4, 5, 7],
                    'H' => vec![1, 2, 4, 5, 6, 7],
                    'I' => vec![0, 3, 9, 12],
                    'J' => vec![1, 2, 3, 4],
                    'K' => vec![4, 5, 6, 10, 13],
                    'L' => vec![3, 4, 5],
                    'M' => vec![1, 2, 4, 5, 8, 10],
                    'N' => vec![1, 2, 4, 5, 8, 13],
                    'O' => vec![0, 1, 2, 3, 4, 5],
                    'P' => vec![0, 1, 4, 5, 6, 7],
                    'Q' => vec![0, 1, 2, 3, 4, 5, 13],
                    'R' => vec![0, 1, 4, 5, 6, 7, 13],
                    'S' => vec![0, 2, 3, 7, 8],
                    'T' => vec![0, 9, 12],
                    'U' => vec![1, 2, 3, 4, 5],
                    'V' => vec![4, 5, 10, 11],
                    'W' => vec![1, 2, 4, 5, 11, 13],
                    'X' => vec![0, 3, 8, 10, 11, 13],
                    'Y' => vec![0, 8, 10, 12],
                    'Z' => vec![0, 3, 10, 11],
                    '[' => vec![0, 3, 4, 5, 10, 13],
                    '\\' => vec![8, 13],
                    ']' => vec![0, 1, 2, 3, 8, 11],
                    '^' => vec![8, 10],
                    '_' => vec![3],
                    '`' => vec![8],
                    'a' => vec![2, 3, 4, 6, 7, 13],
                    'b' => vec![3, 4, 5, 6, 13],
                    'c' => vec![3, 4, 6, 7],
                    'd' => vec![1, 2, 3, 7, 11],
                    'e' => vec![3, 4, 6, 11],
                    'f' => vec![3, 6, 7, 10, 12],
                    'g' => vec![2, 3, 4, 7, 12],
                    'h' => vec![3, 7, 12],
                    'i' => vec![3, 6, 12],
                    'j' => vec![2, 3, 10],
                    'k' => vec![4, 5, 7, 11, 13],
                    'l' => vec![4, 5, 11],
                    'm' => vec![2, 4, 6, 7, 12],
                    'n' => vec![2, 4, 6, 7],
                    'o' => vec![2, 3, 4, 6, 7],
                    'p' => vec![3, 4, 6, 12],
                    'q' => vec![2, 3, 7, 12],
                    'r' => vec![4, 6, 7],
                    's' => vec![3, 7, 13],
                    't' => vec![3, 4, 5, 6],
                    'u' => vec![2, 3, 4],
                    'v' => vec![4, 7, 11],
                    'w' => vec![2, 4, 11, 13],
                    'x' => vec![6, 7, 11, 13],
                    'y' => vec![2, 3, 6, 13],
                    'z' => vec![3, 6, 11],
                    '{' => vec![0, 3, 6, 8, 11],
                    '|' => vec![9, 12],
                    '}' => vec![0, 3, 7, 10, 13],
                    '~' => vec![0],
                    _ => Vec::new(),
                }
            ))
            .collect();
    }
    let mut result: GraphicObjects = Default::default();
    for id in CHAR_MAP.get(&c).unwrap_or(&Vec::new()) {
        result.extend(SEGMENTS[*id].clone());
    }
    result
}
