use std::collections::VecDeque;

struct Solution;

impl Solution{
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        let mut deck = deck;
        deck.sort_unstable();
        let mut deck = deck.into_iter();

        let mut queue = VecDeque::from_iter(0..deck.len());
        let mut result = vec![0; deck.len()];

        while let Some(i) = queue.pop_front() {
            result[i] = deck.next().unwrap();
            queue.rotate_left(1.min(queue.len()));
        }

        result
    }
}

fn main() {
    let deck = vec![17,13,11,2,3,5,7];
    let ret = Solution::deck_revealed_increasing(deck);
    assert_eq!(ret, vec![2,13,3,11,5,17,7]);

    println!("All tests passed!");
}