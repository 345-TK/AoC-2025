use std::char;

use helper::read_input;

fn gather_input() -> Vec<(i64, i64)> {
    let input_file = r"2025\inputs\input_2.txt";
    let contents = read_input(&input_file);
    let range_line = &contents[0];
    
    range_line.split(',')
        .filter_map(|range| {
            let nums: Vec<i64> = range.split('-')
                .filter_map(|x| x.trim().parse().ok())
                .collect();
            
            if nums.len() == 2 {
                Some((nums[0], nums[1]))
            } else {
                None
            }
        })
        .collect()
}

fn detect_invalid_pattern(id: i64, char_patterns: Vec<String>) -> Option<i64> {
    for pattern in char_patterns {       
        if pattern.len() % 2 != 0 {
            continue;
        }
        
        let halves = pattern.split_at(pattern.len() / 2);
        if halves.0 == halves.1 {
            println!("          -> INVALID! Pattern repeats {0} | {1}", halves.0, halves.1);
            return Some(id);  // Return the ID that has invalid pattern
        }
    }
    
    None  // No invalid patterns found
}

fn detect_invalid_pattern_p2(pattern: String) -> Option<i64> {
    // Check if the pattern is made of a repeating substring (at least 2 times)
    let len = pattern.len();
    
    // Try all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=(len / 2) {
        // Pattern must divide evenly into the total length
        if len % pattern_len == 0 {
            let repeat_count = len / pattern_len;
            if repeat_count >= 2 {
                let substring = &pattern[0..pattern_len];
                // Check if the entire pattern is this substring repeated
                let repeated = substring.repeat(repeat_count);
                if repeated == pattern {
                    println!("    INVALID ID {}: '{}' repeated {} times", pattern, substring, repeat_count);
                    return pattern.parse().ok();
                }
            }
        }
    }
    
    None
}

fn detect_invalid_ids_in_range(ranges: (i64, i64)) -> i128 {
    println!("\n=== Processing range: {} to {} ===", ranges.0, ranges.1);
    let mut invalid_ids: Vec<i64> = Vec::new();
    
    for i in ranges.0..=ranges.1 {
        let num_str = i.to_string();
        if let Some(invalid_id) = detect_invalid_pattern_p2(num_str) {
            invalid_ids.push(invalid_id);
        }
    }
    
    println!("=== Range summary: {} checked, {} invalid ===\n", 
             ranges.1 - ranges.0 + 1, invalid_ids.len());
    
    invalid_ids.iter().map(|&x| x as i128).sum()
}

pub fn day_2_solve() {
    let ranges = gather_input();
    let mut invalid_sum = 0;
    for range in ranges {
        invalid_sum += detect_invalid_ids_in_range(range);
    }
    println!("=========================================");
    println!("Invalid sums {invalid_sum}!");
}