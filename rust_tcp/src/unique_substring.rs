use std::{
    collections::{BTreeMap, HashMap, HashSet},
    str::Chars,
    time::SystemTime,
};
pub fn find_longest_unique_substring(
    string: &[char],
    // char_indexes: &'a mut [Option<usize>],
    // offset: usize,
) -> &[char] {
    let mut char_indexes: BTreeMap<&char, usize> = BTreeMap::new();
    // let now = SystemTime::now();
    // // char_indexes.fill(None);
    // println!(
    //     "!! {}  {}",
    //     char_indexes.len(),
    //     now.elapsed().unwrap().as_nanos()
    // );
    let str_len = string.len();
    let mut match_index = 0_usize;
    let mut match_length = 0_usize;
    let mut index = 0_usize;
    let mut is_the_end = false;
    let mut length = 0_usize;
    let mut prev_char_index: usize = 0_usize;
    let mut char_exist = false;
    let char_index = 0_usize;
    // println!("P{}", char_indexes.len());
    for (char_index, char) in string.iter().enumerate() {
        char_exist = false;
        if let Some(prev_char_index_) = char_indexes.get(char) {
            if prev_char_index_ >= &index && prev_char_index_ < &char_index {
                char_exist = true;
                prev_char_index = *prev_char_index_;
            }
        }
        // is_the_end = char_index == str_len - 1;

        if char_exist || char_index == str_len - 1 {
            length = char_index - index;

            if !char_exist {
                // if it's the end and it's not a duplicate
                length += 1;
            }

            if length > match_length {
                match_index = index;
                match_length = length;
            }

            if char_exist {
                index = prev_char_index + 1;
            }
        }

        {
            char_indexes.insert(char, char_index);
        }
    }
    match (match_index, match_length) {
        (0, 0) => &[' '; 0],
        _ => &string[match_index..(match_length + match_index)],
    }
}
pub fn find_longest_unique_substring2(
    string: &[char],
    // char_indexes: &'a mut [Option<usize>],
    // offset: usize,
) -> &[char] {
    let mut char_indexes: BTreeMap<&char, usize> = BTreeMap::new();
    // let now = SystemTime::now();
    // // char_indexes.fill(None);
    // println!(
    //     "!! {}  {}",
    //     char_indexes.len(),
    //     now.elapsed().unwrap().as_nanos()
    // );
    let str_len = string.len();
    let mut match_index = 0_usize;
    let mut match_length = 0_usize;
    let mut index = 0_usize;
    let mut is_the_end = false;
    let mut length = 0_usize;
    let mut prev_char_index: usize = 0_usize;
    let mut char_exist = false;
    let char_index = 0_usize;
    // println!("P{}", char_indexes.len());
    for (char_index, char) in string.iter().enumerate() {
        char_exist = false;
        if let Some(prev_char_index_) = char_indexes.get(char) {
            if prev_char_index_ >= &index && prev_char_index_ < &char_index {
                char_exist = true;
                prev_char_index = *prev_char_index_;
            }
        }
        // is_the_end = char_index == str_len - 1;

        if char_exist || char_index == str_len - 1 {
            length = char_index - index;

            if !char_exist {
                // if it's the end and it's not a duplicate
                length += 1;
            }

            if length > match_length {
                match_index = index;
                match_length = length;
            }

            if char_exist {
                index = prev_char_index + 1;
            }
        }

        {
            char_indexes.insert(char, char_index);
        }
    }
    match (match_index, match_length) {
        (0, 0) => &[' '; 0],
        _ => &string[match_index..(match_length + match_index)],
    }
}
pub fn find_longest_unique_substring_slow(string: &str) -> String {
    let str_len = string.len();
    let mut match_index = 0_usize;
    let mut match_length = 0_usize;

    let mut windows_index = 0_usize;
    for (char_index, char) in string.chars().enumerate() {
        let exist = string
            .chars()
            .skip(windows_index)
            .take(char_index)
            .any(|f| f == char);

        if exist {
            if match_length > char_index - windows_index {
                match_index = windows_index;
                match_length = char_index - windows_index;
            }
            windows_index += 1;
        }
    }
    match (match_index, match_length) {
        (0, 0) => "None".to_string(),
        _ => string
            .chars()
            .skip(match_index)
            .take(match_length)
            .collect::<String>(),
    }
}

pub fn find_longest_unique_substring_alt(
    string: &str,
    char_indexes: &mut BTreeMap<char, usize>,
) -> Option<String> {
    // char_indexes.clear();
    let str_len = string.len();
    let mut chars = string.chars();
    let mut match_index = 0_usize;
    let mut match_length = 0_usize;
    let mut index = 0_usize;
    for (char_index, char) in string.chars().enumerate() {
        let char_exist = match char_indexes.get(&char) {
            Some(i) => i >= &index && i < &char_index,
            None => false,
        };
        let is_the_end = char_index == str_len - 1;

        if char_exist || is_the_end {
            let mut length = char_index - index;

            if !char_exist {
                // if it's the end and it's not a duplicate
                length += 1;
            }

            if length > match_length {
                match_index = index;
                match_length = length;
            }
            if char_exist {
                index = char_indexes.get(&char).unwrap() + 1;
            }
        }
        char_indexes.insert(char, char_index);
    }
    match (match_index, match_length) {
        (0, 0) => None,
        _ => Some(
            string
                .chars()
                .skip(match_index)
                .take(match_length)
                .collect::<String>(),
        ),
    }
}
