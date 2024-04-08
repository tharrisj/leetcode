struct Solution;

impl Solution{
    pub fn check_valid_string(s: String) -> bool {
        let mut counts = [0_i8; 2];

        for &ch in s.as_bytes() {
            if ch == b'(' {
                counts[0] += 1;
            } else {
                counts[0] -= 1;
            }
            if ch != b')' {
                counts[1] += 1;
            } else {
                counts[1] -= 1;
            }

            if counts[1] < 0 {
                return false;
            }
            counts[0] = counts[0].max(0);
        }

        counts[0] == 0
    }
}

fn main() {
    let s = String::from("()");
    let ret = Solution::check_valid_string(s);
    assert_eq!(ret, true);

    let s = String::from("(*)");
    let ret = Solution::check_valid_string(s);
    assert_eq!(ret, true);

    let s = String::from("(*))");
    let ret = Solution::check_valid_string(s);
    assert_eq!(ret, true);

    let s = String::from("(((((*(()((((*((**(((()()*)()()()*((((**)())*)*)))))))(())(()))())((*()()(((()((()*(())*(()**)()(())");
    let ret = Solution::check_valid_string(s);
    assert_eq!(ret, false);

    println!("All tests passed!");
}