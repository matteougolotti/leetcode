
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

fn list_to_number_rec(acc: i64, l: Option<Box<ListNode>>) -> i64 {
    return match l {
        None => acc,
        Some(l) => acc + 10 * list_to_number_rec(l.val as i64, l.next),
    };
}

fn list_to_number(l: Option<Box<ListNode>>) -> i64 {
    return match l {
        None => 0,
        Some(l) => list_to_number_rec(l.val as i64, l.next),
    };
}

fn number_to_list_rec(n: i64) -> Option<Box<ListNode>> {
    return match n {
        0 => None,
        n => Some(Box::new(ListNode{val: (n % 10) as i32, next: number_to_list_rec(n / 10)})),
    };
}

fn number_to_list(n: i64) -> Option<Box<ListNode>> {
    return match n {
        0 => Some(Box::new(ListNode{val: 0, next: None})),
        n => number_to_list_rec(n),
    };
}

pub fn add_two_numbers_rec(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
    return match (l1, l2, carry) {
        (None, None, 0) => None,
        (None, None, n) => Some(Box::new(ListNode{val: n % 10, next: add_two_numbers_rec(&None, &None, n / 10)})),
        (Some(l1), None, n) => Some(Box::new(ListNode{val: (l1.val + n) % 10, next: add_two_numbers_rec(&l1.next, &None, (l1.val + n) / 10)})),  
        (None, Some(l2), n) => Some(Box::new(ListNode{val: (l2.val + n) % 10, next: add_two_numbers_rec(&None, &l2.next, (l2.val + n) / 10)})),
        (Some(l1), Some(l2), n) => Some(Box::new(ListNode{val: (l1.val + l2.val + n) % 10, next: add_two_numbers_rec(&l1.next, &l2.next, (l1.val + l2.val + n) / 10)})),
    };
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    return add_two_numbers_rec(&l1, &l2, 0);
}

fn main() {
    let l1 = number_to_list(342);
    let l2 = number_to_list_rec(465);
    let l3 = add_two_numbers(l1, l2);
    let result = list_to_number(l3);
    println!("Result => {:?}", result);

    let l1 = number_to_list(9);
    let l2 = number_to_list(9999999991);
    let l3 = add_two_numbers(l1, l2);
    let result = list_to_number(l3);
    println!("Result => {:?}", result);
}
