struct Solution;

impl Solution{
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
//        let mut counts: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        let mut counts: std::collections::HashMap<i32, i32> = std::collections::HashMap::with_capacity(nums.len());
        let mut ret = 0;
        let (mut left, mut right) = (0, 0);
        let mut valid = true;

        while right < nums.len() && left < nums.len() {
            if valid {
                let num = nums[right];
                let entry = counts.entry(num).or_insert(0);
                *entry += 1;
                if *entry > k {
                    valid = false;
                }
            } else {
                let num = nums[left];
                let entry = counts.entry(num).or_insert(0);
                *entry -= 1;
                left += 1;
                valid = !counts.values().any(|&x| x > k);
            }
            
            if valid {
                ret = ret.max(right as i32 - left as i32 + 1);
                right += 1;
            }
        }        

        ret
    }
}

fn main() {
    let nums = vec![1,2,3,1,2,3,1,2];
    let k = 2;
    let ret = Solution::max_subarray_length(nums, k);
    assert_eq!(ret, 6);

    let nums = vec![1,2,1,2,1,2,1,2];
    let k = 1;
    let ret = Solution::max_subarray_length(nums, k);
    assert_eq!(ret, 2);

    let nums = vec![5,5,5,5,5,5,5];
    let k = 4;
    let ret = Solution::max_subarray_length(nums, k);
    assert_eq!(ret, 4);

    let nums = vec![2,2,3];
    let k = 1;
    let ret = Solution::max_subarray_length(nums, k);
    assert_eq!(ret, 2);

    println!("All tests passed!");
}