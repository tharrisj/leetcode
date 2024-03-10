struct Solution;

impl Solution{
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums_exist = [0; 1001];
        let mut ret: Vec<i32> = vec![];

        for x in nums1 {
            nums_exist[x as usize] += 1;
        }

        for x in nums2 {
            if nums_exist[x as usize] > 0 {
                ret.push(x);
                nums_exist[x as usize] = 0;
            }
        }

        ret
    }
}

fn main() {
    let nums1 = vec![1,2,2,1];
    let nums2 = vec![2,2];
    let ret = Solution::intersection(nums1, nums2);
    assert_eq!(ret, vec![2]);

    let nums1 = vec![4,9,5];
    let nums2 = vec![9,4,9,8,4];
    let mut ret = Solution::intersection(nums1, nums2);
    ret.sort_unstable();
    assert_eq!(ret, vec![4,9]);

    println!("All Tests Passed!");
}