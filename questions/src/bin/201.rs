struct Solution;

impl Solution{
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut left = left;
        let mut right = right;
        let mut shift = 0;

        while left != right {
            left >>= 1;
            right >>= 1;
            shift += 1;
        }
        left << shift
    }
}

fn main() {
    let (left, right) = (5, 7);
    let ret = Solution::range_bitwise_and(left, right);
    assert_eq!(ret, 4);

    let (left, right) = (0, 0);
    let ret = Solution::range_bitwise_and(left, right);
    assert_eq!(ret, 0);

    let (left, right) = (1, 2147483647);
    let ret = Solution::range_bitwise_and(left, right);
    assert_eq!(ret, 0);

    println!("All tests passed!");
}