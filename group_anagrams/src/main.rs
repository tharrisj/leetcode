struct Solution{}

use std::collections::HashMap;

impl Solution{
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut ret: Vec<Vec<String>> = vec!();
        let mut anagrams: HashMap<String, Vec<String>> = HashMap::new();

        for ele in strs {
            let mut char_vec: Vec<char> = ele.chars().collect();
            char_vec.sort_unstable();
            let string_key: String = char_vec.into_iter().collect();

            let map_ele = anagrams.entry(string_key).or_insert(vec!());
            map_ele.push(ele);
        }

        for (_, val) in anagrams {
            ret.push(val);
        }

        ret
    }
}

fn main() {
    let test: Vec<String> = vec!("eat".to_owned(), "tea".to_owned(), "tan".to_owned(), "ate".to_owned(), "nat".to_owned(), "bat".to_owned());
    let ret = Solution::group_anagrams(test);
    println!("Return is: {:?}", ret);
    
}
