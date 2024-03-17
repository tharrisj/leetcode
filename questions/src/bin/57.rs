struct Solution;

impl Solution{
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        let mut new_interval = new_interval;
        let start = intervals.partition_point(|v| v[1] < new_interval[0]);
        let end = intervals.partition_point(|v| v[0] <= new_interval[1]);

        let slice: Vec<i32> = intervals.drain(start..end).flatten().collect();

        if let [first, .. , last] = &slice[..] {
            new_interval[0] = new_interval[0].min(*first);
            new_interval[1] = new_interval[1].max(*last);
        }

        intervals.insert(start, new_interval);

        intervals
    }
}

fn main() {
    let intervals: Vec<Vec<i32>> = vec![vec![1,3], vec![6,9]];
    let new_interval: Vec<i32> = vec![2,5];
    let ret = Solution::insert(intervals, new_interval);
    assert_eq!(ret, vec![vec![1,5], vec![6,9]]);

    println!("All tests passed!");
}