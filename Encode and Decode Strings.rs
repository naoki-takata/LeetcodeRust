struct Codec;

impl Codec {
    fn new() -> Self {
        Codec
    }
    
    // 文字列のベクトルを1つの文字列にエンコード
    // 各文字列の長さを先頭に付けて、":"で区切る
    // 例: ["Hello", "World"] -> "5:Hello5:World"
    fn encode(&self, strs: Vec<String>) -> String {
        let mut encoded = String::new();
        for s in strs {
            encoded.push_str(&format!("{}:{}", s.len(), s));
        }
        encoded
    }
    
    // エンコードされた文字列を元の文字列のベクトルにデコード
    fn decode(&self, s: String) -> Vec<String> {
        let mut result = Vec::new();
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            // 長さを読み取る（":"まで）
            let mut len_str = String::new();
            while i < chars.len() && chars[i] != ':' {
                len_str.push(chars[i]);
                i += 1;
            }
            
            // ":"をスキップ
            if i < chars.len() {
                i += 1;
            }
            
            // 長さを数値に変換
            let len: usize = len_str.parse().unwrap_or(0);
            
            // 指定された長さ分の文字を読み取る
            let mut str_content = String::new();
            for _ in 0..len {
                if i < chars.len() {
                    str_content.push(chars[i]);
                    i += 1;
                }
            }
            
            result.push(str_content);
        }
        
        result
    }
}

