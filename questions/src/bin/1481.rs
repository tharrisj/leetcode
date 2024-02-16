struct Solution;

impl Solution{
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut k = k;
        let mut count_map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for i in arr {
            let entry = count_map.entry(i).or_insert(0);
            *entry += 1;
        }
        let mut entries: Vec<(i32, i32)> = vec![];
        for (num, count) in count_map {
            entries.push((num, count));
        }
        entries.sort_by_key(|(_, x)| *x);

        while entries.len() > 0 {
            let (_, cnt) = entries[0];
            if cnt > k {
                return entries.len() as i32;
            } else {
                k -= cnt;
                entries.remove(0);
            }
        }
        return 0;
    }
}

fn main() {
    let test = vec![5,5,4];
    let ret = Solution::find_least_num_of_unique_ints(test, 1);
    assert_eq!(ret, 1);

    let test = vec![4,3,1,1,3,3,2];
    let ret = Solution::find_least_num_of_unique_ints(test, 3);
    assert_eq!(ret, 2);

    let test = vec![1];
    let ret = Solution::find_least_num_of_unique_ints(test, 1);
    assert_eq!(ret, 0);

    println!("All tests passed!");
}