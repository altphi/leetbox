#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[must_use]
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

#[must_use]
pub fn list_from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;

    for value in values.into_iter().rev() {
        let mut node = Box::new(ListNode::new(value));
        node.next = head;
        head = Some(node);
    }

    head
}

#[must_use]
pub fn vec_from_list(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = Vec::new();

    while let Some(node) = head {
        out.push(node.val);
        head = node.next;
    }

    out
}
