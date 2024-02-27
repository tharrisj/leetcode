#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
 
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution{
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn depth(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            match root {
                None => (0, 0),
                Some(n) => {
                    let n = n.borrow();
                    let (l, x) = depth(&n.left);
                    let (r, y) = depth(&n.right);
                    let d = x.max(y).max(l+r);
                    if l > r {(l+1, d)} else {(r+1, d)}
                }
            }
        }
        depth(&root).1
    }
}

fn main() {
//    let mut test = TreeNode::new(1);
    let test = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let mut first = test.as_ref().unwrap().borrow_mut();
    first.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    first.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let mut second = first.left.as_ref().unwrap().borrow_mut();
    second.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    second.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    let ret = Solution::diameter_of_binary_tree(test.clone());
    assert_eq!(ret, 3);
    
    println!("All tests Passed!");
}