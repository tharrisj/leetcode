struct Solution;

impl Solution{
    pub fn tribonacci(n: i32) -> i32 {
        let (mut n1, mut n2, mut n3) = (0, 1, 1);
        if n == 0 { return 0; }
        if n < 3 { return 1; }
        for _ in 3..n {
            let trib = n1 + n2 + n3;
            n1 = n2;
            n2 = n3;
            n3 = trib;
        }
        n1+n2+n3
    }
}

fn main() {
    let n = 4;
    let ret = Solution::tribonacci(n);
    assert_eq!(ret, 4);

    let n = 25;
    let ret = Solution::tribonacci(n);
    assert_eq!(ret, 1389537);

    println!("All tests passed!");
}