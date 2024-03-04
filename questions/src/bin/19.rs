// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    fn prepend(self, num: i32) -> Self {
        let mut head = ListNode::new(num);
        head.next = Some(Box::new(self));
        head
    }

    fn new_list(nums: Vec<i32>) -> Self {
        let mut head = ListNode::new(nums[0]);
        for &i in nums.iter().rev() {
            head = head.prepend(i);
        }
        head
    }
}

struct Solution;

impl Solution{
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let cnt = std::iter::successors(head.as_ref(), |last| last.next.as_ref()).count();
        let mut dummy = Some(Box::new(ListNode{val: 0, next: head}));
        let mut prev = (0..cnt - n as usize).fold(dummy.as_mut(), |curr, _| curr.unwrap().next.as_mut() );
        prev.unwrap().next = prev.as_mut().unwrap().next.as_mut().unwrap().next.take();
        dummy.unwrap().next
    }
}

fn main() {
    let test = Some(Box::new(ListNode::new_list(vec![1,2,3,4,5])));
    let ret = Solution::remove_nth_from_end(test, 2);
    assert_eq!(ret, Some(Box::new(ListNode::new_list(vec![1,2,3,5]))));

    println!("All tests passed!");
}