use serde::{Deserialize, Serialize};
use serde_json::Value;

/// ListNode is a base struct used in leetcode exercises, is replicated here to
/// make the code possible to be compiled colally.
#[derive(PartialEq, Eq, Clone, Debug, Default, Serialize, Deserialize)]
pub struct ListNode<T>
where
    T: Clone + Serialize + Default,
{
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T: Clone + Serialize + Default> ListNode<T> {
    #[inline]
    fn new(val: T) -> Self {
        ListNode { next: None, val }
    }

    /// this method only exists for construction by serde, should not be used
    /// in leetcode exercises.
    fn _append(cur_node: &mut ListNode<T>, elem: T) {
        match cur_node.next {
            Some(ref mut node) => ListNode::_append(node, elem),
            None => {
                let node = ListNode::new(elem);
                cur_node.next = Some(Box::new(node));
            }
        }
    }

    fn from_value(val: Value, func: fn(Value) -> T) -> Option<Box<Self>> {
        let mut result = ListNode::new(T::default());
        if val.is_array() {
            for n in val.as_array().unwrap().iter() {
                ListNode::_append(&mut result, func(n.clone()));
            }
        } else {
            result.next = Some(Box::new(ListNode::new(func(val))));
        }
        result.next
    }
}
