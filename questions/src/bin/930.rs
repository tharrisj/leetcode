struct Solution;

impl Solution{
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut ret = 0;
        let mut acc = 0;

        for i in nums[0..].iter().enumerate() {
            acc += i.1;
            if acc == goal { ret += 1; }
            for j in nums[(i.0 + 1)..].iter() {
                acc += j;
                if acc == goal { ret += 1; }
            }
            acc = 0;
        }

        ret
    }
}

fn main() {
    let nums = vec![1,0,1,0,1];
    let goal = 2;
    let ret = Solution::num_subarrays_with_sum(nums, goal);
    assert_eq!(ret, 4);

    let nums = vec![0,0,0,0,0];
    let goal = 0;
    let ret = Solution::num_subarrays_with_sum(nums, goal);
    assert_eq!(ret, 15);

    println!("All tests passed!");
}