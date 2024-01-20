use std::collections::HashMap;

fn two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    let mut comp_map: HashMap<i32, usize> = HashMap::new();
    for (pos, &ele) in nums.iter().enumerate() {
        let complement = target - ele;
        match comp_map.get(&complement) {
            Some(&comp_pos) => return vec!(comp_pos as i32, pos as i32),
            None => {
                comp_map.insert(ele, pos);
            },
        }
    }
    vec!()
}

/*
fn two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    for (pos, &ele) in nums.iter().enumerate() {
        for (pos2, &ele2) in nums[pos+1..].iter().enumerate() {
            if ele + ele2 == target {
                return vec!(pos as i32, (pos+pos2+1) as i32);
            }
        }
    }
    vec!()
}
*/

fn main() {
    let nums: Vec<i32> = vec!(2, 7, 11, 15);
    let target: i32 = 9;
    let ret: Vec<i32> = two_sum(&nums, target);
    let ans: Vec<i32> = vec!(0, 1);
    assert_eq!(ret, ans);

    let nums: Vec<i32> = vec!(3, 2, 4);
    let target: i32 = 6;
    let ret: Vec<i32> = two_sum(&nums, target);
    let ans: Vec<i32> = vec!(1, 2);
    assert_eq!(ret, ans);

    let nums: Vec<i32> = vec!(3, 3);
    let target: i32 = 6;
    let ret: Vec<i32> = two_sum(&nums, target);
    let ans: Vec<i32> = vec!(0, 1);
    assert_eq!(ret, ans);
}
