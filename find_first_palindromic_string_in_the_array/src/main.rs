struct Solution{}

impl Solution{
    fn is_palindrome(word: &str) -> bool {
        let chars: Vec<char> = word.chars().collect();
        let (mut left, mut right) = (chars.len()/2, chars.len()/2);
        if chars.len() % 2 == 0 { left -= 1; }
        
        loop {
            match (chars.get(left), chars.get(right)) {
                (Some(l), Some(r)) => {
                    if l != r { return false; }
                    left = match left.checked_sub(1) {
                        Some(i) => i,
                        None => break,
                    };
                    right += 1;
                }
                _ => {
                    break;
                }
            }
        }

        true
    }

    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in words {
            if Solution::is_palindrome(&word) { return word.to_owned(); }
        }

        "".to_owned()
    }
}

fn main() {
    let test: Vec<String> = vec![String::from("abc"), String::from("car"), String::from("ada"), String::from("racecar"), String::from("cool")];
    let ret = Solution::first_palindrome(test);
    assert_eq!(ret, String::from("ada"));

    println!("All tests passed!");
}
