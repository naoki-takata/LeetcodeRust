impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }
        
        let mut result = Vec::new();
        let num = num as u64;
        
        // 単位: Billion, Million, Thousand
        let units = vec!["Billion", "Million", "Thousand"];
        let mut value = num;
        
        // Billion (10^9)
        if value >= 1_000_000_000 {
            let billions = value / 1_000_000_000;
            result.extend(Self::convert_three_digits(billions as i32));
            result.push("Billion".to_string());
            value %= 1_000_000_000;
        }
        
        // Million (10^6)
        if value >= 1_000_000 {
            let millions = value / 1_000_000;
            result.extend(Self::convert_three_digits(millions as i32));
            result.push("Million".to_string());
            value %= 1_000_000;
        }
        
        // Thousand (10^3)
        if value >= 1_000 {
            let thousands = value / 1_000;
            result.extend(Self::convert_three_digits(thousands as i32));
            result.push("Thousand".to_string());
            value %= 1_000;
        }
        
        // 残りの3桁未満の部分
        if value > 0 {
            result.extend(Self::convert_three_digits(value as i32));
        }
        
        result.join(" ")
    }
    
    // 3桁以下の数値を英語の単語に変換
    fn convert_three_digits(num: i32) -> Vec<String> {
        let mut result = Vec::new();
        
        if num == 0 {
            return result;
        }
        
        // 100の位
        if num >= 100 {
            let hundreds = num / 100;
            result.push(Self::ones(hundreds).to_string());
            result.push("Hundred".to_string());
        }
        
        // 残りの2桁
        let remainder = num % 100;
        if remainder > 0 {
            if remainder < 20 {
                result.push(Self::ones(remainder).to_string());
            } else {
                let tens = remainder / 10;
                let ones = remainder % 10;
                result.push(Self::tens(tens).to_string());
                if ones > 0 {
                    result.push(Self::ones(ones).to_string());
                }
            }
        }
        
        result
    }
    
    // 1-19の数字を英語に変換
    fn ones(num: i32) -> &'static str {
        match num {
            1 => "One",
            2 => "Two",
            3 => "Three",
            4 => "Four",
            5 => "Five",
            6 => "Six",
            7 => "Seven",
            8 => "Eight",
            9 => "Nine",
            10 => "Ten",
            11 => "Eleven",
            12 => "Twelve",
            13 => "Thirteen",
            14 => "Fourteen",
            15 => "Fifteen",
            16 => "Sixteen",
            17 => "Seventeen",
            18 => "Eighteen",
            19 => "Nineteen",
            _ => "",
        }
    }
    
    // 10の位（20-90）を英語に変換
    fn tens(num: i32) -> &'static str {
        match num {
            2 => "Twenty",
            3 => "Thirty",
            4 => "Forty",
            5 => "Fifty",
            6 => "Sixty",
            7 => "Seventy",
            8 => "Eighty",
            9 => "Ninety",
            _ => "",
        }
    }
}

