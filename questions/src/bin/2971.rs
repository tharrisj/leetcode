struct Solution;

impl Solution{
    pub fn largest_perimeter(nums: Vec<i32>) -> i64 {
        if nums.len() < 3 { return -1; }
        let ret: i64 = -1;
        let mut nums = nums;
        nums.sort_unstable();
        let mut cur_sum: i64 = nums.iter().map(|&x| x as i64).sum();

        for i in (2..nums.len()).rev() {
            cur_sum -= nums[i] as i64;
            if cur_sum > nums[i] as i64 {
                return cur_sum + nums[i] as i64;
            }
        }

        ret
    }
}

fn main() {
    let test = vec![5,5,5];
    let ret = Solution::largest_perimeter(test);
    assert_eq!(ret, 15);

    println!("All tests Passed!");
}