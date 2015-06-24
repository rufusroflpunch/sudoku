# Sudoku Solver (Rust)

This was a Rust learning excercise I undertook. I wrote a Sudoku solver in rust. This was both my first rust application (of note) and my first try at a sudoku solver.

## Installation

Clone the git repo, then run:

```
cargo build --release
```

## Usage

Input a sudoku puzzle into a file, like this:

```
4_8 7_1 __2
_5_ 4_2 __6
__9 ___ 14_

__4 5__ __7
_1_ 3_7 _8_
3__ __8 5__

_85 ___ 2__
7__ 2_9 _3_
9__ 8_4 7_5
```

All characters that aren't 1-9 and _ are ignored. Run the application like this: 
`target/release/sudoku sudoku.txt` and it will output the solution.

## Algorithm

1. Count pre-solved cells.
2. Begin at cell 0.
3. If solved counter == 81, print solution and exit.
4. If cell is empty, check for possible solutions.
5. If there is only one solution, insert into puzzle, increment solved counter and go to step 2.
6. If there is more than solutions, increment cell and go to step 3.
