impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut left = 0;
        let mut right = n;
        
        // 二分探索: citations[mid] >= n - mid となる最小のmidを見つける
        // 配列は昇順にソートされているため、位置iから最後までの論文数は n - i
        // citations[i] >= n - i なら、h-indexは少なくとも n - i
        while left < right {
            let mid = left + (right - left) / 2;
            if citations[mid] >= (n - mid) as i32 {
                // mid以降のすべての論文が少なくとも n - mid 回引用されている
                // より小さいmidを探す
                right = mid;
            } else {
                // midの論文は n - mid 回未満の引用しかない
                // より大きいmidを探す
                left = mid + 1;
            }
        }
        
        // h-index = n - left
        (n - left) as i32
    }
}

