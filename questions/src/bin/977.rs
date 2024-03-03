struct Solution;

impl Solution{
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.iter_mut().for_each(|x| *x *= *x);
        nums.sort_unstable();
        nums
    }

    pub fn sorted_squares_better(nums: Vec<i32>) -> Vec<i32> {
        let mut ret: Vec<i32> = vec![];

        let mut stack: Vec<i32> = vec![];
        for x in nums {
            let num = x * x;
            if x.is_negative() {
                stack.push(num);
                continue;
            }

            while stack.len() > 0 && num > *stack.get(stack.len()-1).unwrap_or(&0) {
                let x = stack.pop().unwrap();
                ret.push(x);
            }

            ret.push(num);
        }

        if stack.len() > 0 {
            ret = ret.into_iter().chain(stack.into_iter().rev()).collect();
        }

        ret
    }
}

fn main() {
    let test = vec![-4,-1,0,3,10];
    let ret = Solution::sorted_squares(test);
    assert_eq!(ret, vec![0,1,9,16,100]);

    let test = vec![-4,-1,0,3,10];
    let ret = Solution::sorted_squares_better(test);
    assert_eq!(ret, vec![0,1,9,16,100]);

    println!("All Tests Passed!");
}