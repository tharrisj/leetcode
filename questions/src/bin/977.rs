struct Solution;

impl Solution{
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.iter_mut().for_each(|x| *x *= *x);
        nums.sort_unstable();
        nums
    }
}

fn main() {
    let test = vec![-4,-1,0,3,10];
    let ret = Solution::sorted_squares(test);
    assert_eq!(ret, vec![0,1,9,16,100]);

    println!("All Tests Passed!");
}