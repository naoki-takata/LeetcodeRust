impl Solution {
    pub fn three_sum_smaller(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        if n < 3 {
            return 0;
        }
        
        let mut nums = nums;
        nums.sort();
        
        let mut count = 0;
        
        // 最初の要素を固定
        for i in 0..n - 2 {
            let mut j = i + 1;
            let mut k = n - 1;
            
            // 2ポインタで探索
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                
                if sum < target {
                    // nums[i] + nums[j] + nums[k] < target の場合、
                    // nums[i] + nums[j] + nums[k], nums[i] + nums[j] + nums[k-1], ...
                    // すべて有効なので、k - j 個の組み合わせが有効
                    count += (k - j) as i32;
                    j += 1;
                } else {
                    // sum >= target の場合、kを減らす
                    k -= 1;
                }
            }
        }
        
        count
    }
}

