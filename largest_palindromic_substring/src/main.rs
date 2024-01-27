struct Solution{}

// TODO: Implement better solution. I could just take each character and attempt to expand to each side to find longest palindrome.
impl Solution {
    pub fn largest_palindrome(s: String) -> String {
        let str: Vec<char> = s.chars().collect();
        let n = str.len();
        let mut l_pal: String = String::new();
        
        for i in 0..n {
            let (mut left, mut right) = (i, i);
            loop {
                match (str.get(left), str.get(right)) {
                    (Some(lch), Some(rch)) => {
                        if lch == rch {
                            if right - left + 1 > l_pal.len() {
                                l_pal = str[left..=right].iter().collect();
                            }
                        } else { break; }
                    },
                    _ => (),
                }
                if left == 0 || right == n-1 { break ;}
                left -= 1;
                right += 1;
            }
        
            let (mut left, mut right) = (i, i+1);
            loop {
                match (str.get(left), str.get(right)) {
                    (Some(lch), Some(rch)) => {
                        if lch == rch {
                            if right - left + 1 > l_pal.len() {
                                l_pal = str[left..=right].iter().collect();
                            }
                        } else { break; }
                    },
                    _ => (),
                }
                if left == 0 || right == n-1 { break ;}
                left -= 1;
                right += 1;
            }
        }
        
        l_pal
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
