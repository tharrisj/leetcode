// Definition for a binary tree node.
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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(n: Option<Rc<RefCell<TreeNode>>>, d: i32, ret: &mut (i32, i32)) {
            if let Some(bx) = n {
                let node = bx.borrow();
                if d > ret.0 {
                    *ret = (d, node.val)
                }
                dfs(node.left.clone(), d+1, ret);
                dfs(node.right.clone(), d+1, ret);
            }
        }
        let mut r = (-1, 0);
        dfs(root, 0, &mut r);
        r.1
    }
}

fn main() {
    let test = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let mut first = test.as_ref().unwrap().borrow_mut();
    first.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    first.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let mut second = first.left.as_ref().unwrap().borrow_mut();
    second.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    second.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    let ret = Solution::find_bottom_left_value(test.clone());
    assert_eq!(ret, 4);

    println!("Tests Complete!")
}