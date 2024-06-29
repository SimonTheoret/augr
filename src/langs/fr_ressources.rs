use crate::langs::{KBAugmentationMap, OCRAugmentationMap};
use std::collections::HashMap;

#[rustfmt::skip]
pub(crate) fn keyboard_fr() -> KBAugmentationMap {
    let kb_fr: HashMap<&'static str, Vec<&'static str>> = HashMap::from([
        ("&", vec!["1", "é", "2", "a", "z"]),
        ("é", vec!["2", "&", "1", "\"", "3", "a", "z", "e"]),
        ("\"", vec!["3", "é", "2", "'", "4", "z", "e"]),
        ("'", vec!["4", "\"", "3", "(", "5", "e", "r"]),
        ("(", vec!["5", "'", "4", "§", "6", "r", "t", "y"]),
        ("§", vec!["6", "(", "5", "è", "7", "t", "y", "u"]),
        ("è", vec!["7", "§", "6", "!", "8", "y", "u", "i"]),
        ("!", vec!["8", "è", "7", "ç", "9", "u", "i", "o"]),
        ("ç", vec!["9", "!", "8", "à", "0", "i", "o", "p"]),
        ("a", vec!["&", "1", "é", "2", "z", "q", "s"]),
        ("z", vec!["&", "1", "é", "2", "\"", "3", "a", "e", "q", "s", "d"]),
        ("e", vec!["é", "2", "\"", "3", "'", "4", "z", "r", "s", "d", "f"]),
        ("r", vec!["\"", "3", "'", "4", "(", "5", "e", "t", "d", "f", "g"]),
        ("t", vec!["'", "4", "(", "5", "§", "6", "r", "y", "f", "g", "h"]),
        ("y", vec!["(", "5", "§", "6", "è", "7", "t", "u", "g", "h", "j"]),
        ("u", vec!["§", "6", "è", "7", "!", "8", "i", "h", "j", "k"]),
        ("i", vec!["è", "7", "!", "8", "ç", "9", "u", "o", "j", "k", "l"]),
        ("o", vec!["!", "8", "ç", "9", "à", "0", "i", "p", "k", "l"]),
        ("p", vec!["ç", "9", "à", "0", "o", "l"]),
        ("q", vec!["a", "z", "q", "s", "w", "x"]),
        ("s", vec!["a", "z", "e", "q", "d", "w", "x", "c"]),
        ("d", vec!["z", "e", "r", "s", "f", "x", "c", "v"]),
        ("f", vec!["e", "r", "t", "d", "g", "c", "v", "b"]),
        ("g", vec!["r", "t", "y", "f", "h", "v", "b", "n"]),
        ("h", vec!["t", "y", "u", "g", "j", "b", "n", ","]),
        ("j", vec!["y", "u", "i", "h", "k", "n", ",", ";", "."]),
        ("k", vec!["u", "i", "o", "j", "l", ",", ";", ".", ", vec!", "/"]),
        ("l", vec!["i", "o", "p", "k", "m", "M", ";", ".", ", vec!", "/", "=", "+"]),
        ("w", vec!["q", "s", "x"]),
        ("x", vec!["q", "s", "d", "w", "c"]),
        ("c", vec!["s", "d", "f", "x", "v"]),
        ("v", vec!["d", "f", "g", "c", "b"]),
        ("b", vec!["f", "g", "h", "v", "n"]),
        ("n", vec!["g", "h", "j", "b", ","]),
        (",", vec!["h", "j", "k", "n", ";", "."]),
        ("1", vec!["2", "a"]),
        ("2", vec!["1", "3", "a", "z"]),
        ("3", vec!["2", "4", "z", "e"]),
        ("4", vec!["3", "5", "e", "r"]),
        ("5", vec!["4"])]
     );
    kb_fr
}

#[rustfmt::skip]
pub(crate) fn ocr_fr() -> OCRAugmentationMap {
    let ocr_fr: HashMap<&'static str, Vec<&'static str>> = HashMap::from([
        ("0", vec!["8", "9", "o", "O", "D"]),
        ("8", vec!["9", "o", "O", "D", "0"]),
        ("9", vec!["8", "o", "O", "D", "0"]),
        ("o", vec!["8", "9", "O", "D", "0"]),
        ("O", vec!["8", "9", "o", "D", "0"]),
        ("D", vec!["8", "9", "o", "O", "0"]),

        ("1", vec!["4", "7", "l", "I"]),
        ("4", vec!["1", "7", "l", "I"]),
        ("7", vec!["1", "4", "l", "I"]),
        ("l", vec!["1", "4", "7", "I"]),
        ("I", vec!["1", "4", "7", "l"]),

        ("2", vec!["z", "Z"]),
        ("z", vec!["2", "Z"]),
        ("Z", vec!["z", "2"]),

        ("5", vec!["8"]),
        ("8", vec!["5"]),

        ("6", vec!["b", "G"]),
        ("b", vec!["6", "G"]),
        ("G", vec!["b", "6"]),

        ("8", vec!["s", "S", "@", "&"]),
        ("s", vec!["8", "S", "@", "&"]),
        ("S", vec!["s", "8", "@", "&"]),
        ("@", vec!["s", "S", "8", "&"]),
        ("&", vec!["s", "S", "@", "8"]),

        ("9", vec!["g", "q"]),
        ("g", vec!["9", "q"]),
        ("q", vec!["g", "9"]),

        ("o", vec!["u"]),
        ("u", vec!["o"]),

        ("r", vec!["k"]),
        ("k", vec!["k"]),

        ("C", vec!["G"]),
        ("G", vec!["C"]),

        ("O", vec!["D", "U"]),
        ("D", vec!["O", "U"]),
        ("U", vec!["D", "O"]),

        ("E", vec!["B"]),
        ("B", vec!["E"]),

    ]);
    ocr_fr
}
