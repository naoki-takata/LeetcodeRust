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
use std::collections::{VecDeque, BTreeMap};

impl Solution {
    pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }

        // BFSで各ノードを(列インデックス, 行インデックス, 値)として保存
        // BTreeMapを使用して列インデックスで自動的にソート
        let mut column_map: BTreeMap<i32, Vec<(i32, i32)>> = BTreeMap::new();
        // (ノード, 列インデックス, 行インデックス)
        let mut queue = VecDeque::new();
        queue.push_back((root.unwrap(), 0, 0));

        while let Some((node, col, row)) = queue.pop_front() {
            let node_ref = node.borrow();
            
            // 列ごとにグループ化（行インデックスと値のペアを保存）
            column_map
                .entry(col)
                .or_insert_with(Vec::new)
                .push((row, node_ref.val));

            // 左の子は列-1、右の子は列+1
            if let Some(left) = &node_ref.left {
                queue.push_back((Rc::clone(left), col - 1, row + 1));
            }
            if let Some(right) = &node_ref.right {
                queue.push_back((Rc::clone(right), col + 1, row + 1));
            }
        }

        // 各列について、行インデックスでソート（同じ行内では左から右の順序は既に保たれている）
        let mut result = Vec::new();
        for (_, mut values) in column_map {
            // 行インデックスでソート
            values.sort_by_key(|&(row, _)| row);
            // 値だけを抽出
            result.push(values.into_iter().map(|(_, val)| val).collect());
        }

        result
    }
}

