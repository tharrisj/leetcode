struct Solution;

impl Solution{
    pub fn length_of_last_word(s: String) -> i32 {
        let s = String::from(s.trim_end());
        for (spot, ch) in s.chars().rev().enumerate() {
            if ch == ' ' {
                return spot as i32;
            }
        }

        s.len() as i32
    }
}

fn main() {
    let s = String::from("Hello World");
    let ret = Solution::length_of_last_word(s);
    assert_eq!(ret, 5);

    let s = String::from("    fly me     to    the moon  ");
    let ret = Solution::length_of_last_word(s);
    assert_eq!(ret, 4);

    let s = String::from("luffy is still joyboy");
    let ret = Solution::length_of_last_word(s);
    assert_eq!(ret, 6);

    println!("All tests passed!");
}