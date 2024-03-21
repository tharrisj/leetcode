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
}

struct Solution;

impl Solution{
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ret_head: Option<Box<ListNode>> = None;
        let mut node = &head;

        loop {
            match node {
                Some(ref node_val) => {
                    ret_head = Some(Box::new(ListNode{val: node_val.val, next: ret_head}));
                    node = &node_val.next;
                },
                None => {
                    break;
                }
            }
        }

        ret_head
    }
}

fn main() {
    let mut head = Some(Box::new(ListNode::new(5)));
    head = Some(Box::new(ListNode{val: 4, next: head}));
    head = Some(Box::new(ListNode{val: 3, next: head}));
    head = Some(Box::new(ListNode{val: 2, next: head}));
    head = Some(Box::new(ListNode{val: 1, next: head}));
    let ret = Solution::reverse_list(head);
    println!("Ret is: {:?}", ret);

    println!("All tests passed!");
}