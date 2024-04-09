struct Solution;

impl Solution{
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;
        let k = k as usize;

        for (pos, &val) in tickets.iter().enumerate() {
            if pos < k {
                ret += val.min(tickets[k]);
            }
            if pos > k {
                ret += val.min(tickets[k] -1);
            }
            if pos == k {
                ret += tickets[k];
            }
        }

        ret
    }
}

fn main() {
    let tickets = vec![2,3,2];
    let k = 2;
    let ret = Solution::time_required_to_buy(tickets, k);
    assert_eq!(ret, 6);

    let tickets = vec![5,1,1,1];
    let k = 0;
    let ret = Solution::time_required_to_buy(tickets, k);
    assert_eq!(ret, 8);

    println!("All tests passed!");
}