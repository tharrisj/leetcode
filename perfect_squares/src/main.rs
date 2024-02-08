struct Solution{}

impl Solution{
    fn check_for_2(n: i32) -> bool {
        for i in 1..((n as f64).sqrt() as i32 + 1) {
            let v = n - i * i;
            if ((v as f64).sqrt() as i32).pow(2) == v {
                return true;
            }
        }

        false
    }

    pub fn num_squares(n: i32) -> i32 {
        let mut num = n;
        loop {
            if num % 4 != 0 { break; }
            num /= 4;
        }

        if num % 8 == 7 { return 4; }
        if ((num as f64).sqrt() as i32).pow(2) == num { return 1; }
        if Self::check_for_2(num) { return 2; }

        3
    }
}

fn main() {
    let test = 12;
    let ret = Solution::num_squares(test);
    assert_eq!(ret, 3);

    let test = 13;
    let ret = Solution::num_squares(test);
    assert_eq!(ret, 2);

    println!("All tests passed!");
}
