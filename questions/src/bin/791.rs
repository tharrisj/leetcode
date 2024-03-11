struct Solution;

impl Solution{
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut ret = String::new();
        let mut arr: [i32; 26] = [0; 26];

        for byte in s.bytes() {
            arr[(byte - b'a') as usize] += 1;
        }

        for byte in order.bytes() {
            let repeat = arr[(byte - b'a') as usize];
            let char = byte as char;
            ret.push_str(&std::iter::repeat(char).take(repeat as usize).collect::<String>());
            arr[(byte - b'a') as usize] = 0;
        }

        for (byte, &repeat) in arr.iter().enumerate() {
            if repeat > 0 {
                ret.push_str(&std::iter::repeat((byte as u8 + b'a') as char).take(repeat as usize).collect::<String>())
            }
        }

        ret
    }
}

fn main() {
    let order = String::from("cba");
    let s = String::from("abcd");
    let ret = Solution::custom_sort_string(order, s);
    assert_eq!(ret, String::from("cbad"));

    let order = String::from("bcafg");
    let s = String::from("abcd");
    let ret = Solution::custom_sort_string(order, s);
    assert_eq!(ret, String::from("bcad"));

    println!("All Tests Passed!");
}