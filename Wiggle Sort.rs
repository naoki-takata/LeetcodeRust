impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let n = nums.len();
        
        // 配列を一度走査して、各位置で条件に合うようにスワップ
        for i in 0..n - 1 {
            if i % 2 == 0 {
                // 偶数インデックス: nums[i] <= nums[i+1] である必要がある
                if nums[i] > nums[i + 1] {
                    nums.swap(i, i + 1);
                }
            } else {
                // 奇数インデックス: nums[i] >= nums[i+1] である必要がある
                if nums[i] < nums[i + 1] {
                    nums.swap(i, i + 1);
                }
            }
        }
    }
}

