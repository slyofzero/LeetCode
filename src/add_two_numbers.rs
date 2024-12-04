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
}

fn make_list(array: Vec<i32>) -> Option<Box<ListNode>> {
    let mut output_list: Option<Box<ListNode>> = None;

    for val in array.into_iter().rev() {
        let mut new_node = ListNode::new(val);
        new_node.next = output_list;
        output_list = Some(Box::new(new_node));
    }

    return output_list;
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current_l1 = l1.as_ref();
    let mut current_l2 = l2.as_ref();
    let mut is_carried_over = false;
    let mut output_vec  = Vec::new();

    while current_l1.is_some() || current_l2.is_some() {
        let node_1_val = current_l1.map_or(0, |x| x.val);
        let node_2_val = current_l2.map_or(0, |x| x.val);
        let mut sum = node_1_val + node_2_val + (is_carried_over as i32);

        if sum > 9 {
            is_carried_over = true;
            sum %= 10;
        } else { is_carried_over = false; }

        current_l1 = current_l1.map_or(None, |x| x.next.as_ref());
        current_l2 = current_l2.map_or(None, |x| x.next.as_ref());

        output_vec.push(sum);
    }

    if is_carried_over { output_vec.push(1); }
    
    return make_list(output_vec);
}

fn main() {
    let l1 = make_list(vec![2,4,3]);
    let l2 = make_list(vec![5,6,4]);
    
    // println!("{:?}", l1);
    // println!("{:?}", l2);

    let output = add_two_numbers(l1, l2);
    println!("{:?}", output);
}