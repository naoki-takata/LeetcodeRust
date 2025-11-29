impl Solution {
    pub fn verify_preorder(preorder: Vec<i32>) -> bool {
        // スタックを使ってBSTの性質を検証
        // スタックには、現在のノードの祖先ノードを保存
        let mut stack = Vec::new();
        let mut lower_bound = i32::MIN;
        
        for &val in &preorder {
            // 現在の値がlower_boundより小さい場合、BSTの性質に違反
            if val < lower_bound {
                return false;
            }
            
            // スタックから、現在の値より小さいノードをすべて取り出す
            // これにより、現在のノードが右の子である場合の親ノードを見つける
            while let Some(&top) = stack.last() {
                if val > top {
                    // 右の子に移動したので、lower_boundを更新
                    lower_bound = stack.pop().unwrap();
                } else {
                    break;
                }
            }
            
            // 現在のノードをスタックに追加
            stack.push(val);
        }
        
        true
    }
}

