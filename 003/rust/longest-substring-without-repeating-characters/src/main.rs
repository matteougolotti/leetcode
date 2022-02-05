use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut characters: HashMap<char, usize> = HashMap::new();
    let mut max = 0;

    let mut start: usize = 0;
    for (i, c) in s.chars().enumerate() {
        if characters.contains_key(&c) {
            if let Some(end) = characters.get(&c) {
                let end: usize = *end;
                if let Some(substring_to_remove) = s.get(start..end) {
                    for char_to_remove in substring_to_remove.chars() {
                        characters.remove(&char_to_remove);
                    }
                }
                start = end + 1;
            }
        }

        characters.insert(c, i);
        max = std::cmp::max(max, characters.len() as i32);
    }

    return max;
}

fn main() {
    let s = String::from("abcabcbb");
    let result = length_of_longest_substring(s);
    println!("Result => {}", result);
}
