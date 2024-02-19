struct Solution;

impl Solution{
    pub fn is_power_of_two(n: i32) -> bool {
        dbg!((n as f64).log2());
        dbg!((((n as f64).log2()) as i32) as f64);
        (n as f64).log2() == (((n as f64).log2()) as i32) as f64
    }
}

fn main() {
    let test = 1;
    let ret = Solution::is_power_of_two(test);
    assert_eq!(ret, true);

    let test = 16;
    let ret = Solution::is_power_of_two(test);
    assert_eq!(ret, true);

    let test = 3;
    let ret = Solution::is_power_of_two(test);
    assert_eq!(ret, false);

    let test = 2097151;
    let ret = Solution::is_power_of_two(test);
    assert_eq!(ret, false);

    println!("All tests passed!");
}