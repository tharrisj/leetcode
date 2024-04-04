struct Solution;

impl Solution{
    pub fn max_depth(s: String) -> i32 {
        let mut max_depth = 0;
        let mut cur_depth = 0;

        for ch in s.chars(){
            if ch == '(' {
                cur_depth += 1;
            }
            if ch == ')' {
                cur_depth -= 1;
            }
            max_depth = max_depth.max(cur_depth);
        }
        max_depth
    }
}

fn main() {
    let s = String::from("(1+(2*3)+((8)/4))+1");
    let ret = Solution::max_depth(s);
    assert_eq!(ret, 3);

    let s = String::from("(1+(2*3)+((8)/4))+1");
    let ret = Solution::max_depth(s);
    assert_eq!(ret, 3);

    println!("All tests passed!");
}