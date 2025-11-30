// Super Ugly Number
// 指定された素数の配列の素因数のみを持つn番目の正の整数を求める
// 動的プログラミング + ポインタ配列を使用

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let n = n as usize;
        let k = primes.len();
        
        // dp[i] = i番目のsuper ugly number（0-indexed）
        let mut dp = vec![1; n];
        
        // 各素数に対して、その素数を掛けた次の候補を追跡するインデックス
        // indices[i] = 素数primes[i]に対して次に掛けるべきdpのインデックス
        let mut indices = vec![0; k];
        
        for i in 1..n {
            // すべての素数×現在のインデックス位置の値の最小値を求める
            // オーバーフローを防ぐため、i64で計算してからi32に戻す
            let mut min_val = i64::MAX;
            for j in 0..k {
                let candidate = dp[indices[j]] as i64 * primes[j] as i64;
                min_val = min_val.min(candidate);
            }
            
            dp[i] = min_val as i32;
            
            // 最小値と等しい候補を生成したすべてのポインタを進める（重複を避けるため）
            for j in 0..k {
                if dp[indices[j]] as i64 * primes[j] as i64 == min_val {
                    indices[j] += 1;
                }
            }
        }
        
        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::nth_super_ugly_number(12, vec![2, 7, 13, 19]), 32);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::nth_super_ugly_number(1, vec![2, 3, 5]), 1);
    }

    #[test]
    fn test_single_prime() {
        assert_eq!(Solution::nth_super_ugly_number(5, vec![2]), 16);
        // [1, 2, 4, 8, 16]
    }

    #[test]
    fn test_two_primes() {
        assert_eq!(Solution::nth_super_ugly_number(10, vec![2, 3]), 12);
        // [1, 2, 3, 4, 6, 8, 9, 12, 16, 18]
    }
}

