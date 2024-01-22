#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode{
            next: None,
            val,
        }
    }
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sum_list= ListNode::new(0);
        let mut cur_sum_list =&mut sum_list;
        let mut cur1 = &l1;
        let mut cur2 = &l2;
        let mut carry = 0;
        loop {
            if *cur1 == None && *cur2 == None && carry == 0 { break; }
            let val1 = match cur1 {
                Some(ref ln) => ln.val,
                None => 0,
            };
            let val2 = match cur2 {
                Some(ref ln) => ln.val,
                None => 0,
            };
            let mut sum_val = val1+val2+carry;
            carry = sum_val / 10;
            sum_val = sum_val % 10;
    //        sum_list = Some(Box::new(ListNode{ val: sum_val, next: sum_list }));
            cur_sum_list.next = Some(Box::new(ListNode{val: sum_val, next: None}));
            cur_sum_list = cur_sum_list.next.as_mut().unwrap();
            
            cur1 = match cur1 {
                Some(ref ln) => &ln.next,
                None => &None,
            };

            cur2 = match cur2 {
                Some(ref ln) => &ln.next,
                None => &None,
            }
        }

        sum_list.next
}

fn create_from_vec(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut local = nums;
    if let Some(first_ele) = local.pop() {
        let mut ret_list = ListNode::new(first_ele);
        for &digit in local.iter().rev() {
            ret_list = ListNode{ val: digit, next: Some(Box::new(ret_list))};
        }
        return Some(Box::new(ret_list));
    }

    Some(Box::new(ListNode::new(0)))
}

fn main() {
    let first = create_from_vec(vec!(2, 4, 3));
    let second = create_from_vec(vec!(5, 6, 4));
    let ret = add_two_numbers(first, second);
    println!("Let return be: {:?}", ret);

    let first = Some(Box::new(ListNode::new(0)));
    let second = Some(Box::new(ListNode::new(0)));
    let ret = add_two_numbers(first, second);
    println!("let return be: {:?}", ret);

    let first = create_from_vec(vec!(9,9,9,9,9,9,9));
    let second = create_from_vec(vec!(9,9,9,9));
    let ret = add_two_numbers(first, second);
    println!("let return be: {:?}", ret);

    /*
    let first = Some(Box::new(ListNode::new(9)));
    let sec_vec: Vec<i32> = vec!(1,9,9,9,9,9,9,9,9,9);
    let second: Option<Box<ListNode>> = create_from_vec(sec_vec);
    let ret = add_two_numbers(first, second);
    println!("let return be: {:?}", ret);
     */
}
