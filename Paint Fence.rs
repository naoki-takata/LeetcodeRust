impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as i32;
        
        // エッジケース: n=1の場合はk通り
        if n == 1 {
            return k;
        }
        
        // same[i]: i番目のポストまでで、最後の2つが同じ色の場合の数
        // diff[i]: i番目のポストまでで、最後の2つが異なる色の場合の数
        let mut same = 0; // same[1] = 0 (1つだけでは同じ色のペアは作れない)
        let mut diff = k; // diff[1] = k (最初のポストはk色選べる)
        
        for i in 2..=n {
            // 前の状態を保存
            let prev_same = same;
            let prev_diff = diff;
            
            // 遷移:
            // same[i] = diff[i-1] (前の2つが異なる色の場合のみ、同じ色を選べる)
            // diff[i] = (same[i-1] + diff[i-1]) * (k-1) (前の状態に関係なく、前のポストと異なる色を選ぶ)
            same = prev_diff;
            diff = (prev_same + prev_diff) * (k - 1);
        }
        
        same + diff
    }
}

