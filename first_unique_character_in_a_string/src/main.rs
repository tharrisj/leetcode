struct Solution{}

const CHAR_OFFSET: usize = b'a' as usize;

impl Solution{
    pub fn first_uniq_char(s: String) -> i32 {
        let mut ret: i32 = -1;
        let mut seen = [0; 26];

        for (i, ch) in s.chars().enumerate() {
            if seen[ch as usize - CHAR_OFFSET] == 0 {
                seen[ch as usize - CHAR_OFFSET] = i as i32 + 1;
            } else if seen[ch as usize - CHAR_OFFSET] > 0 {
                seen[ch as usize - CHAR_OFFSET] = -1_i32;
            }
        }

        if let Some(&spot) = seen.iter()
            .filter(|&&x| x > 0)
            .min() {
            ret = spot - 1;
        }

        ret
    }
}

fn main() {
    let test = b'z' - b'a';
    println!("Test is {test}");

    let s = "leetcode".to_owned();
    let ret = Solution::first_uniq_char(s);
    assert_eq!(ret, 0);

    let s = "loveleetcode".to_owned();
    let ret = Solution::first_uniq_char(s);
    assert_eq!(ret, 2);

    let s = "aabb".to_owned();
    let ret = Solution::first_uniq_char(s);
    assert_eq!(ret, -1);

    println!("All tests passed!");
}
