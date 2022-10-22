extern crate core;


#[derive(Debug)]
pub struct ListNode {
    pub val: Box<u8>,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: u8) -> Self {
        ListNode {
            next: None,
            val: Box::new(val),
        }
    }

    #[inline]
    pub fn new_boxed(val: Box<u8>) -> Self {
        ListNode { next: None, val }
    }

    #[inline]
    pub fn with_next(self, next: ListNode) -> Self {
        let next = Some(Box::new(next));
        Self { next, ..self }
    }

    #[inline]
    pub fn with_next_boxed(self, next: Box<ListNode>) -> Self {
        let next = Some(next);
        Self { next, ..self }
    }

    pub fn add_other(self, other: ListNode) -> ListNode {
        let mut head: Option<Box<ListNode>> = None;
        let mut cur: &Option<Box<ListNode>> = &Some(Box::new(self));
        let mut other_cur: &Option<Box<ListNode>> = &Some(Box::new(other));
        let mut add_to_next_digit = 0u8;

        loop {
            match (cur, other_cur) {
                (Some(node), Some(other_node)) => {
                    let summary = *node.val + *other_node.val + add_to_next_digit;
                    let mut new_node = Box::new(ListNode::new(summary % 10));
                    add_to_next_digit = summary / 10;

                    new_node.next = head;
                    head = Some(new_node);

                    cur = &node.next;
                    other_cur = &other_node.next;
                }
                (Some(node), None) => {
                    let summary = *node.val + add_to_next_digit;
                    let mut new_node = Box::new(ListNode::new(summary % 10));
                    add_to_next_digit = summary / 10;

                    new_node.next = head;
                    head = Some(new_node);

                    cur = &node.next;
                }
                (None, Some(node)) => {
                    let summary = *node.val + add_to_next_digit;
                    let mut new_node = Box::new(ListNode::new(summary % 10));
                    add_to_next_digit = summary / 10;

                    new_node.next = head;
                    head = Some(new_node);

                    other_cur = &node.next;
                }
                (None, None) => {
                    if add_to_next_digit == 1 {
                        let mut new_node = Box::new(ListNode::new(1));
                        new_node.next = head;
                        head = Some(new_node);
                    }
                    break;
                }
            };
        };

        let mut reverse_head = None;
        while let Some(mut node) = head {
            head = node.next;
            node.next = reverse_head;
            reverse_head = Some(node);
        };

        *reverse_head.unwrap()
    }
}

fn main() {
    println!("It's as simple as two times two!");
    let a = ListNode::new(3u8).with_next(ListNode::new(2u8).with_next(ListNode::new(1u8)));

    // 456
    let b = ListNode::new(6u8).with_next(ListNode::new(5u8).with_next(ListNode::new(4u8)));

    let c = a.add_other(b);

    let digit0 = c;
    println!("{}", *digit0.val);
    assert!(digit0.next.is_some());

    let digit1 = digit0.next.unwrap();
    println!("{}", *digit1.val);

    let digit2 = digit1.next.unwrap();
    println!("{}", *digit2.val);
}

#[cfg(test)]
mod test_add {
    use super::*;

    #[test]
    fn zero() {
        let a = ListNode::new(0u8);
        let b = ListNode::new(0u8);

        let c = a.add_other(b);

        assert_eq!(0, *c.val);
        assert!(c.next.is_none());
    }

    #[test]
    fn simple_add() {
        // 123
        let a = ListNode::new(3u8).with_next(ListNode::new(2u8).with_next(ListNode::new(1u8)));

        // 456
        let b = ListNode::new(6u8).with_next(ListNode::new(5u8).with_next(ListNode::new(4u8)));

        let c = a.add_other(b);

        let digit0 = c;
        assert_eq!(9u8, *digit0.val);
        assert!(digit0.next.is_some());

        let digit1 = digit0.next.unwrap();
        assert_eq!(7u8, *digit1.val);
        assert!(digit1.next.is_some());

        let digit2 = digit1.next.unwrap();
        assert_eq!(5u8, *digit2.val);
        assert!(digit2.next.is_none());
    }

    #[test]
    fn nine_in_the_middle() {
        // 1983
        let a = ListNode::new(3u8).with_next(
            ListNode::new(8u8).with_next(ListNode::new(9u8).with_next(ListNode::new(1u8))),
        );

        // 127
        let b = ListNode::new(7u8).with_next(ListNode::new(2u8).with_next(ListNode::new(1u8)));

        let c = a.add_other(b);

        let digit0 = c;
        assert_eq!(0u8, *digit0.val);
        assert!(digit0.next.is_some());

        let digit1 = digit0.next.unwrap();
        assert_eq!(1u8, *digit1.val);
        assert!(digit1.next.is_some());

        let digit2 = digit1.next.unwrap();
        assert_eq!(1u8, *digit2.val);
        assert!(digit2.next.is_some());

        let digit3 = digit2.next.unwrap();
        assert_eq!(2u8, *digit3.val);
        assert!(digit3.next.is_none());
    }

    #[test]
    fn nine_in_the_middle_trans() {
        // 1983
        let a = ListNode::new(3u8).with_next(
            ListNode::new(8u8).with_next(ListNode::new(9u8).with_next(ListNode::new(1u8))),
        );

        // 127
        let b = ListNode::new(7u8).with_next(ListNode::new(2u8).with_next(ListNode::new(1u8)));

        let c = b.add_other(a);

        let digit0 = c;
        assert_eq!(0u8, *digit0.val);
        assert!(digit0.next.is_some());

        let digit1 = digit0.next.unwrap();
        assert_eq!(1u8, *digit1.val);
        assert!(digit1.next.is_some());

        let digit2 = digit1.next.unwrap();
        assert_eq!(1u8, *digit2.val);
        assert!(digit2.next.is_some());

        let digit3 = digit2.next.unwrap();
        assert_eq!(2u8, *digit3.val);
        assert!(digit3.next.is_none());
    }

    #[test]
    fn nines() {
        // 99 999
        let a = ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(
            ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(ListNode::new(9u8))),
        ));

        // 9 999
        let b = ListNode::new(9u8).with_next(
            ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(ListNode::new(9u8))),
        );

        let c = a.add_other(b);

        let digit0 = c;
        assert_eq!(8u8, *digit0.val);
        assert!(digit0.next.is_some());

        let digit1 = digit0.next.unwrap();
        assert_eq!(9u8, *digit1.val);
        assert!(digit1.next.is_some());

        let digit2 = digit1.next.unwrap();
        assert_eq!(9u8, *digit2.val);
        assert!(digit2.next.is_some());

        let digit3 = digit2.next.unwrap();
        assert_eq!(9u8, *digit3.val);
        assert!(digit3.next.is_some());

        let digit4 = digit3.next.unwrap();
        assert_eq!(0u8, *digit4.val);
        assert!(digit4.next.is_some());

        let digit5 = digit4.next.unwrap();
        assert_eq!(1u8, *digit5.val);
        assert!(digit5.next.is_none());
    }

    #[test]
    fn nines_trans() {
        // 99 999
        let a = ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(
            ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(ListNode::new(9u8))),
        ));

        // 9 999
        let b = ListNode::new(9u8).with_next(
            ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(ListNode::new(9u8))),
        );

        let c = b.add_other(a);

        let digit0 = c;
        assert_eq!(8u8, *digit0.val);
        assert!(digit0.next.is_some());

        let digit1 = digit0.next.unwrap();
        assert_eq!(9u8, *digit1.val);
        assert!(digit1.next.is_some());

        let digit2 = digit1.next.unwrap();
        assert_eq!(9u8, *digit2.val);
        assert!(digit2.next.is_some());

        let digit3 = digit2.next.unwrap();
        assert_eq!(9u8, *digit3.val);
        assert!(digit3.next.is_some());

        let digit4 = digit3.next.unwrap();
        assert_eq!(0u8, *digit4.val);
        assert!(digit4.next.is_some());

        let digit5 = digit4.next.unwrap();
        assert_eq!(1u8, *digit5.val);
        assert!(digit5.next.is_none());
    }


    #[test]
    fn nines_plus_one() {
        // 99 999
        let a = ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(
            ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(ListNode::new(9u8))),
        ));

        // 1
        let b = ListNode::new(1);

        let c = b.add_other(a);

        let digit0 = c;
        assert_eq!(0u8, *digit0.val);
        assert!(digit0.next.is_some());

        let digit1 = digit0.next.unwrap();
        assert_eq!(0u8, *digit1.val);
        assert!(digit1.next.is_some());

        let digit2 = digit1.next.unwrap();
        assert_eq!(0u8, *digit2.val);
        assert!(digit2.next.is_some());

        let digit3 = digit2.next.unwrap();
        assert_eq!(0u8, *digit3.val);
        assert!(digit3.next.is_some());

        let digit4 = digit3.next.unwrap();
        assert_eq!(0u8, *digit4.val);
        assert!(digit4.next.is_some());

        let digit5 = digit4.next.unwrap();
        assert_eq!(1u8, *digit5.val);
        assert!(digit5.next.is_none());
    }
}
