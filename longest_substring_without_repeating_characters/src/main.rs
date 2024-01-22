use std::collections::HashMap;
use rand::{distributions, Rng};

// TODO: improve by keeping track of the largest difference in character positions and returning the largest to get the largest substring
fn length_of_longest_substring(s: String) -> i32 {
    let mut max_length: i32 = 0;
    let mut unique_vals: HashMap<char, usize> = HashMap::new();
    let mut iter1 = s.chars().enumerate();

    while let Some((i, _)) = iter1.next() {
        let slice = &s[i..];
        let mut length: i32 = 0;
        let mut iter2 = slice.chars().enumerate();
        while let Some((j, ch2)) = iter2.next() {
            match unique_vals.get(&ch2) {
                Some(_) => {
                    unique_vals.clear();
                    break;
                }
                None => {
                    unique_vals.insert(ch2, j);
                    length += 1;
                }
            }
            if length > max_length { max_length = length; }
        }
    }

    max_length
}

fn main() {
    let length: usize = 100;
    let distro = &distributions::Alphanumeric;
    let test_string: String = rand::thread_rng()
        .sample_iter(distro)
        .take(length)
        .map(char::from)
        .collect();

    println!("Test String: {}", test_string);
    let longest = length_of_longest_substring(test_string);
    println!("Longest is: {}", longest);

    let test_string: String = "abcabcbb".to_owned();
    let longest = length_of_longest_substring(test_string);
    println!("Testing if this is correct! {}", longest);
    assert_eq!(longest, 3);

    let test_string: String = "bbbbb".to_owned();
    let longest = length_of_longest_substring(test_string);
    println!("Testing if this is correct! {}", longest);
    assert_eq!(longest, 1);

    let test_string: String = "pwwkew".to_owned();
    let longest = length_of_longest_substring(test_string);
    println!("Testing if this is correct! {}", longest);
    assert_eq!(longest, 3);
}
