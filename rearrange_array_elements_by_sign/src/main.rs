struct Solution;

impl Solution{
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut pos: Vec<i32> = vec!();
        let mut neg: Vec<i32> = vec!();
        let mut ret: Vec<i32> = vec!();
        
        for &i in &nums {
            if i.is_positive() {
                pos.push(i);
            } else {
                neg.push(i);
            }
        }

        for i in 0..nums.len()/2 {
            ret.push(pos[i]);
            ret.push(neg[i]);
        }

        ret
    }
}

fn main() {
    let test = vec!(3, 1, -2, -5, 2, -4);
    let ret = Solution::rearrange_array(test);
    println!("Vec is: {:?}", ret);

    let test = vec!(-1,1);
    let ret = Solution::rearrange_array(test);
    println!("Vec is: {:?}", ret);
}
