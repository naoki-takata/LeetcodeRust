impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        // dp[i] = i を完全平方数の和として表すのに必要な最小の完全平方数の個数
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0; // 0は0個の完全平方数で表せる
        
        // 各数値について、完全平方数の和として表す最小個数を計算
        for i in 1..=n {
            // 1から√iまでの完全平方数を試す
            let mut j = 1;
            while j * j <= i {
                let square = j * j;
                // dp[i] = min(dp[i], dp[i - square] + 1)
                if dp[i - square] != i32::MAX {
                    dp[i] = dp[i].min(dp[i - square] + 1);
                }
                j += 1;
            }
        }
        
        dp[n]
    }
}

