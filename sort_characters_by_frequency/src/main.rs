struct Solution{}

impl Solution{
    pub fn frequency_sort(s: String) -> String {
        let mut ret = String::new();
        let mut chars_count: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
        for ch in s.chars() {
            let entry = chars_count.entry(ch).or_insert(0);
            *entry += 1;
        }
        let mut chars_vec: Vec<(char, i32)> = vec!();
        for (char, count) in chars_count {
            chars_vec.push((char, count));
        }
        chars_vec.sort_by_key(|(_, count)| *count);
        
        for (char, count) in chars_vec.into_iter().rev(){
            let append = std::iter::repeat(char).take(count as usize).collect::<String>();
            ret.push_str(&append);
        }
        ret
    }
}

fn main() {
    let test = "tree".to_owned();
    let ret = Solution::frequency_sort(test);
    println!("ret is: {ret}");
}
