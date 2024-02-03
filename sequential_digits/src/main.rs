struct Solution{}

impl Solution{
    fn is_sequential(num: i32) -> bool {
        let mut test = num;
        let mut prev = test % 10;
        test /= 10;
        while test > 0 {
            let cur = test % 10;
            if prev != cur+1 {
                return false
            }
            prev = cur;
            test /= 10;
        }

        true
    }

    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec!();
                
        for i in low..=high {
            if Solution::is_sequential(i) {
                res.push(i);
            }
        }

        res
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
