fn is_palindrome(s: &str) -> bool {
    let (beginning, end) = match s.len() % 2 {
        0 => (&s[0..s.len() / 2], &s[s.len() / 2..s.len()]),
        _ => (&s[0..s.len() / 2], &s[s.len() / 2 + 1..s.len()]),
    };

    return beginning.chars().eq(end.chars().rev());
}

fn max_palindrome_around(s: &String, p_start: usize, p_end: usize) -> &str {
    let mut max_palindrome = &s[p_start..p_end];


    let mut width: usize = 0;
    while (p_start as i32 - width as i32 >= 0) && (p_end + width <= s.len()) && is_palindrome(&s[p_start - width..p_end + width]) {
        max_palindrome = &s[p_start - width..p_end + width];
        width += 1;
    }

    return max_palindrome;
}

pub fn longest_palindrome(s: String) -> String {
    let mut max_palindrome = &s[0..1];

    for i in 0..s.len() {
        let odd_palindrome = max_palindrome_around(&s, i, i + 1);
        if max_palindrome.len() < odd_palindrome.len() {
            max_palindrome = odd_palindrome;
        }
        if i < s.len() - 1 && s[i..i+1] == s[i+1..i+2] {
            let even_palindrome = max_palindrome_around(&s, i, i + 2);
            if max_palindrome.len() < even_palindrome.len() {
                max_palindrome = even_palindrome;
            }
        }
    }

    return max_palindrome.to_string();
}

fn main() {
    let s = String::from("babad");
    let result = longest_palindrome(s);
    println!("Result => {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_short_string() {
        assert_eq!(String::from("abba"), longest_palindrome(String::from("xadabbaxyz")));
        assert_eq!(String::from("bab"), longest_palindrome(String::from("babad")));
        assert_eq!(String::from("bb"), longest_palindrome(String::from("cbbd")));
    }

}
