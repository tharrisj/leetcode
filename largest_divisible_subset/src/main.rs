struct Solution{}

impl Solution{
    pub fn check_if_pow(base: i32, num: i32) -> bool {
        if base == 1 { return true; }
        let num = num as f64;
        let ans = num.log(base as f64);
        ans == ((ans as i32) as f64)
    }

    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ret: Vec<i32> = vec!();

        while nums.len() > 0 {
            let check_num = nums[0];
            let mut cur_check: Vec<i32> = vec!(check_num);
            nums.remove(0);
            let mut i = 0;
            while i < nums.len() {
                if Self::check_if_pow(check_num, nums[i]) {
                    cur_check.push(nums[i]);
                    nums.remove(i);
                } else {
                    i += 1;
                }
            }

            if ret.len() < cur_check.len() { ret = cur_check; }

        }

        ret
    }
}

fn main() {
    let test = vec!(1,2,3);
    let ret = Solution::largest_divisible_subset(test);
    assert_eq!(ret, vec!(1,2));

    let test = vec!(1,2,4,8);
    let ret = Solution::largest_divisible_subset(test);
    assert_eq!(ret, vec!(1,2,4,8));
}
