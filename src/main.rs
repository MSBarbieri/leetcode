#![allow(dead_code)]
use std::{fmt::Debug, fs};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(PartialEq, Eq, Clone, Debug, Default, Serialize, Deserialize)]
pub struct ListNode<T: Clone + Serialize + Default + Debug> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T: Clone + Serialize + Default + Debug> ListNode<T> {
    #[inline]
    fn new(val: T) -> Self {
        ListNode { next: None, val }
    }

    fn _append(&mut self, elem: T) {
        match self.next {
            Some(ref mut node) => node._append(elem),
            None => {
                let node = ListNode::new(elem);
                self.next = Some(Box::new(node));
            }
        }
    }
    fn from_value(val: Value, func: fn(Value) -> T) -> Option<Box<Self>> {
        let mut result = ListNode::new(T::default());
        if val.is_array() {
            for n in val.as_array().unwrap().iter() {
                result._append(func(n.clone()));
            }
        } else {
            result.next = Some(Box::new(ListNode::new(func(val))));
        }
        result.next
    }
}

#[path = "2.rs"]
pub mod current_import;

fn main() -> Result<()> {
    pub use current_import::Solution;

    let values = read_file("./src/2.tests.dat")?;
    // calling code change by every test
    // ----------------------------------
    for i in (0..values.len()).step_by(2) {
        let l1 = ListNode::from_value(values[i].clone(), |v| return v.as_i64().unwrap() as i32);
        let l2 = ListNode::from_value(values[i + 1].clone(), |v| v.as_i64().unwrap() as i32);

        let result = current_import::Solution::add_two_numbers(l1, l2);
        // println!("{:?}", result);
    }
    // -----------------------------------

    Ok(())
}

pub fn read_file(path: &str) -> Result<Vec<Value>> {
    let content = fs::read_to_string(path)?;
    Ok(content
        .trim()
        .split('\n')
        .flat_map(|t| serde_json::from_str(t))
        .collect::<Vec<Value>>())
}
