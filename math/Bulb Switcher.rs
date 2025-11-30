// Bulb Switcher
// n個の電球があり、最初はすべてオフ
// 1ラウンド目：すべての電球をオンにする
// 2ラウンド目：2つおきの電球をオフにする（2, 4, 6, ...）
// 3ラウンド目：3つおきの電球をトグルする（3, 6, 9, ...）
// iラウンド目：iつおきの電球をトグルする（i, 2i, 3i, ...）
// nラウンド目：最後の電球のみをトグルする
// 
// 解法：
// 電球iは、iの約数であるラウンドでトグルされます
// 電球が最終的にオンになるのは、トグル回数が奇数の場合
// 約数の数が奇数になるのは、完全平方数の場合のみ
// （例：1, 4, 9, 16, ...）
// したがって、n以下の完全平方数の数を返せばよい
// これは floor(sqrt(n)) で計算できます

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        // n以下の完全平方数の数 = floor(sqrt(n))
        // 例：n=3の場合、完全平方数は1のみ（1^2=1）なので1を返す
        // 例：n=9の場合、完全平方数は1,4,9（1^2, 2^2, 3^2）なので3を返す
        (n as f64).sqrt() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        // n = 3
        // ラウンド1: [on, on, on]
        // ラウンド2: [on, off, on]  (2番目をオフ)
        // ラウンド3: [on, off, off] (3番目をトグル)
        // 結果：1個がオン
        assert_eq!(Solution::bulb_switch(3), 1);
    }

    #[test]
    fn test_example2() {
        // n = 0: 電球がないので0
        assert_eq!(Solution::bulb_switch(0), 0);
    }

    #[test]
    fn test_example3() {
        // n = 1: 1は完全平方数なので1個がオン
        assert_eq!(Solution::bulb_switch(1), 1);
    }

    #[test]
    fn test_n4() {
        // n = 4: 完全平方数は1,4なので2個がオン
        // ラウンド1: [on, on, on, on]
        // ラウンド2: [on, off, on, off]  (2,4をオフ)
        // ラウンド3: [on, off, off, off] (3をトグル)
        // ラウンド4: [on, off, off, on]  (4をトグル)
        assert_eq!(Solution::bulb_switch(4), 2);
    }

    #[test]
    fn test_n9() {
        // n = 9: 完全平方数は1,4,9なので3個がオン
        assert_eq!(Solution::bulb_switch(9), 3);
    }

    #[test]
    fn test_n16() {
        // n = 16: 完全平方数は1,4,9,16なので4個がオン
        assert_eq!(Solution::bulb_switch(16), 4);
    }

    #[test]
    fn test_large_number() {
        // 大きな数でも正しく動作することを確認
        // n = 100: 完全平方数は1,4,9,16,25,36,49,64,81,100なので10個
        assert_eq!(Solution::bulb_switch(100), 10);
    }
}

