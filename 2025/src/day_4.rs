use helper::read_input;

fn read_contents() -> Vec<String> {
    read_input(r"2025\inputs\input_4.txt")
}

fn count_adjacents(adjacents: Vec<(usize, usize, char)>) -> usize {
    adjacents.iter().filter(|x| x.2 == '@').count()
}

fn placement_valid(count_adjacents: i32) -> bool {
    if count_adjacents < 4 {
        return true;
    }
    return false;
}


fn get_adjacent(i: usize, j: usize, grid: &Vec<Vec<char>>) -> Vec<(usize, usize, char)> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut adjacent = Vec::new();
    
    // 8 directions: N, NE, E, SE, S, SW, W, NW
    let directions = [
        (-1, 0), (-1, 1), (0, 1), (1, 1),
        (1, 0), (1, -1), (0, -1), (-1, -1)
    ];
    
    for (di, dj) in directions {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;
        
        if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
            let ni = ni as usize;
            let nj = nj as usize;
            adjacent.push((ni, nj, grid[ni][nj]));
        }
    }
    
    adjacent
}

fn show_grid(grid: &Vec<Vec<char>>){ 
    for line in grid {
        for col in line {
            print!("{col}")
        }
        println!();
    }
}

fn search_grid(grid: &mut Vec<Vec<char>>) -> (i32, usize) {
    let mut valid_rolls = 0;
    let mut valid_cells: Vec<(usize, usize)> = Vec::new();
    for index in 0..grid.len() {
        for cell in 0..grid.len() {
            if grid[index][cell] != '@' {continue;}
            let adjacents = get_adjacent(index,cell, &grid);
            let count_adjacents = count_adjacents(adjacents);
            let valid = placement_valid(count_adjacents.try_into().unwrap());
            //println!("Looking at [{0},{1}] {2} has {3} adjacents, valid? {4}",index, cell, grid[index][cell], count_adjacents, valid);
            if valid {
                valid_rolls += 1;
                valid_cells.push((index, cell));
            }
        }
    }
    let count = valid_cells.len();
    remove_valid_rolls(valid_cells, grid);
    (valid_rolls, count)
}

fn remove_valid_rolls(valid_cells: Vec<(usize, usize)>, grid: &mut Vec<Vec<char>>) {
    for valid_cell in valid_cells {
        print!("Removing {:?}", valid_cell);
        grid[valid_cell.0][valid_cell.1] = '.'
    }
    println!();
}

fn solve_part_1_and_2(contents: Vec<String>) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in contents {
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }
    show_grid(&grid);

    let mut total_valid_rolls = 0;
    loop {
        let (valid_rolls, removals) = search_grid(&mut grid);
        if total_valid_rolls == 0 {
            println!("Part 1 result {valid_rolls}");
        }
        if removals == 0 {
            println!("Found no move cells to remove");
            break;
        }
        println!("Found valid rolls: {valid_rolls}");
        total_valid_rolls += valid_rolls;
    }
    println!("Total valid rolls: {total_valid_rolls}");
    
}

pub fn day_4_solve() {
    let contents =  read_contents();
    solve_part_1_and_2(contents);
}