struct Solution;

impl Solution{
    pub fn make_good(s: String) -> String {
        let diff = b'a' - b'A';
        let mut stack: Vec<u8> = vec![];

        for &ch in s.as_bytes() {
            stack.push(ch);
            if stack.len() > 1 {
                if stack[stack.len() - 1].abs_diff(stack[stack.len() - 2]) == diff {
                    stack.pop();
                    stack.pop();
                }
            }
        }

        String::from_utf8(stack).unwrap_or(String::from(""))
    }
}

fn main() {
    let s = String::from("leEeetcode");
    let ret = Solution::make_good(s);
    assert_eq!(ret, String::from("leetcode"));

    let s = String::from("abBAcC");
    let ret = Solution::make_good(s);
    assert_eq!(ret, String::from(""));

    let s = String::from("s");
    let ret = Solution::make_good(s);
    assert_eq!(ret, String::from("s"));

    println!("All tests passed!");
}