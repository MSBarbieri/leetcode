/// Category: algorithms
/// Level: Medium
/// Percent: 72.27182%

/// Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
///
///
/// Example 1:
/// Input: n = 3
/// Output: ["((()))","(()())","(())()","()(())","()()()"]
/// Example 2:
/// Input: n = 1
/// Output: ["()"]
///
///
/// Constraints:
///
///
/// 	1 <= n <= 8
///

pub struct Solution;
/// @lc code=start
pub struct Tree {
    pub val: u32,
    pub childs: Vec<Tree>,
    pub text: String,
}
impl Tree {
    pub fn new(val: u32, text: String) -> Self {
        Tree {
            val,
            text,
            childs: vec![],
        }
    }
    pub fn clone_string(chars: &str) -> String {
        let mut text = String::default();
        for t in chars.chars() {
            text.push(t);
        }
        return text;
    }

    pub fn create_next_level(&mut self, limit: u32) {
        let mut text = self.text.clone();
        if self.val < limit {
            for (i, c) in text.chars().enumerate() {
                if c == ')' {
                    let mut s = Tree::clone_string(&text[0..i]);
                    s.push_str("()");
                    s.push_str(&text[i..self.text.len()]);
                    let mut node = Tree::new(self.val + 1, s);
                    node.create_next_level(limit);
                    self.childs.push(node);
                }
            }
            text.push_str("()");
            let mut node_2 = Tree::new(self.val + 1, text);
            node_2.create_next_level(limit);
            self.childs.push(node_2);
        }
    }

    fn _print_tree(&self, val: u32, step: usize, result: &mut Vec<String>) {
        if self.val == val {
            result.push(self.text.clone());
        } else {
            for n in step..self.childs.len() {
                self.childs.get(n).unwrap()._print_tree(val, n, result);
            }
        }
    }
    pub fn print_tree(&mut self, depth: u32) -> Vec<String> {
        let mut val = vec![];
        self._print_tree(depth, 0, &mut val);
        return val;
    }
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut tree = Tree::new(1, String::from("()"));
        tree.create_next_level(n as u32);

        return tree.print_tree(n as u32);
    }
}
/// @lc code=end
struct End;
