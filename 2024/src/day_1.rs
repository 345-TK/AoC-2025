use std::{collections::HashMap, mem::swap};

use helper::read_input;

fn build_lists(contents: &Vec<String>) -> (Vec<i32>, Vec<i32>) {

    let mut nums1: Vec<i32> = Vec::new();
    let mut nums2: Vec<i32> = Vec::new();

    for item in contents {
        let nums: Vec<i32> = item
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        if nums.len() >= 2 {
            nums1.push(nums[0]);
            nums2.push(nums[1]);
        }
    }
    return (nums1, nums2)
}

fn sort_list(mut list: Vec<i32>) -> (Vec<i32>, HashMap<i32, i32>) {
    let n = list.len();
    let mut counts: HashMap<i32, i32> = HashMap::new();
    
    // Count all elements first
    for &num in &list {
        *counts.entry(num).or_insert(0) += 1;
    }
    
    // Bubble sort
    for i in 0..n-1 {
        let mut swapped = false;
        for j in 0..n-i-1 {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }

    (list, counts)
}

pub fn solve() {
    let input_file = r"2024\inputs\input_1.txt";
    let contents = read_input(&input_file);
    let lists = build_lists(&contents);
    let list_and_counts_1 = sort_list(lists.0);
    let list_and_counts_2 = sort_list(lists.1);
    let list_1 = list_and_counts_1.0;
    let list_2 = list_and_counts_2.0;
    let counts_2 = list_and_counts_2.1;

    let mut cum_distance = 0;
    for i in 0..list_1.len() {
        let distance = (list_1[i] - list_2[i]).abs();
        println!("L_1 {} - {} = {}", list_1[i], list_2[i], distance);
        cum_distance += distance;
    }

    let mut sim_score_total = 0;
    for n in list_1 {
        let freq = counts_2.get(&n).unwrap_or(&0);
        let sim_score = n * freq;
        println!("N {n} freq {freq} sim_score {sim_score}");
        sim_score_total += sim_score;

    }
    println!("Similarity score: {}", sim_score_total);
    println!("Cum distance {cum_distance}");
}

pub fn main() {
    solve();
}