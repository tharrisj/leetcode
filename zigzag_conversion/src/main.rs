struct Solution{}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 { return s; }
        let mut zz_ret = String::new();
        let mut filler: Vec<Vec<char>> = vec!(vec!(' ';s.len());num_rows as usize);

        let mut rev: bool = false;
        let mut v_ind: i32 = 0;
        let mut h_ind: i32 = 0;
        for ch in s.chars() {
            filler[v_ind as usize][h_ind as usize] = ch;

            if v_ind == 0 || v_ind == num_rows-1 { rev = !rev; }

            if rev {
                v_ind += 1;
            } else {
                v_ind -= 1;
                h_ind += 1;
            }
        }

        for c_vec in filler {
            let tmp: String = c_vec
                .iter()
                .filter(|&&ch| ch != ' ')
                .collect();
            zz_ret.push_str(&tmp);
        }

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
