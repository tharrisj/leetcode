struct Solution{}

impl Solution{
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { return false; }
        if x < 10 { return true; }
        let mut num = x;
        let mut digits: [i32;10] = [0;10];
        let mut pos: usize = 0;

        while num > 0 {
            digits[pos] = num % 10;
            num /= 10;
            pos += 1;
        }

        for i in 0..=pos.div_ceil(2) {
            if digits[i] != digits[pos-1-i] { return false; }
        }

        true
    }
}

fn main() {
    let test = 121;
    let ret = Solution::is_palindrome(test);
    assert_eq!(ret, true);

    let test = -121;
    let ret = Solution::is_palindrome(test);
    assert_eq!(ret, false);

    let test = 10;
    let ret = Solution::is_palindrome(test);
    assert_eq!(ret, false);

    let test = 0;
    let ret = Solution::is_palindrome(test);
    assert_eq!(ret, true);

    let test = 1;
    let ret = Solution::is_palindrome(test);
    assert_eq!(ret, true);

    println!("All tests succeeded");
}
