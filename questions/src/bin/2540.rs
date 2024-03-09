struct Solution;

impl Solution{
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, 0);

        loop {
            match (nums1.get(i), nums2.get(j)) {
                (Some(&x), Some(&y)) => {
                    if x == y {
                        return x;
                    } else if x > y {
                        j += 1;
                    } else {
                        i += 1;
                    }
                },
                _ => { break; }
            }
        }

        -1
    }
}

fn main() {
    let nums1 = vec![1,2,3];
    let nums2 = vec![2,4];
    let ret = Solution::get_common(nums1, nums2);
    assert_eq![ret, 2];

    let nums1 = vec![1,2,3,6];
    let nums2 = vec![2,3,4,5];
    let ret = Solution::get_common(nums1, nums2);
    assert_eq![ret, 2];

    println!("All Tests Passed!");
}