use std::char;

use helper::read_input;

fn gather_input() -> Vec<String> {
    let input_file = r"2025\inputs\input_3.txt";
    let contents = read_input(&input_file);
    contents
}

fn sort_joltages(line: String) -> Vec<char> {
    let mut joltages: Vec<char> = line.chars().collect();
    let num_jolts = joltages.len();
    
    for i in 0..num_jolts {
        for j in i+1..num_jolts {
            // Re-read both values for each comparison
            let curr_jolt: i32 = joltages[i].to_string().parse().unwrap_or(0);
            let other_jolt: i32 = joltages[j].to_string().parse().unwrap_or(0);
            if curr_jolt > other_jolt {
                joltages.swap(i, j);
            }
        }
    }
    joltages
}

fn sort_contents(contents: Vec<String>) -> Vec<Vec<char>> {
    contents.iter()
        .map(|line| sort_joltages(line.clone()))
        .collect()
}

fn solve_part_1(contents: Vec<String>) {
    let mut cum_max_joltage: i64 = 0;
    for battery_bank in contents {
        let joltages: Vec<char> = battery_bank.chars().collect();
        let mut max_joltage = 0;
        let count_joltages = joltages.len();
        
        for i in 0..count_joltages {
            let curr_joltage = joltages[i].to_string();
            
            for j in i+1..count_joltages {
                let target_joltage = joltages[j].to_string();
                
                let combined = curr_joltage.clone() + &target_joltage;
                let joltage_size: i64 = combined.parse().unwrap_or(0);
            
                
                if joltage_size > max_joltage {
                    max_joltage = joltage_size;
                }
            }
        }
        cum_max_joltage += max_joltage;
        println!("Max joltage for battery bank: {}", max_joltage);
    }
    println!("Cum max joltage {cum_max_joltage}");
}

fn solve_part_2(norm_contents: Vec<String>) {
    let mut cum_max_joltage: i128 = 0;
    
    println!("=== Part 2: Finding Max 12-Digit Numbers ===\n");
    
    for (idx, battery_bank) in norm_contents.iter().enumerate() {
        println!("Battery Bank #{}: {}", idx + 1, battery_bank);
        println!("  Length: {} digits", battery_bank.len());
        
        let joltages: Vec<char> = battery_bank.chars().collect();
        let max_jolts = find_max_12_digit(&joltages);
        
        println!("  Max 12-digit: {}", max_jolts);
        cum_max_joltage += max_jolts;
        println!("  Running total: {}\n", cum_max_joltage);
    }
    
    println!("=== Final Result ===");
    println!("Cumulative max joltage: {}", cum_max_joltage);
}

fn shuffle_array(mut digits: Vec<char>, from_idx: usize) -> Vec<char> {
    // moves digits from right to left from start index.
    let n = digits.len();
    
    if from_idx >= n {
        return digits;
    }
    
    // Shift all elements from from_idx to the left by one position
    for i in from_idx..n-1 {
        digits[i] = digits[i + 1];
    }
    
    digits
}

fn find_max_12_digit(digits: &[char]) -> i128 {
    let n = digits.len();
    let k = 12; // need exactly 12

    println!("    [DEBUG] Input digits: {:?}", digits.iter().collect::<String>());
    println!("    [DEBUG] Total digits: {}, Need: {}", n, k);

    let mut result = String::new();
    let mut start_idx = 12;

    // first get the first k digits as a vector
    let mut joltages: Vec<char> = digits[..k].to_vec();
    println!("    [DEBUG] Initial 12 digits: {:?}", joltages.iter().collect::<String>());
    
    for i in start_idx..n {
        let new_digit = digits[i];
        // println!("    [DEBUG] Processing digit at idx {}: '{}'", i, new_digit);
        // println!("    [DEBUG] Current joltages: {:?}", joltages.iter().collect::<String>());
        
        for j in 0..k-1 {
            if joltages[j] < joltages[j+1] {
                //println!("    [DEBUG]   Found swap at position {}: '{}' < '{}'", j, joltages[j], joltages[j+1]);
                joltages[j] = joltages[j+1];
                joltages = shuffle_array(joltages, j+1);
                joltages[k-1] = new_digit;
                //println!("    [DEBUG]   After shuffle: {:?}", joltages.iter().collect::<String>());
                break;
            }
        }
        if joltages[k-1] < new_digit {
            joltages[k-1] = new_digit;
        }
        //println!("    [DEBUG] Final 12 digits: {:?}", joltages);
    }
    
    let result_str: String = joltages.iter().collect();
    let result_val = result_str.parse::<i128>().unwrap_or(0);
    println!("    [DEBUG] Final 12 digits: {}", result_str);
    println!("    [DEBUG] Final value: {}", result_val);
    
    result_val
}


pub fn day_3_solve() {

    let contents = gather_input();
    //solve_part_1(contents);
    let sorted_contents = sort_contents(contents.clone());
    solve_part_2(contents.clone());
}