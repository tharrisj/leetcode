struct Solution;

impl Solution{
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        
        for i in 0..nums.len() {
            if i as i32 != nums[i] {
                return i as i32;
            }
        }
        nums.len() as i32
    }
}

fn main() {
    let test = vec![3,0,1];
    let ret = Solution::missing_number(test);
    assert_eq!(ret, 2);

    let test = vec![0,1];
    let ret = Solution::missing_number(test);
    assert_eq!(ret, 2);

    let test = vec![9,6,4,2,3,5,7,0,1];
    let ret = Solution::missing_number(test);
    assert_eq!(ret, 8);

    println!("All tests passed!");
}