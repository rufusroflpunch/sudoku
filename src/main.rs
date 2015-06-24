use std::env::args;
use std::fs::File;
use std::io::Read;
use std::vec::*;

// The starting cell for each row.
static ROWS: [i32; 9] = [ 0, 9, 18, 27, 36, 45, 54, 63, 72 ];

// The section number for each cell.
static SECS: [i32; 81] = [
    0, 0, 0, 1, 1, 1, 2, 2, 2, 
    0, 0, 0, 1, 1, 1, 2, 2, 2, 
    0, 0, 0, 1, 1, 1, 2, 2, 2, 
    3, 3, 3, 4, 4, 4, 5, 5, 5,
    3, 3, 3, 4, 4, 4, 5, 5, 5,
    3, 3, 3, 4, 4, 4, 5, 5, 5,
    6, 6, 6, 7, 7, 7, 8, 8, 8,
    6, 6, 6, 7, 7, 7, 8, 8, 8,
    6, 6, 6, 7, 7, 7, 8, 8, 8
];

static Y_VALS: [i32; 81] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 
    1, 1, 1, 1, 1, 1, 1, 1, 1,
    2, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3, 3,
    4, 4, 4, 4, 4, 4, 4, 4, 4,
    5, 5, 5, 5, 5, 5, 5, 5, 5,
    6, 6, 6, 6, 6, 6, 6, 6, 6,
    7, 7, 7, 7, 7, 7, 7, 7, 7,
    8, 8, 8, 8, 8, 8, 8, 8, 8
];

static X_VALS: [i32; 81] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8
];

fn main() {

    if args().count() <= 1 {
        println!("Error: Please enter filename containing Sudoku puzzle to solve.");
        return;
    }


    // Open file and read contents
    let mut f = File::open(
        args().nth(1).expect("Missing argument. Please enter a filename.")
        )
        .ok().expect("Unable to open file.");
    let mut contents = String::new();
    if let Err(e) = f.read_to_string(&mut contents) {
        println!("Error: {}", e);
    }
    

    // Make sure the file is the right size
    if contents.chars().count() < 81 {
        println!("File is too small for contain a puzzle.");
    }
    

    // Create puzzle in memory
    let mut puz = get_puzzle(&contents);
    let mut puz: Vec<_> = puz.iter().filter(|&x| *x < 10).map(|&x| x).collect();


    // Solve it!
    let mut i: usize = 0;
    let mut buf: Vec<i32> = Vec::new();
    let mut solved = puz.iter().filter(|&x| *x > 0).count(); // How many come solved?
    let mut total_loops = 0;

    while solved < 81 {
        // Stop the puzzle if it can't be solved by the algorithm
        total_loops = total_loops + 1;
        if total_loops > 1000 { println!("Can't solve puzzle."); break; }

        // If we have reached the end, loop back
        if i >= 81 { i = 0; }

        // Clear the buffer from the previous run
        buf.clear();

        if puz[i] == 0 {
            // Get every option that the cell could be, put it in a buffer
            for y in 1..10 {
                if check_cell(&puz, i, y) { buf.push(y); }
            }


            // If there is only one possible option for that cell, then put it in the puzzle
            if buf.len() == 1 {
                puz[i] = buf[0];
                solved = solved + 1;
            }
        }

        i = i + 1; // If all else fails, move on
    }


    // Output the puzzle
    print_puz(&puz);
}

// Take an input string of characters and turn it into a puzzle vector.
fn get_puzzle(puz_str: &str) -> Vec<i32> {
    let mut puz: Vec<i32> = Vec::new();

    for c in puz_str.chars() {
        puz.push(match c {
            '1'       => 1,
            '2'       => 2,
            '3'       => 3,
            '4'       => 4,
            '5'       => 5,
            '6'       => 6,
            '7'       => 7,
            '8'       => 8,
            '9'       => 9,
            '_'       => 0,
            _         => 10,
        });
    }

    puz
}


// Get the number from a cell in the puzzle vector
fn get_cell(v: &Vec<i32>, x: i32, y: i32) -> i32 {
    let yu = y as usize;
    let xu = x as usize;
    v[yu*9+xu]
}


// Return the coords for a given index
fn get_coords(idx: usize) -> (i32, i32) {
    (X_VALS[idx], Y_VALS[idx])
}


// Check row for number. Return false if number is found.
fn check_row(v: &Vec<i32>, row: i32, value: i32) -> bool {
    let start = ROWS[row as usize] as usize;
    for x in start..start+9 {
        if v[x] == value { return false; }
    }

    true
}


// Check column for number. Return false if number is found.
fn check_col(v: &Vec<i32>, col: i32, value: i32) -> bool {
    for y in 0..9 {
        if get_cell(v, col, y) == value { return false; }
    }

    true
}


// Check section for number. Return false if number is found.
fn check_sec(v: &Vec<i32>, sec: i32, value: i32) -> bool {
    for i in 0..81 {
        if SECS[i as usize] == sec && v[i as usize] == value { return false; }
    }

    true
}


// Print the final table
fn print_puz(v: &Vec<i32>) {
    for y in 0..9 {
        if y % 3 == 0 { println!(""); }
        for x in 0..9 {
           if x % 3 == 0 { print!(" "); }
           match get_cell(&v, x, y){
                e @ 1...9 => print!("{}", e),
                0 => print!("_"),
                _ => unreachable!()
           }
        }
        println!("");
    }
}


// Check row, column and section to make sure the value is valid
fn check_cell(puz: &Vec<i32>, idx: usize, val: i32) -> bool {
    let (x, y) = get_coords(idx);

    check_row(&puz, y, val) && check_col(&puz, x, val) && check_sec(&puz, SECS[idx], val)
}
