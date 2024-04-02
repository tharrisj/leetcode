struct Solution;

impl Solution{
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut mem: std::collections::HashMap<char, char> = std::collections::HashMap::new();
        let s1: Vec<char> = s.chars().collect();
        let s2: Vec<char> = t.chars().collect();

        for (pos, ch) in s1.iter().enumerate() {
            if let Some(mirror) = mem.get(ch) {
                if *mirror != s2[pos] {
                    return false;
                }
            } else {
                let val_exist = mem.values().any(|&x| x == s2[pos]);
                if val_exist {
                    return false;
                } else {
                    mem.insert(*ch, s2[pos]);
                }
            }
        }

        true
    }
}

fn main() {
    let s = String::from("egg");
    let t = String::from("add");
    let ret = Solution::is_isomorphic(s, t);
    assert_eq!(ret, true);

    let s = String::from("foo");
    let t = String::from("bar");
    let ret = Solution::is_isomorphic(s, t);
    assert_eq!(ret, false);

    let s = String::from("paper");
    let t = String::from("title");
    let ret = Solution::is_isomorphic(s, t);
    assert_eq!(ret, true);

    println!("All tests passed!");
}