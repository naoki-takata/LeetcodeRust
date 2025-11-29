impl Solution {
    pub fn min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
        let n = costs.len();
        if n == 0 {
            return 0;
        }
        let k = costs[0].len();
        
        // 前の家の各色での最小コストを保持
        let mut prev = costs[0].clone();
        
        // 各家について処理
        for i in 1..n {
            // 前の家の最小値と2番目の最小値を求める（最適化のため）
            let mut min1 = i32::MAX;
            let mut min2 = i32::MAX;
            let mut min1_idx = 0;
            
            for j in 0..k {
                if prev[j] < min1 {
                    min2 = min1;
                    min1 = prev[j];
                    min1_idx = j;
                } else if prev[j] < min2 {
                    min2 = prev[j];
                }
            }
            
            // 現在の家の各色での最小コストを計算
            let mut curr = vec![0; k];
            for j in 0..k {
                // 前の家が色jでない場合の最小コストを使用
                if j == min1_idx {
                    curr[j] = costs[i][j] + min2;
                } else {
                    curr[j] = costs[i][j] + min1;
                }
            }
            
            prev = curr;
        }
        
        // 最後の家の各色での最小コストの最小値を返す
        *prev.iter().min().unwrap()
    }
}

