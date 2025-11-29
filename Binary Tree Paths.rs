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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result = Vec::new();
        let mut path = Vec::new();
        
        Self::dfs(&root, &mut path, &mut result);
        
        result
    }
    
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        path: &mut Vec<i32>,
        result: &mut Vec<String>,
    ) {
        if let Some(n) = node {
            let node_ref = n.borrow();
            path.push(node_ref.val);
            
            // 葉ノードの場合（左右の子が両方None）
            if node_ref.left.is_none() && node_ref.right.is_none() {
                // パスを文字列に変換して追加
                let path_str = path.iter()
                    .map(|&x| x.to_string())
                    .collect::<Vec<_>>()
                    .join("->");
                result.push(path_str);
            } else {
                // 左の子を探索
                Self::dfs(&node_ref.left, path, result);
                // 右の子を探索
                Self::dfs(&node_ref.right, path, result);
            }
            
            // バックトラック: 現在のノードをパスから削除
            path.pop();
        }
    }
}

