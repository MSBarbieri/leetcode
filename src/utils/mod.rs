use anyhow::Result;
use serde_json::Value;
use std::fs;

mod list_node;
pub use list_node::ListNode;

pub fn read_file(path: &str) -> Result<Vec<Value>> {
    let content = fs::read_to_string(path)?;
    Ok(content
        .trim()
        .split('\n')
        .flat_map(|t| serde_json::from_str(t))
        .collect::<Vec<Value>>())
}
