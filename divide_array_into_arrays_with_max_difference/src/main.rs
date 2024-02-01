struct Solution{}

impl Solution{
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut res: Vec<Vec<i32>> = vec!();

        for i in nums.chunks(3){
            if i[2] - i[0] <= k {
                res.push(vec!(i[0], i[1], i[2]));
            } else {
                return vec!(vec!());
            }
        }
        
        res
    }
}

fn main() {
    let test = vec!(1,3,4,8,7,9,3,5,1);
    let k = 2;
    let ret = Solution::divide_array(test, k);
    let solution = vec!(vec!(1,1,3), vec!(3,4,5), vec!(7,8,9));
    assert_eq!(ret, solution);

    let test = vec!(1,3,3,2,7,3);
    let k = 3;
    let ret = Solution::divide_array(test, k);
    let solution = vec!(vec!());
    assert_eq!(ret, solution);

    println!("All tests passed!");
}
