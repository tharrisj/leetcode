struct Solution;

impl Solution{
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut lengths: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        let mut zeros = 0;
        let mut ones = 0;
        let mut max_len = 0;
        lengths.insert(0, -1);

        for (key, &val) in nums.iter().enumerate() {
            if val == 0 { zeros += 1; }
            else { ones += 1; }

            let diff = zeros - ones;
            if lengths.contains_key(&diff) {
                max_len = max_len.max(key as i32 - lengths[&diff]);
            } else {
                lengths.insert(diff, key as i32);
            }
        }

        max_len
    }
}

fn main() {
    let nums = vec![0,1];
    let ret = Solution::find_max_length(nums);
    assert_eq!(ret, 2);

    let nums = vec![0,1,0];
    let ret = Solution::find_max_length(nums);
    assert_eq!(ret, 2);

    println!("All tests passed!");
}