impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        // 0以下の場合はugly numberではない
        if n <= 0 {
            return false;
        }
        
        let mut num = n;
        
        // 2で割り切れる限り割る
        while num % 2 == 0 {
            num /= 2;
        }
        
        // 3で割り切れる限り割る
        while num % 3 == 0 {
            num /= 3;
        }
        
        // 5で割り切れる限り割る
        while num % 5 == 0 {
            num /= 5;
        }
        
        // 最終的に1になったら、2、3、5以外の素因数がない
        num == 1
    }

    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut ugly = vec![1; n];
        let mut i2 = 0;
        let mut i3 = 0;
        let mut i5 = 0;
        
        for i in 1..n {
            // 次のugly numberの候補を計算
            let next2 = ugly[i2] * 2;
            let next3 = ugly[i3] * 3;
            let next5 = ugly[i5] * 5;
            
            // 最小値を選ぶ
            let next = next2.min(next3).min(next5);
            ugly[i] = next;
            
            // 使用したポインタを進める（重複を避けるため）
            if next == next2 {
                i2 += 1;
            }
            if next == next3 {
                i3 += 1;
            }
            if next == next5 {
                i5 += 1;
            }
        }
        
        ugly[n - 1]
    }
}

