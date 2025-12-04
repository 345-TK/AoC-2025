use helper::read_input;

struct Operation {
    first: String,
    second: i32,
}

fn process_operation(operation: &str) -> Operation {
    // Each operation is a direction L OR R plus number
    let halves = operation.split_at(1);
    let direction = halves.0;
    let clicks = halves.1;
    let num_clicks: i32 = clicks.parse().expect("Failed to parse operation");
    return Operation { first: direction.to_string(), second: num_clicks }
}

fn perform_operation(current_dial: i32, direction: &str, clicks_to_make: i32) -> (i32, i32) {
    let turns = clicks_to_make / 100;  // Each full 100 clicks = 1 pass through 0
    let rotation = clicks_to_make % 100;  // Remaining partial rotation
    
    let mut full_passes = turns;
    
    let new_dial = if direction == "L" {
        // Check if partial rotation crosses 0
        if current_dial > 0 && (current_dial - rotation) <= 0 {
            full_passes += 1;
        }
        (current_dial - rotation).rem_euclid(100)
    } else {
        // Check if partial rotation crosses 0 (wrapping from 99 to 0)
        if current_dial + rotation >= 100 {
            full_passes += 1;
        }
        (current_dial + rotation).rem_euclid(100)
    };
    
    (new_dial, full_passes)
}

pub fn day_1_solve() {
    let file_path = r".\inputs\input_1.txt";
    let contents = read_input(&file_path);

    let mut current_dial_number: i32 = 50;
    let mut num_dial_at_zero = 0;

    println!("Dial starts at {current_dial_number}");
    for i in contents {
        let operations = process_operation(&i);
        let dial_and_passes = perform_operation(current_dial_number, &operations.first, operations.second);
        current_dial_number = dial_and_passes.0;
        //num_dial_at_zero += if current_dial_number == 0 {1} else {0};
        num_dial_at_zero += dial_and_passes.1;
    }
    println!("Number of times the dial was at zero or past 0: {num_dial_at_zero}");
}