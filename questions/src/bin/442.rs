struct Solution;

impl Solution{
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32>{
        let mut nums = nums;
        let mut ret = vec![];

        for pos in 0..nums.len() {
            let num = nums[pos];
            if nums[(num.abs() - 1) as usize].is_negative() {
                ret.push(nums[pos].abs());
            } else {
                nums[(num.abs() -1) as usize] *= -1;
            }
        }

        ret
    }
}

fn main() {
    let nums = vec![4,3,2,7,8,2,3,1];
    let ret = Solution::find_duplicates(nums);
    assert_eq!(ret, vec![2,3]);

    let nums = vec![1,1,2];
    let ret = Solution::find_duplicates(nums);
    assert_eq!(ret, vec![1]);

    let nums = vec![];
    let ret = Solution::find_duplicates(nums);
    assert_eq!(ret, vec![]);

    println!("All tests passed!");
}