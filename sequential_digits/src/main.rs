struct Solution{}

impl Solution{
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        
    }
}

fn main() {
    let (low, high) = (100, 300);
    let res = Solution::sequential_digits(low, high);
    assert_eq!(res, vec!(123,234));

    let (low, high) = (1000, 13000);
    let res = Solution::sequential_digits(low, high);
    assert_eq!(res, vec!(1234,2345,3456,4567,5678,6789,12345));

    println!("Tests Passed!");
}
