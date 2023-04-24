use std::env;
use hex;

const MAX_LEN: usize = 16_348;

const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVXYZ";
const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const DIGITS: &str = "0123456789";

fn gen_pattern(len: usize) -> Option<String> {
    if len > MAX_LEN {
        eprintln!("Given length not supported");
        return None;
    }

    let mut pattern = String::with_capacity(len);
    for upper in UPPER.chars() {
        for lower in LOWER.chars() {
            for digit in DIGITS.chars() {
                pattern.push(upper);
                pattern.push(lower);
                pattern.push(digit);
                if pattern.len() >= len {
                    return Some(pattern[..len].to_string());
                }
            }
        }
    }
    Some(pattern)
}

fn find_pattern(pattern: &str) -> Option<usize> {
    let needle = if pattern.starts_with("0x") {
        let needle_vec = hex::decode(&pattern[2..]).ok()?;
        let reversed: Vec<u8> = needle_vec.iter().copied().rev().collect();
        String::from_utf8(reversed).ok()?
    } else {
        pattern.to_string()
    };

    let mut haystack = String::new();
    for upper in UPPER.chars() {
        for lower in LOWER.chars() {
            for digit in DIGITS.chars() {
                haystack.push(upper);
                haystack.push(lower);
                haystack.push(digit);
                if let Some(idx) = haystack.find(&needle[..]) {
                    return Some(idx);
                }
            }
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./pat-rs <length>|<search_pattern>");
        return;
    }

    let arg = &args[1];
    if let Ok(len) = arg.parse::<usize>() {
        if let Some(pattern) = gen_pattern(len) {
            println!("{}", pattern);
        } else {
            eprintln!("Generating pattern failed");
        }
    } else if let Some(pos) = find_pattern(arg) {
        println!(
            "Pattern {} found at position {} (first occurrence)",
            arg,
            pos
        );
    } else {
        println!("Pattern not found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_pattern() {
        assert_eq!(gen_pattern(23), Some("Aa0Aa1Aa2Aa3Aa4Aa5Aa6Aa".to_string()));
        assert_ne!(gen_pattern(20), Some("Aa1Aa2Aa3Aa4Aa5Aa6Aa".to_string()));
        assert_eq!(gen_pattern(0), Some("".to_string()));
        assert_eq!(gen_pattern(MAX_LEN + 1), None);
    }

    #[test]
    fn test_find_pattern() {
        assert_eq!(find_pattern("Aa5"), Some(15));
        assert_eq!(find_pattern("0x42346642"), Some(942));
        assert_eq!(find_pattern("423642"), None);
    }
}
