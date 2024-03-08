struct Solution;

impl Solution{
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut counts = [0; 101];
        let mut max_count = 0;
        let mut max_num = 0;

        for x in nums{
            counts[x as usize] += 1;
            if counts[x as usize] > max_count {
                max_count = counts[x as usize];
                max_num = max_count;
            } else if counts[x as usize] == max_count {
                max_num += max_count
            }
        }

        max_num
    }
}

fn main() {
    let nums = vec![1,2,2,3,1,4];
    let ret = Solution::max_frequency_elements(nums);
    assert_eq![ret, 4];

    let nums = vec![1,2,3,4,5];
    let ret = Solution::max_frequency_elements(nums);
    assert_eq![ret, 5];

    let nums = vec![1,1,1,1];
    let ret = Solution::max_frequency_elements(nums);
    assert_eq![ret, 4];

    let nums = vec![6,13,15,15,11,6,7,12,4,11]; // 4,6,6,7,11,11,12,13,15,15
    let ret = Solution::max_frequency_elements(nums);
    assert_eq![ret, 6];

    println!("All Tests Passed!");
}