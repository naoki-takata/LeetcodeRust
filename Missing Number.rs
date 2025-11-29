impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        // 0からnまでの期待される合計
        let expected_sum = n * (n + 1) / 2;
        // 実際の配列の合計
        let actual_sum: i32 = nums.iter().sum();
        // 欠けている数 = 期待される合計 - 実際の合計
        expected_sum - actual_sum
    }
    
    // XORアプローチ（Follow upの要件を満たす別解）
    pub fn missing_number_xor(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        // 0からnまでのすべての数と配列のすべての数をXOR
        // 同じ数はXORで0になるため、欠けている数だけが残る
        let mut result = 0;
        for i in 0..=n {
            result ^= i;
        }
        for &num in &nums {
            result ^= num;
        }
        result
    }
}

