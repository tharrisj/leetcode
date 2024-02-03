struct Solution{}

impl Solution{
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let mut max_sum_vec = vec!(0; arr.len()+1);
        for i in 0..arr.len() {
            let mut cur_max = 0;
            for j in (0..=i).rev().take(k as usize) {
                cur_max = cur_max.max(arr[j]);
                max_sum_vec[i + 1] = max_sum_vec[i + 1].max((i - j + 1) as i32 * cur_max + max_sum_vec[j]);
            }
        }
        max_sum_vec[arr.len()]
    }
}

fn main() {
    let test = vec!(1,15,7,9,2,5,10);
    let lim = 3;
    let ret = Solution::max_sum_after_partitioning(test, lim);
    assert_eq!(ret, 84);

    let test = vec!(1,4,1,5,7,3,6,1,9,9,3);
    let lim = 4;
    let ret = Solution::max_sum_after_partitioning(test, lim);
    assert_eq!(ret, 83);

    let test = vec!(1);
    let lim = 1;
    let ret = Solution::max_sum_after_partitioning(test, lim);
    assert_eq!(ret, 1);

    println!("All tests passed!");
}
