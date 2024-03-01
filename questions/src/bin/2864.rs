struct Solution;

impl Solution{
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut ret = String::new();
        s.chars().for_each(|ch| if ch == '1' {ret.push('1')});
        ret.pop();
        for _ in ret.len()..s.len()-1 {
            ret.push('0');
        }
        ret.push('1');

        ret.to_owned()
    }
}

fn main() {
    let test = String::from("010");
    let ret = Solution::maximum_odd_binary_number(test);
    assert_eq!(ret, String::from("001"));

    let test = String::from("0101");
    let ret = Solution::maximum_odd_binary_number(test);
    assert_eq!(ret, String::from("1001"));

    println!("All Tests Passed!");
}