struct Solution{}

impl Solution{
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec!();
        let mut pow10 = low.ilog10();
        let mut lead_dig = low / 10_i32.pow(pow10);
        let mut cur_dig;
        let mut num = low;
        
        while pow10 < 9 && num < high {
            if lead_dig + pow10 as i32 > 9 {
                pow10 += 1;
                lead_dig = 1;
            } else {
                cur_dig = lead_dig;
                num = 0;
                for _ in 0..=pow10 {
                    num *= 10;
                    num += cur_dig;
                    cur_dig += 1;
                }
                if low <= num && num <= high {
                    res.push(num);
                }
                lead_dig += 1;
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
