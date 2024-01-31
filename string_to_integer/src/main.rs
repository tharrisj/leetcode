struct Solution{}

impl Solution{
    pub fn my_atoi(s: String) -> i32 {
        let clean_str = s.trim_start_matches(|c: char| c.is_whitespace());
        let mut acc: i32 = 0;
        let mut pos: bool = true;
        let mut chars = clean_str.chars().peekable();

        if let Some(&ch) = chars.peek() {
            if ch == '-' { pos = false; chars.next(); }
            if ch == '+' { pos = true; chars.next(); }
        }

        for ch in chars {
            let add = match ch.to_digit(10) {
                Some(x) => x as i32,
                None => break,
            };
            acc = match acc.checked_mul(10) {
                Some(x) => x,
                None => { if pos {return i32::MAX} else {return i32::MIN} },
            };
            acc = match acc.checked_add(add) {
                Some(x) => x,
                None => { if pos {return i32::MAX} else {return i32::MIN} },
            };
        }

        if !pos { acc = acc * -1; }
        acc
    }
}

fn main() {
    let test = String::from("42");
    let ret = Solution::my_atoi(test);
    assert_eq!(ret, 42_i32);

    let test = String::from("   -42");
    let ret = Solution::my_atoi(test);
    assert_eq!(ret, -42_i32);

    let test = String::from("4193 with words");
    let ret = Solution::my_atoi(test);
    assert_eq!(ret, 4193_i32);

    let test = String::from("-91283472332");
    let ret = Solution::my_atoi(test);
    assert_eq!(ret, i32::MIN);

    let test = String::from("-2147483647");
    let ret = Solution::my_atoi(test);
    assert_eq!(ret, -2147483647_i32);

    println!("All tests passed!");
}
