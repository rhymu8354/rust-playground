// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

struct Solution {}

impl Solution {
    fn reverse(
        l: &Option<Box<ListNode>>,
        rl: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match l {
            Some(node) => Self::reverse(
                &node.next,
                Some(Box::new(ListNode{val: node.val, next: rl}))
            ),
            None => rl,
        }
    }
    
    fn add_two_numbers_reversed(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
        first: bool,
    ) -> Option<Box<ListNode>> {
        let (val1, next1) = match l1 {
            Some(l1n) => (l1n.val, l1n.next),
            None => (0, None),
        };
        let (val2, next2) = match l2 {
            Some(l2n) => (l2n.val, l2n.next),
            None => (0, None),
        };
        let sum = val1 + val2 + carry;
        if next1.is_none() && next2.is_none() && sum == 0 && !first {
            None
        } else {
            Some(Box::new(ListNode{val: sum % 10, next: Self::add_two_numbers_reversed(next1, next2, sum / 10, false)}))
        }
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let l1r = Self::reverse(&l1, None);
        let l2r = Self::reverse(&l2, None);
        let l12r = Self::add_two_numbers_reversed(l1r, l2r, 0, true);
        Self::reverse(&l12r, None)
    }
}

fn print_list(node: &ListNode) {
    print!("{}", node.val);
    if node.next.is_some() {
        print!(" -> ");
        print_list(node.next.as_ref().unwrap());
    } else {
        println!();
    }
}

fn main() {
    let l1 = Some(Box::new(ListNode{val: 7, next: Some(Box::new(ListNode{val: 2, next: Some(Box::new(ListNode{val: 4, next: Some(Box::new(ListNode{val: 3, next: None}))}))}))}));
    let l2 = Some(Box::new(ListNode{val: 5, next: Some(Box::new(ListNode{val: 6, next: Some(Box::new(ListNode{val: 4, next: None}))}))}));
    let l3 = Solution::add_two_numbers(l1, l2);
    print_list(l3.as_ref().unwrap());
}
