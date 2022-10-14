#[derive(Debug, PartialEq)]
pub struct Telemetry {
    pub point: Point,
    pub time_stamp: u64,
}

impl Telemetry {
    pub fn new(point: Point, time_stamp: u64) -> Self {
        Self { point, time_stamp }
    }
}

#[derive(Debug, PartialEq)]
pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
}

impl Point {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }
}

#[derive(Debug, PartialEq)]
pub struct ListNode {
    pub val: Telemetry,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: Telemetry) -> Self {
        ListNode { next: None, val }
    }

    pub fn with_next(self, next: ListNode) -> Self {
        let next = Some(Box::new(next));
        Self { next, ..self }
    }

    // method should return part of path between two time points
    pub fn get_part(self, from: u64, to: u64) -> Option<ListNode> {
        let mut cur: Option<Box<ListNode>> = Some(Box::new(self));

        while let Some(node) = cur {
            if node.val.time_stamp > to {
                cur = node.next;
            } else {
                cur = Some(node);
                break;
            }
        }

        let mut head: Option<Box<ListNode>> = None;
        while let Some(mut node) = cur {
            if node.val.time_stamp >= from {
                cur = node.next;
                node.next = head;
                head = Some(node);
            } else {
                break;
            }
        }

        let mut reverse_head = None;
        while let Some(mut node) = head {
            head = node.next;
            node.next = reverse_head;
            reverse_head = Some(node);
        };

        if let Some(node) = reverse_head {
            Some(*node)
        } else {
            None
        }
    }

}

fn main() {
    println!("Rust is fun, they said.");
    let first = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 100));
    let second = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 200));
    let third = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 300));
    let fourth = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 400));
    let fifth = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 500));
    let sixth = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 600));
    let seventh = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 700));
    let eighth = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 800));
    let ninth = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 900));
    let tenth = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 1000));

    let second = second.with_next(first);
    let third = third.with_next(second);
    let fourth = fourth.with_next(third);
    let fifth = fifth.with_next(fourth);
    let sixth = sixth.with_next(fifth);
    let seventh = seventh.with_next(sixth);
    let eighth = eighth.with_next(seventh);
    let ninth = ninth.with_next(eighth);

    let eleven = tenth.with_next(ninth);

    // println!("{}", eleven.len());
    let sublist = eleven.get_part(100, 200);
}


#[cfg(test)]
mod test {
    use super::*;

    fn stub() -> ListNode {
        let first = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 100));
        let second = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 200));
        let third = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 300));
        let fourth = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 400));
        let fifth = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 500));
        let sixth = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 600));
        let seventh = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 700));
        let eighth = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 800));
        let ninth = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 900));
        let tenth = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 1000));

        let second = second.with_next(first);
        let third = third.with_next(second);
        let fourth = fourth.with_next(third);
        let fifth = fifth.with_next(fourth);
        let sixth = sixth.with_next(fifth);
        let seventh = seventh.with_next(sixth);
        let eighth = eighth.with_next(seventh);
        let ninth = ninth.with_next(eighth);

        tenth.with_next(ninth)
    }

    #[test]
    fn not_found() {
        let list = stub();
        let sublist = list.get_part(50, 70);
        assert!(sublist.is_none());
    }

    #[test]
    fn exact_all() {
        let list = stub();
        let sublist = list.get_part(100, 1000);
        assert!(sublist.is_some());

        let sublist = sublist.unwrap();
        assert_eq!(sublist.val.time_stamp, 1000);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 900);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 800);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 700);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 600);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 500);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 400);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 300);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 200);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 100);
    }

    #[test]
    fn gte_lte_all() {
        let list = stub();
        let sublist = list.get_part(0, 1500);
        assert!(sublist.is_some());

        let sublist = sublist.unwrap();
        assert_eq!(sublist.val.time_stamp, 1000);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 900);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 800);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 700);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 600);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 500);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 400);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 300);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 200);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 100);
    }

    #[test]
    fn exact_sublist() {
        let list = stub();
        let sublist = list.get_part(400, 700);
        assert!(sublist.is_some());

        let sublist = sublist.unwrap();
        assert_eq!(sublist.val.time_stamp, 700);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 600);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 500);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 400);
        assert!(sublist.next.is_none());
    }

    #[test]
    fn gte_lte_sublist() {
        let list = stub();
        let sublist = list.get_part(350, 750);
        assert!(sublist.is_some());

        let sublist = sublist.unwrap();
        assert_eq!(sublist.val.time_stamp, 700);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 600);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 500);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 400);
        assert!(sublist.next.is_none());
    }
}