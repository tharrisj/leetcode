struct Solution{}

impl Solution{
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = 0;

        for i in nums {
            if count == 0 { candidate = i }
            if i == candidate { count += 1; }
            else { count -= 1; }

        }

        candidate
    }
}

fn main() {
    let test = vec!(3,2,3);
    let ret = Solution::majority_element(test);
    assert_eq!(ret, 3);

    let test: Vec<i32> = vec!(2,2,1,1,1,2,2);
    let ret = Solution::majority_element(test);
    assert_eq!(ret, 2);

    let test: Vec<i32> = vec!(1);
    let ret = Solution::majority_element(test);
    assert_eq!(ret, 1);

    println!("All tests passed!");
}
