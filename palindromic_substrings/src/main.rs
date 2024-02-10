struct Solution{}

impl Solution{
    fn expand_cnt(left: usize, right: usize, chars: &Vec<char>) -> i32 {
        let mut ret = 0;
        let mut left = left;
        let mut right = right;

        loop {
            match (chars.get(left), chars.get(right)) {
                (Some(&lch), Some(&rch)) => {
                    if lch == rch { ret += 1; }
                    else { break; }
                    left = match left.checked_sub(1) {
                        Some(x) => x,
                        None => break,
                    };
                    right += 1;
                },
                _ => break,
            }
        }

        ret
    }

    pub fn count_substrings(s: String) -> i32 {
        let mut ret = 0;
        let chars: Vec<_> = s.chars().collect();

        for i in 0..chars.len() {
            ret += Self::expand_cnt(i, i, &chars);
            println!("Ret is {ret} at {i}");
            ret += Self::expand_cnt(i, i+1, &chars);
            println!("Ret is {ret} at {i}");
        }

        ret
    }
}

fn main() {
    let test = "abc".to_owned();
    let ret = Solution::count_substrings(test);
    assert_eq!(ret, 3);

    let test = "aaa".to_owned();
    let ret = Solution::count_substrings(test);
    assert_eq!(ret, 6);

    let test = "fdsklf".to_owned();
    let ret = Solution::count_substrings(test);
    assert_eq!(ret, 6);

    println!("All Tests Passed!");
}
