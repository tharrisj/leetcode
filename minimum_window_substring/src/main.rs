struct Solution{}

const CHAR_START_INDEX: usize = 65;
const NUM_OF_CHARS: usize = 58;

impl Solution{
    pub fn min_window(s: String, t: String) -> String {
        let mut ret = String::new();
        let s_chars: Vec<char> = s.chars().collect();
        let (mut start, mut end) = (0,0);

        let mut tarr = [0; NUM_OF_CHARS];
        for ch in t.chars() {
            tarr[ch as usize - CHAR_START_INDEX] += 1;
        }

        while start < s_chars.len()+1 && end < s_chars.len()+1 {
            // We found a character that still needs to be included
            if tarr.iter().any(|&x| x > 0) {
                if let Some(&cur_char) = s_chars.get(end) {
                    tarr[cur_char as usize - CHAR_START_INDEX] -= 1;
                };
                end += 1;
            } else {
                // We have a substring that is smaller than return
                if ret.len() == 0 || ret.len() > end - start {
                    ret = s[start..end].to_owned();
                }
                if let Some(&cur_char) = s_chars.get(start) {
                    tarr[cur_char as usize - CHAR_START_INDEX] += 1;
                };
                start += 1;
            }
        }

        ret
    }
}

fn main() {
    let s = "ADOBECODEBANC".to_owned();
    let t = "ABC".to_owned();
    let ret = Solution::min_window(s, t);
    assert_eq!(ret, "BANC");

    let s = "a".to_owned();
    let t = "a".to_owned();
    let ret = Solution::min_window(s, t);
    assert_eq!(ret, "a");

    let s = "a".to_owned();
    let t = "aa".to_owned();
    let ret = Solution::min_window(s, t);
    assert_eq!(ret, "");

    let s = "ab".to_owned();
    let t = "b".to_owned();
    let ret = Solution::min_window(s, t);
    assert_eq!(ret, "b");

    println!("All tests passed!");
}
