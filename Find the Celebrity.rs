// The knows API is already defined for you.
// return a bool, whether a knows b
// fn knows(a: i32, b: i32) -> bool;

impl Solution {
    pub fn find_celebrity(&self, n: i32) -> i32 {
        let n = n as usize;
        
        // ステップ1: セレブリティの候補を見つける
        // セレブリティは誰も知らないので、knows(candidate, i)がtrueになることはない
        // もしknows(candidate, i)がtrueなら、candidateはセレブリティではない
        // その場合、iはセレブリティの候補になる（なぜなら、candidateがiを知っているということは、
        // iがセレブリティである可能性がある）
        let mut candidate = 0;
        for i in 1..n {
            if self.knows(candidate as i32, i as i32) {
                // candidateがiを知っている場合、candidateはセレブリティではない
                // （セレブリティは誰も知らない）
                candidate = i;
            }
            // candidateがiを知らない場合、iはセレブリティではない
            // （全員がセレブリティを知っている必要がある）
        }
        
        // ステップ2: 候補が本当にセレブリティかどうかを検証する
        for i in 0..n {
            if i == candidate {
                continue;
            }
            
            // すべての人が候補を知っている必要がある
            if !self.knows(i as i32, candidate as i32) {
                return -1;
            }
            
            // 候補は誰も知らない必要がある
            if self.knows(candidate as i32, i as i32) {
                return -1;
            }
        }
        
        candidate as i32
    }
}

