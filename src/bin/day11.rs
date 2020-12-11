use aoc2020_niels_overkamp::common::{self, AOCResult};
use std::alloc::handle_alloc_error;
use core::mem;
use std::io::{stdout, Write};

const DAY: &str = "day11";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    if input.len() == 0 {
        return Err("Input empty".into());
    }

    let mut board: Vec<Vec<Option<bool>>> = vec![];
    let width = input[0].len();
    let height = input.len();
    for line in input {
        let mut row = vec![];
        for c in line.chars() {
            let res = match c {
                '.' => Ok(row.push(None)),
                'L' => Ok(row.push(Some(false))),
                '#' => Ok(row.push(Some(true))),
                _   => Err(format!("Unknown character {}", c))
            };
            if let Err(e) = res {
                return Err(e.into());
            }

        }
        board.push(row);
    }
    let neighbours = [(0,1), (1,1), (1,0),(1,-1), (0,-1),(-1,-1), (-1,0), (-1,1)];

    let mut look_board = board.clone();
    let mut board_empty = board.clone();
    let mut look_board_empty = board.clone();
    let mut has_changed: bool = true;
    let mut look_has_changed: bool = true;
    let mut iter_count = 0;
    while has_changed || look_has_changed {
        iter_count+=1;
        // println!("board");
        // print_board(&board);
        // println!("look board");
        // print_board(&look_board);
        has_changed = false;
        look_has_changed = false;
        for (y, row) in (0..height).zip(&board) {
            let empty_row = &mut board_empty[y];
            let empty_look_row = &mut look_board_empty[y];
            for (x, cell) in (0..width).zip(row) {
                match cell {
                    None => (),
                    Some(occupied) => {
                        let mut count = 0;
                        let mut look_count = 0;
                        let look_occupied = look_board[y][x].unwrap();
                        for (dy, dx) in neighbours.iter() {
                            {
                                if look(&look_board, x as i32, y as i32, *dx, *dy, width as i32, height as i32) {
                                    look_count += 1;
                                }
                                let y = ((y as i32)+*dy);
                                if y < 0 || y as usize >= height {continue};
                                let x = ((x as i32) + *dx);
                                if x < 0 || x as usize >= width {continue};
                                if board[y as usize][x as usize].unwrap_or(false) {
                                    count += 1;
                                }
                            }
                        }
                        // print!("board: {} {}", x, y);
                        if *occupied && count >= 4 {
                            // println!("Too many people");
                            empty_row[x] = Some(false);
                            has_changed = true;
                        } else if !*occupied && count == 0 {
                            // println!("Ahhh freedom");
                            empty_row[x] = Some(true);
                            has_changed = true;
                        } else {
                            // println!("No change, still: {}", occupied);
                            empty_row[x] = Some(*occupied);
                        }
                        // print!("look board: {} {}", x, y);
                        if look_occupied && look_count >= 5 {
                            // println!("Too many people");
                            empty_look_row[x] = Some(false);
                            look_has_changed = true;
                        } else if !look_occupied && look_count == 0 {
                            // println!("Ahhh freedom");
                            empty_look_row[x] = Some(true);
                            look_has_changed = true;
                        } else {
                            // println!("No change, still: {}", look_occupied);
                            empty_look_row[x] = Some(look_occupied);
                        }
                    }
                }
            }
            // println!("board {}", y);
            // print_board(&board_empty);
            // println!("look board {}", y);
            // print_board(&look_board_empty);
        }
        let temp = board;
        board = board_empty;
        board_empty = temp;

        let temp = look_board;
        look_board = look_board_empty;
        look_board_empty = temp;
    }

    let mut count = 0;
    for row in &board {
        for cell in row {
            if let Some(true) = cell {
                count += 1;
            }
        }
    }


    let mut look_count = 0;
    for row in &look_board {
        for cell in row {
            if let Some(true) = cell {
                look_count += 1;
            }
        }
    }

    return Ok([Some(count.to_string()), Some(look_count.to_string())]);
}

fn look(board: &Vec<Vec<Option<bool>>>, x: i32, y: i32, dx: i32, dy: i32, max_x: i32, max_y: i32) -> bool {
    let mut x = x + dx;
    let mut y = y + dy;
    while x < max_x && x >= 0 && y < max_y && y >= 0 {
        if let Some(occupied) = board[y as usize][x as usize] {
            return occupied;
        }
        x = x + dx;
        y = y + dy;
    }
    return false;
}

fn print_board(board: &Vec<Vec<Option<bool>>>) {
    board.iter().for_each(|r| {
        let s: String = r.iter().map(|c| match c {Some(true) => '#', Some(false) => 'L', None => '.', }).collect();
        println!("{}",s);
    });
    println!();
}

#[test]
pub fn test_day11() {
    assert!(common::run_test(DAY, &run))
}