use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
impl Solution{
pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, k: i32) -> Vec<i32> {
    let mut graph = HashMap::new();
    build(&root, &None, &mut graph);
    let mut ans = vec![];
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let target_val = target.as_ref().unwrap().borrow().val;
    queue.push_back((target_val, 0));
    visited.insert(target_val);
    while let Some((curr, dist)) = queue.pop_front() {
        if dist == k {
            ans.push(curr);
            continue;
        }
        if let Some(neighbors) = graph.get(&curr) {
            for &next in neighbors {
                if !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back((next, dist + 1));
                }
            }
        }
    }
    ans
}
}
fn build(curr: &Option<Rc<RefCell<TreeNode>>>, parent: &Option<Rc<RefCell<TreeNode>>>, graph: &mut HashMap<i32, Vec<i32>>) {
    if let Some(curr_node) = curr.as_ref() {
        if let Some(parent_node) = parent.as_ref() {
            let cur_val = curr_node.borrow().val;
            let parent_val = parent_node.borrow().val;
            graph.entry(cur_val).or_insert(Vec::new()).push(parent_val);
            graph.entry(parent_val).or_insert(Vec::new()).push(cur_val);
        }
        build(&curr_node.borrow().left, curr, graph);
        build(&curr_node.borrow().right, curr, graph);
    }
}
