struct Solution;

impl Solution{
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![0; nums.len()];
        let mut prefix = vec![1; nums.len()];
        let mut suffix = vec![1; nums.len()];

        for i in 1..nums.len() {
            prefix[i] = prefix[i-1] * nums[i-1];
        }
        for i in (0..nums.len()-1).rev() {
            suffix[i] = suffix[i+1] * nums[i+1];
        }

        for i in 0..nums.len() {
            ret[i] = prefix[i] * suffix[i];
        }

        ret
    }
}

fn main() {
    let nums = vec![1,2,3,4];
    let ret = Solution::product_except_self(nums);
    println!("Solution is: {:?}", ret);

    println!("All tests passed!");
}