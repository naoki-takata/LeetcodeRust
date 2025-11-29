// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
        let mut closest = i32::MAX;
        let mut min_diff = f64::MAX;
        
        Self::dfs(&root, target, &mut closest, &mut min_diff);
        
        closest
    }
    
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        target: f64,
        closest: &mut i32,
        min_diff: &mut f64,
    ) {
        if let Some(n) = node {
            let node_ref = n.borrow();
            let val = node_ref.val as f64;
            
            // 現在のノードとターゲットの距離を計算
            let diff = (val - target).abs();
            
            // より近い値を見つけた場合、または距離が同じでより小さい値の場合
            if diff < *min_diff || (diff == *min_diff && node_ref.val < *closest) {
                *min_diff = diff;
                *closest = node_ref.val;
            }
            
            // BSTの特性を利用: ターゲットが現在の値より小さい場合は左へ、大きい場合は右へ
            if target < val {
                Self::dfs(&node_ref.left, target, closest, min_diff);
            } else if target > val {
                Self::dfs(&node_ref.right, target, closest, min_diff);
            }
            // target == val の場合は既に最適解が見つかっているので、探索を終了
        }
    }
}

