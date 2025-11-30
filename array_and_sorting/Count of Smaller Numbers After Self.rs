impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];
        // (値, 元のインデックス)のペアを作成
        let mut pairs: Vec<(i32, usize)> = nums.into_iter().enumerate().map(|(i, v)| (v, i)).collect();
        
        Self::merge_sort(&mut pairs[..], &mut result);
        
        result
    }
    
    fn merge_sort(pairs: &mut [(i32, usize)], result: &mut Vec<i32>) {
        let n = pairs.len();
        if n <= 1 {
            return;
        }
        
        let mid = n / 2;
        Self::merge_sort(&mut pairs[..mid], result);
        Self::merge_sort(&mut pairs[mid..], result);
        Self::merge(pairs, result, mid);
    }
    
    fn merge(pairs: &mut [(i32, usize)], result: &mut Vec<i32>, mid: usize) {
        let n = pairs.len();
        let mut left = 0;
        let mut right = mid;
        let mut merged = Vec::new();
        let mut right_count = 0; // 右側の配列からマージされた要素の数
        
        // マージ処理
        while left < mid && right < n {
            if pairs[left].0 <= pairs[right].0 {
                // 左側の要素をマージする際、現在までに右側からマージされた要素の数をカウントに追加
                result[pairs[left].1] += right_count;
                merged.push(pairs[left]);
                left += 1;
            } else {
                // 右側の要素が小さい場合、right_countを増やす
                right_count += 1;
                merged.push(pairs[right]);
                right += 1;
            }
        }
        
        // 残りの左側の要素を処理
        while left < mid {
            result[pairs[left].1] += right_count;
            merged.push(pairs[left]);
            left += 1;
        }
        
        // 残りの右側の要素を処理
        while right < n {
            merged.push(pairs[right]);
            right += 1;
        }
        
        // マージ結果を元の配列にコピー
        for (i, &pair) in merged.iter().enumerate() {
            pairs[i] = pair;
        }
    }
}

