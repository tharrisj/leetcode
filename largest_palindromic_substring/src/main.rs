struct Solution{}

impl Solution {
    pub fn largest_palindrome(s: String) -> String {
        let largest_str = String::new();
    
        largest_str
    }

}


fn main() {
    let test_string = String::from("babad");
    let long_pal = Solution::largest_palindrome(test_string.clone());
    println!("Longest palindrome for {} is {}", test_string, long_pal);
    assert_eq!(long_pal, String::from("bab"));

    let test_string = String::from("cbbd");
    let long_pal = Solution::largest_palindrome(test_string.clone());
    println!("Longest palindrome for {} is {}", test_string, long_pal);
    assert_eq!(long_pal, String::from("bb"));
}
