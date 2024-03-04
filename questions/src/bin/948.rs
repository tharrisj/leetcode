struct Solution;

impl Solution{
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        let mut tokens = tokens;
        tokens.sort_unstable();
        let mut power = power;
        let mut score = 0;
        let (mut left, mut right) = (0, tokens.len() as i32 - 1);
        let mut max_score = 0;

        while left <= right {
            if tokens[left as usize] <= power {
                power -= tokens[left as usize];
                score += 1;
                left += 1;
            } else if score > 0 {
                power += tokens[right as usize];
                score -= 1;
                right -= 1;
            } else {
                break;
            }
            max_score = max_score.max(score);
        }

        max_score
    }
}

fn main() {
    let tokens = vec![100];
    let power = 50;
    let ret = Solution::bag_of_tokens_score(tokens, power);
    assert_eq!(ret, 0);

    let tokens = vec![200, 100];
    let power = 150;
    let ret = Solution::bag_of_tokens_score(tokens, power);
    assert_eq!(ret, 1);

    let tokens = vec![100, 200, 300, 400];
    let power = 200;
    let ret = Solution::bag_of_tokens_score(tokens, power);
    assert_eq!(ret, 2);

    println!("All Tests Passed!");
}