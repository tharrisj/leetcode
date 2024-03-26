struct Solution;

impl Solution{
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let len = nums.len();

        for n in &mut nums {
            if *n <= 0 {
                *n = i32::MAX;
            }
        }

        for i in 0..len {
            let n = (nums[i].abs() - 1) as usize;
            if n < len {
                nums[n] = nums[n].abs() * -1;
            }
        }

        for (pos, n) in nums.iter().enumerate() {
            if n.is_positive() {
                return (pos + 1) as i32;
            }
        }

        (len + 1) as i32
    }
}

fn main() {
    let nums = vec![1,2,0];
    let ret = Solution::first_missing_positive(nums);
    assert_eq!(ret, 3);

    let nums = vec![3,4,-1,1];
    let ret = Solution::first_missing_positive(nums);
    assert_eq!(ret, 2);

    let nums = vec![7,8,9,11,12];
    let ret = Solution::first_missing_positive(nums);
    assert_eq!(ret, 1);

    println!("All tests passed!");
}