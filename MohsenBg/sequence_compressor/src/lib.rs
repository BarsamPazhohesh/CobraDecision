const DELIMITER: char = '\0';

pub fn compress(txt: &str) -> String {
    let mut compress_string = String::new();
    
    if txt.is_empty() {
        return compress_string;
    }

    let mut counter = 1;
    let mut prv_char = txt.chars().next().unwrap();

    for current_char in txt.chars().skip(1) { 
        if current_char == prv_char {
            counter += 1;
        } else {
            compress_string.push_str(&format!("{}{}{}", counter, prv_char, DELIMITER));
            prv_char = current_char;
            counter = 1;
        }
    }

    compress_string.push_str(&format!("{}{}{}", counter, prv_char, DELIMITER));
    
    compress_string
}

pub fn decompress(txt: &str) -> String {
    let split_parts: Vec<&str> = txt.split(&DELIMITER.to_string()).collect(); 
    let mut decompressed_string = String::new(); 

    for part in split_parts {
        if !part.is_empty() {
            let number_str: String = part.chars().take(part.len() - 1).collect();
            let count: i32 = number_str.parse().unwrap_or(0); 
            let character = part.chars().last().unwrap_or('\0');

            for _ in 0..count {
                decompressed_string.push(character);
            }
        }
    }

    decompressed_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_empty_string() {
        let result = compress("");
        let result = result.replace(DELIMITER, "");
        assert_eq!(result, "");
    }

    #[test]
    fn test_compress_single_character() {
        let result = compress("aaaa");
        let result = result.replace(DELIMITER, ""); 
        assert_eq!(result, "4a");
    }

    #[test]
    fn test_compress_multiple_different_characters() {
        let result = compress("aa333bbbc");
        let result = result.replace(DELIMITER, ""); 
        assert_eq!(result, "2a333b1c");
    }

    #[test]
    fn test_compress_single_character_no_repeat() {
        let result = compress("abcd");
        let result = result.replace(DELIMITER, ""); 
        assert_eq!(result, "1a1b1c1d");
    }

    #[test]
    fn test_compress_only_one_character() {
        let result = compress("z");
        let result = result.replace(DELIMITER, ""); 
        assert_eq!(result, "1z");
    }

    #[test]
    fn test_decompress_empty_string() {
        let result = decompress("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_decompress_single_character() {
        let result = decompress(&format!("4a{}", DELIMITER));
        assert_eq!(result, "aaaa");
    }

    #[test]
    fn test_decompress_multiple_different_characters() {
        let result = decompress(&format!("2a{}3b{}1c{}", DELIMITER, DELIMITER, DELIMITER));
        assert_eq!(result, "aabbbc");
    }

    #[test]
    fn test_decompress_single_character_no_repeat() {
        let result = decompress(&format!("1a{}1b{}1c{}1d{}", DELIMITER, DELIMITER, DELIMITER, DELIMITER));
        assert_eq!(result, "abcd");
    }

    #[test]
    fn test_decompress_only_one_character() {
        let result = decompress(&format!("1z{}", DELIMITER));
        assert_eq!(result, "z");
    }
}
