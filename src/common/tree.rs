use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub type Node = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Node,
    pub right: Node,
}

impl TreeNode {
    #[inline]
    #[must_use]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

#[must_use]
pub fn tree_from_level_order(values: &[Option<i32>]) -> Node {
    if values.is_empty() || values[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());

    let mut i = 1;

    while i < values.len() {
        let current = queue.pop_front().unwrap();

        if i < values.len() {
            if let Some(v) = values[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(v)));
                current.borrow_mut().left = Some(left.clone());
                queue.push_back(left);
            }
            i += 1;
        }

        if i < values.len() {
            if let Some(v) = values[i] {
                let right = Rc::new(RefCell::new(TreeNode::new(v)));
                current.borrow_mut().right = Some(right.clone());
                queue.push_back(right);
            }
            i += 1;
        }
    }

    Some(root)
}

#[must_use]
pub fn level_order_from_tree(root: Node) -> Vec<Option<i32>> {
    let mut out = Vec::new();
    let mut queue = VecDeque::new();

    queue.push_back(root);

    while let Some(node_opt) = queue.pop_front() {
        match node_opt {
            Some(node) => {
                let node_ref = node.borrow();
                out.push(Some(node_ref.val));
                queue.push_back(node_ref.left.clone());
                queue.push_back(node_ref.right.clone());
            }
            None => out.push(None),
        }
    }

    while matches!(out.last(), Some(None)) {
        out.pop();
    }

    out
}
