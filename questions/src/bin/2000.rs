struct Solution;

impl Solution{
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let chars = word.chars();
        let mut acc: Vec<char> = vec![];
        let mut found = false;
        for c in chars{
            acc.push(c);
            if c == ch && !found {
                acc = acc.into_iter().rev().collect();
                found = true;
            }
        }

        acc.iter().collect()
    }
}

fn main() {
    let word = String::from("abcdefd");
    let ch = 'd';
    let ret = Solution::reverse_prefix(word, ch);
    assert_eq!(ret, String::from("dcbaefd"));

    println!("All tests passed!");
}