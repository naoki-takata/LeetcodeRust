impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n < 2 {
            return 0;
        }
        
        // 3つの状態を管理:
        // hold: 株を持っている状態での最大利益
        // not_hold: 株を持っていない状態での最大利益（クールダウン中でない）
        // cooldown: クールダウン中（前日に売った）状態での最大利益
        let mut hold = -prices[0]; // 最初の日に買った場合
        let mut not_hold = 0; // 最初の日に何もしなかった場合
        let mut cooldown = 0; // 最初の日はクールダウン状態にならない
        
        for i in 1..n {
            let prev_hold = hold;
            let prev_not_hold = not_hold;
            let prev_cooldown = cooldown;
            
            // 今日株を持っている状態:
            // 1. 前日から持ち続ける
            // 2. 前日に持っていなくて（クールダウン中でない）今日買う
            hold = prev_hold.max(prev_not_hold - prices[i]);
            
            // 今日株を持っていない状態（クールダウン中でない）:
            // 1. 前日から持っていないまま（クールダウン中でない）
            // 2. 前日がクールダウン中だった
            not_hold = prev_not_hold.max(prev_cooldown);
            
            // 今日がクールダウン中:
            // 前日に株を持っていて今日売った
            cooldown = prev_hold + prices[i];
        }
        
        // 最終日の最大利益は、株を持っていない状態またはクールダウン中の最大値
        not_hold.max(cooldown)
    }
}

