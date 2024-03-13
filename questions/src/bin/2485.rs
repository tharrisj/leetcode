struct Solution;

impl Solution{
    pub fn pivot_integer(n: i32) -> i32 {
        let mut left_sum = n * (n+1)/2;
        let mut right_sum = 0;
        for x in (n/2..=n).rev() {
            left_sum -= x;
            if left_sum == right_sum { return x; }
            right_sum += x;
            if left_sum < right_sum { break; }
        }

        -1
    }
}

fn main() {
    let n = 8;
    let ret = Solution::pivot_integer(n);
    assert_eq!(ret, 6);

    let n = 1;
    let ret = Solution::pivot_integer(n);
    assert_eq!(ret, 1);

    let n = 4;
    let ret = Solution::pivot_integer(n);
    assert_eq!(ret, -1);
    
    println!("All Tests Passed!");
}