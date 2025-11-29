impl Solution {
    pub fn get_factors(n: i32) -> Vec<Vec<i32>> {
        // nが1以下の場合は空のリストを返す
        if n <= 1 {
            return Vec::new();
        }
        
        let mut result = Vec::new();
        let mut current = Vec::new();
        
        Self::backtrack(n, n, 2, &mut current, &mut result);
        
        result
    }
    
    fn backtrack(original_n: i32, n: i32, start: i32, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        // startからsqrt(n)まで探索
        let mut i = start;
        while i * i <= n {
            if n % i == 0 {
                // iが因数である場合
                current.push(i);
                // 残りの因数を再帰的に探索
                Self::backtrack(original_n, n / i, i, current, result);
                current.pop();
            }
            i += 1;
        }
        
        // n自体も因数として追加できる場合
        // 条件: n >= start（順序を保つため）、currentが空でない（少なくとも1つの因数がある）、
        // そしてnが元の値より小さい（n自体を単独で返さないため）
        if n >= start && !current.is_empty() && n < original_n {
            current.push(n);
            result.push(current.clone());
            current.pop();
        }
    }
}

