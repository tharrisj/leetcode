struct Solution{}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut zz_ret = String::new();

        zz_ret
    }
}

fn main() {
    let input = String::from("PAYPALISHIRING");
    let rows = 3;
    let ret = Solution::convert(input.clone(), rows);
    println!("Return from {input}: {ret} ");
    assert_eq!(ret, String::from("PAHNAPLSIIGYIR"));

    let input: String = String::from("A");
    let rows: i32 = 1;
    let ret: String = Solution::convert(input.clone(), rows);
    println!("Return from {input}: {ret} ");
    assert_eq!(ret, String::from("A"));
}
