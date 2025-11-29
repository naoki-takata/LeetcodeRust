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
    pub fn closest_k_values(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: f64,
        k: i32,
    ) -> Vec<i32> {
        let mut result = Vec::new();
        let mut values = Vec::new();
        
        // インオーダー走査でソート済みリストを取得
        Self::inorder(&root, &mut values);
        
        // ターゲットに最も近い位置を見つける
        let mut closest_idx = 0;
        let mut min_diff = f64::MAX;
        
        for (i, &val) in values.iter().enumerate() {
            let diff = (val as f64 - target).abs();
            if diff < min_diff {
                min_diff = diff;
                closest_idx = i;
            }
        }
        
        // 両方向にポインタを動かしてk個を取得
        let mut left = closest_idx as i32;
        let mut right = closest_idx as i32 + 1;
        let k = k as usize;
        
        while result.len() < k {
            let left_diff = if left >= 0 {
                Some((values[left as usize] as f64 - target).abs())
            } else {
                None
            };
            
            let right_diff = if right < values.len() as i32 {
                Some((values[right as usize] as f64 - target).abs())
            } else {
                None
            };
            
            match (left_diff, right_diff) {
                (Some(l), Some(r)) => {
                    if l <= r {
                        result.push(values[left as usize]);
                        left -= 1;
                    } else {
                        result.push(values[right as usize]);
                        right += 1;
                    }
                }
                (Some(_), None) => {
                    result.push(values[left as usize]);
                    left -= 1;
                }
                (None, Some(_)) => {
                    result.push(values[right as usize]);
                    right += 1;
                }
                (None, None) => break,
            }
        }
        
        result
    }
    
    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
        if let Some(n) = node {
            let node_ref = n.borrow();
            Self::inorder(&node_ref.left, values);
            values.push(node_ref.val);
            Self::inorder(&node_ref.right, values);
        }
    }
}

