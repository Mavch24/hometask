fn gray(n: u8) -> Vec<String> {
    let mut result = vec!["0".to_string(), "1".to_string()];
    
    for _ in 1..n {
        let mut new_result = Vec::new();
        
        for code in &result {
            new_result.push("0".to_string() + code);
        }
        
        for code in result.iter().rev() {
            new_result.push("1".to_string() + code);
        }
        
        result = new_result;
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_data = [
            (0, vec!("")),
            (1, vec!("0", "1")),
            (2, vec!("00", "01", "10", "11")),
            (3, vec!("000", "001", "010", "011", 
                     "100", "101", "110", "111")),
            (4, vec!("0000", "0001", "0010", "0011", 
                     "0100", "0101", "0110", "0111", 
                     "1000", "1001", "1010", "1011",
                     "1100", "1101", "1110", "1111")),
        ];

        test_data.iter().for_each(|(n, out)| 
            assert_eq!(gray(*n), *out)
        );
    }
}
