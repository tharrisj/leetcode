use std::collections::HashMap;

struct Solution{}

impl Solution {
    fn is_palindrome(s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        for (i, &ch) in chars.iter().enumerate() {
            if  ch != chars[chars.len() - 1 - i] { return false; }
            if i >= chars.len()/2 { break; }
        }

        true
    }

    pub fn largest_palindrome(s: String) -> String {
//        let mut largest_str = String::from(&s[0..1]);
        let mut largest_str: String = String::new();
        let mut char_pos: HashMap<char, Vec<usize>> = HashMap::new();

        for (i, ch) in s.chars().enumerate() {
            // get the positions this character has been in before
            let positions = char_pos.entry(ch).or_insert(Vec::new());
            positions.push(i);

            // check all string slices between those previous characters to see if they are palindromes
            for &pos in positions.as_slice() {
                let str_len = i-pos+1;
                if str_len > largest_str.len() {
                    if Self::is_palindrome(&s[pos..=i]) {
                        largest_str = s[pos..=i].to_owned();
                    }
                }
            }
//            positions.push(i);
        }
    
        largest_str
    }

}


fn main() {
    let test_string = String::from("babad");
    let long_pal = Solution::largest_palindrome(test_string.clone());
    println!("Longest palindrome for {} is {}", test_string, long_pal);
    assert_eq!(long_pal, String::from("bab"));

    let test_string = String::from("cbbd");
    let long_pal = Solution::largest_palindrome(test_string.clone());
    println!("Longest palindrome for {} is {}", test_string, long_pal);
    assert_eq!(long_pal, String::from("bb"));

    let test_string = String::from("a");
    let long_pal = Solution::largest_palindrome(test_string.clone());
    println!("Longest palindrome for {} is {}", test_string, long_pal);
    assert_eq!(long_pal, String::from("a"));
}
