impl Solution {
    // 方法1: ループを使った実装
    pub fn add_digits(num: i32) -> i32 {
        let mut n = num;
        while n >= 10 {
            let mut sum = 0;
            while n > 0 {
                sum += n % 10;
                n /= 10;
            }
            n = sum;
        }
        n
    }
    
    // 方法2: O(1)の数学的解法（フォローアップ）
    // 数字根（digital root）の公式を使用
    // num == 0 の場合: 0
    // num % 9 == 0 の場合: 9
    // それ以外: num % 9
    pub fn add_digits_o1(num: i32) -> i32 {
        if num == 0 {
            0
        } else if num % 9 == 0 {
            9
        } else {
            num % 9
        }
    }
    
    // 方法3: より簡潔なO(1)実装
    // 1 + (num - 1) % 9 という式を使うと、0の場合も正しく処理できる
    pub fn add_digits_o1_compact(num: i32) -> i32 {
        1 + (num - 1) % 9
    }
}

