struct Solution;

impl Solution{
    pub fn minimum_length(s: String) -> i32 {
        let s_chars = s.into_bytes();
        let (mut left, mut right) = (0 as usize, s_chars.len()-1);

        while left < right && s_chars[left] == s_chars[right] {
            let check = s_chars[left];
            while left < right-1 && s_chars[left+1] == check { left += 1; }
            while left < right-1 && s_chars[right-1] == check { right -= 1; }
            left += 1;
            right -= 1;
        }
        right as i32 - left as i32 + 1
    }
}

fn main() {
    let s = String::from("ca");
    let ret = Solution::minimum_length(s);
    assert_eq!(ret, 2);

    let s = String::from("cabaabac");
    let ret = Solution::minimum_length(s);
    assert_eq!(ret, 0);

    let s = String::from("aabccabba");
    let ret = Solution::minimum_length(s);
    assert_eq!(ret, 3);

    println!("All Tests Passed!");
}