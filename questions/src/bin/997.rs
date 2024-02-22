struct Solution;

impl Solution{
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize + 1;
        let mut info = vec![(0, 0); n];
        
        trust.into_iter().for_each(|x| {
            info[x[0] as usize].1 += 1;
            info[x[1] as usize].0 += 1;
        });

        for i in 1..n {
            if info[i].0 == n-2 && info[i].1 == 0 {
                return i as i32;
            }
        }

        -1
    }
}

fn main() {
    let n = 2;
    let trust: Vec<Vec<i32>> = vec![vec![1,2]];
    let ret = Solution::find_judge(n, trust);
    assert_eq!(ret, 2);

    let n = 3;
    let trust: Vec<Vec<i32>> = vec![vec![1,3],vec![2,3]];
    let ret = Solution::find_judge(n, trust);
    assert_eq!(ret, 3);

    let n = 3;
    let trust: Vec<Vec<i32>> = vec![vec![1,3],vec![2,3],vec![3,1]];
    let ret = Solution::find_judge(n, trust);
    assert_eq!(ret, -1);

    println!("All Tests Passed!");
}