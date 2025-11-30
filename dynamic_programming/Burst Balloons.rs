// Burst Balloons
// 風船を割って得られる最大コイン数を求める
// 区間DPを使用: dp[left][right]は、leftからrightの範囲（両端を除く）の風船をすべて割ったときの最大コイン数

// 注意: LeetCode提出時は以下の行を削除またはコメントアウトしてください
// LeetCode環境では既に`Solution`構造体が定義されています

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        
        // 配列の両端に1を追加して、範囲外を1として扱えるようにする
        let mut balloons = vec![1];
        balloons.extend(nums);
        balloons.push(1);
        let m = balloons.len();
        
        // dp[left][right]: leftからrightの範囲（両端を除く）の風船をすべて割ったときの最大コイン数
        // leftとrightは境界（1）を表し、実際の風船はleft+1からright-1の範囲
        let mut dp = vec![vec![0; m]; m];
        
        // 範囲の長さを小さい順に計算
        // lengthは実際の風船の数（境界を除く）
        for length in 1..=n {
            // leftは左端の境界（0から始まる）
            for left in 0..=m - length - 2 {
                let right = left + length + 1; // 右端の境界
                
                // 最後に割る風船をkとする（left < k < right）
                // kを割ると、nums[left] * nums[k] * nums[right]のコインが得られる
                for k in (left + 1)..right {
                    let coins = dp[left][k] + dp[k][right] + balloons[left] * balloons[k] * balloons[right];
                    dp[left][right] = dp[left][right].max(coins);
                }
            }
        }
        
        // 全体の範囲（0からm-1、つまり最初と最後の1の間）の最大コイン数
        dp[0][m - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::max_coins(vec![1, 5]), 10);
    }

    #[test]
    fn test_single_balloon() {
        assert_eq!(Solution::max_coins(vec![5]), 5);
    }
}

