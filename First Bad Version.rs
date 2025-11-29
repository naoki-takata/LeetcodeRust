// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        
        // 二分探索で最初のbadバージョンを見つける
        while left < right {
            let mid = left + (right - left) / 2; // オーバーフローを防ぐため
            
            if self.isBadVersion(mid) {
                // midがbadの場合、最初のbadバージョンはmidまたはそれより前にある
                right = mid;
            } else {
                // midがgoodの場合、最初のbadバージョンはmidより後にある
                left = mid + 1;
            }
        }
        
        // left == right のとき、それが最初のbadバージョン
        left
    }
}

