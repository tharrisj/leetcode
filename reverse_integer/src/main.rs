struct Solution{}

impl Solution{
    pub fn reverse(x: i32) -> i32 {
        let mut num: i32 = x;
        let mut acc: i32 = 0;
        while num != 0 {
            acc = match acc.checked_mul(10) {
                Some(i) => i,
                None => { return 0; }
            };
            acc = match acc.checked_add(num % 10) {
                Some(i) => i,
                None => { return 0; }
            };
            num /= 10;
        }
        acc
    }
}

fn main() {
    let test = 123;
    let ret = Solution::reverse(test);
    println!("The reverse of {test} is {ret}");

    let test = -123;
    let ret = Solution::reverse(test);
    println!("The reverse of {test} is {ret}");
}
