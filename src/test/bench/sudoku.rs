// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-pretty very bad with line comments

#![feature(box_syntax)]
#![allow(non_snake_case)]

use std::io::BufferedReader;
use std::io::stdio::StdReader;
use std::io;
use std::iter::repeat;
use std::num::Int;
use std::os;

// Computes a single solution to a given 9x9 sudoku
//
// Call with "-" to read input sudoku from stdin
//
// The expected line-based format is:
//
// 9,9
// <row>,<column>,<color>
// ...
//
// Row and column are 0-based (i.e. <= 8) and color is 1-based (>=1,<=9).
// A color of 0 indicates an empty field.
//
// If called without arguments, sudoku solves a built-in example sudoku
//

// internal type of sudoku grids
type grid = Vec<Vec<u8> > ;

struct Sudoku {
    grid: grid
}

impl Sudoku {
    pub fn new(g: grid) -> Sudoku {
        return Sudoku { grid: g }
    }

    pub fn from_vec(vec: &[[u8;9];9]) -> Sudoku {
        let g = range(0, 9u).map(|i| {
            range(0, 9u).map(|j| { vec[i][j] }).collect()
        }).collect();
        return Sudoku::new(g)
    }

    pub fn read(mut reader: &mut BufferedReader<StdReader>) -> Sudoku {
        /* assert first line is exactly "9,9" */
        assert!(reader.read_line().unwrap() == "9,9".to_string());

        let mut g = repeat(vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8])
                          .take(10).collect::<Vec<_>>();
        for line in reader.lines() {
            let line = line.unwrap();
            let comps: Vec<&str> = line.as_slice()
                                       .trim()
                                       .split(',')
                                       .collect();

            if comps.len() == 3u {
                let row = comps[0].parse::<u8>().unwrap();
                let col = comps[1].parse::<u8>().unwrap();
                g[row as uint][col as uint] = comps[2].parse().unwrap();
            }
            else {
                panic!("Invalid sudoku file");
            }
        }
        return Sudoku::new(g)
    }

    pub fn write(&self, writer: &mut io::Writer) {
        for row in range(0u8, 9u8) {
            write!(writer, "{}", self.grid[row as uint][0]);
            for col in range(1u8, 9u8) {
                write!(writer, " {}", self.grid[row as uint][col as uint]);
            }
            write!(writer, "\n");
         }
    }

    // solve sudoku grid
    pub fn solve(&mut self) {
        let mut work: Vec<(u8, u8)> = Vec::new(); /* queue of uncolored fields */
        for row in range(0u8, 9u8) {
            for col in range(0u8, 9u8) {
                let color = self.grid[row as uint][col as uint];
                if color == 0u8 {
                    work.push((row, col));
                }
            }
        }

        let mut ptr = 0u;
        let end = work.len();
        while ptr < end {
            let (row, col) = work[ptr];
            // is there another color to try?
            let the_color = self.grid[row as uint][col as uint] +
                                (1 as u8);
            if self.next_color(row, col, the_color) {
                //  yes: advance work list
                ptr = ptr + 1u;
            } else {
                // no: redo this field aft recoloring pred; unless there is none
                if ptr == 0u { panic!("No solution found for this sudoku"); }
                ptr = ptr - 1u;
            }
        }
    }

    fn next_color(&mut self, row: u8, col: u8, start_color: u8) -> bool {
        if start_color < 10u8 {
            // colors not yet used
            let mut avail = box Colors::new(start_color);

            // drop colors already in use in neighbourhood
            self.drop_colors(&mut *avail, row, col);

            // find first remaining color that is available
            let next = avail.next();
            self.grid[row as uint][col as uint] = next;
            return 0u8 != next;
        }
        self.grid[row as uint][col as uint] = 0u8;
        return false;
    }

    // find colors available in neighbourhood of (row, col)
    fn drop_colors(&mut self, avail: &mut Colors, row: u8, col: u8) {
        for idx in range(0u8, 9u8) {
            /* check same column fields */
            avail.remove(self.grid[idx as uint][col as uint]);
            /* check same row fields */
            avail.remove(self.grid[row as uint][idx as uint]);
        }

        // check same block fields
        let row0 = (row / 3u8) * 3u8;
        let col0 = (col / 3u8) * 3u8;
        for alt_row in range(row0, row0 + 3u8) {
            for alt_col in range(col0, col0 + 3u8) {
                avail.remove(self.grid[alt_row as uint][alt_col as uint]);
            }
        }
    }
}

// Stores available colors as simple bitfield, bit 0 is always unset
struct Colors(u16);

static HEADS: u16 = (1u16 << 10) - 1; /* bits 9..0 */

impl Colors {
    fn new(start_color: u8) -> Colors {
        // Sets bits 9..start_color
        let tails = !0u16 << start_color as uint;
        return Colors(HEADS & tails);
    }

    fn next(&self) -> u8 {
        let Colors(c) = *self;
        let val = c & HEADS;
        if 0u16 == val {
            return 0u8;
        } else {
            return val.trailing_zeros() as u8
        }
    }

    fn remove(&mut self, color: u8) {
        if color != 0u8 {
            let Colors(val) = *self;
            let mask = !(1u16 << color as uint);
            *self    = Colors(val & mask);
        }
    }
}

static DEFAULT_SUDOKU: [[u8;9];9] = [
         /* 0    1    2    3    4    5    6    7    8    */
  /* 0 */  [0u8, 4u8, 0u8, 6u8, 0u8, 0u8, 0u8, 3u8, 2u8],
  /* 1 */  [0u8, 0u8, 8u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8],
  /* 2 */  [7u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8],
  /* 3 */  [0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8],
  /* 4 */  [0u8, 5u8, 0u8, 0u8, 0u8, 3u8, 6u8, 0u8, 0u8],
  /* 5 */  [6u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8],
  /* 6 */  [0u8, 9u8, 5u8, 0u8, 0u8, 6u8, 0u8, 7u8, 0u8],
  /* 7 */  [0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 6u8, 0u8],
  /* 8 */  [4u8, 0u8, 0u8, 0u8, 0u8, 7u8, 2u8, 0u8, 3u8]
];

#[cfg(test)]
static DEFAULT_SOLUTION: [[u8;9];9] = [
         /* 0    1    2    3    4    5    6    7    8    */
  /* 0 */  [1u8, 4u8, 9u8, 6u8, 7u8, 5u8, 8u8, 3u8, 2u8],
  /* 1 */  [5u8, 3u8, 8u8, 1u8, 2u8, 9u8, 7u8, 4u8, 6u8],
  /* 2 */  [7u8, 2u8, 6u8, 8u8, 3u8, 4u8, 1u8, 5u8, 9u8],
  /* 3 */  [9u8, 1u8, 4u8, 5u8, 6u8, 8u8, 3u8, 2u8, 7u8],
  /* 4 */  [2u8, 5u8, 7u8, 4u8, 9u8, 3u8, 6u8, 1u8, 8u8],
  /* 5 */  [6u8, 8u8, 3u8, 7u8, 1u8, 2u8, 5u8, 9u8, 4u8],
  /* 6 */  [3u8, 9u8, 5u8, 2u8, 8u8, 6u8, 4u8, 7u8, 1u8],
  /* 7 */  [8u8, 7u8, 2u8, 3u8, 4u8, 1u8, 9u8, 6u8, 5u8],
  /* 8 */  [4u8, 6u8, 1u8, 9u8, 5u8, 7u8, 2u8, 8u8, 3u8]
];

#[test]
fn colors_new_works() {
    assert_eq!(*Colors::new(1), 1022u16);
    assert_eq!(*Colors::new(2), 1020u16);
    assert_eq!(*Colors::new(3), 1016u16);
    assert_eq!(*Colors::new(4), 1008u16);
    assert_eq!(*Colors::new(5), 992u16);
    assert_eq!(*Colors::new(6), 960u16);
    assert_eq!(*Colors::new(7), 896u16);
    assert_eq!(*Colors::new(8), 768u16);
    assert_eq!(*Colors::new(9), 512u16);
}

#[test]
fn colors_next_works() {
    assert_eq!(Colors(0).next(), 0u8);
    assert_eq!(Colors(2).next(), 1u8);
    assert_eq!(Colors(4).next(), 2u8);
    assert_eq!(Colors(8).next(), 3u8);
    assert_eq!(Colors(16).next(), 4u8);
    assert_eq!(Colors(32).next(), 5u8);
    assert_eq!(Colors(64).next(), 6u8);
    assert_eq!(Colors(128).next(), 7u8);
    assert_eq!(Colors(256).next(), 8u8);
    assert_eq!(Colors(512).next(), 9u8);
    assert_eq!(Colors(1024).next(), 0u8);
}

#[test]
fn colors_remove_works() {
    // GIVEN
    let mut colors = Colors::new(1);

    // WHEN
    colors.remove(1);

    // THEN
    assert_eq!(colors.next(), 2u8);
}

#[test]
fn check_DEFAULT_SUDOKU_solution() {
    // GIVEN
    let mut sudoku = Sudoku::from_vec(&DEFAULT_SUDOKU);
    let solution   = Sudoku::from_vec(&DEFAULT_SOLUTION);

    // WHEN
    sudoku.solve();

    // THEN
    assert!(sudoku.equal(&solution));
}

fn main() {
    let args        = os::args();
    let use_default = args.len() == 1u;
    let mut sudoku = if use_default {
        Sudoku::from_vec(&DEFAULT_SUDOKU)
    } else {
        Sudoku::read(&mut *io::stdin().lock())
    };
    sudoku.solve();
    sudoku.write(&mut io::stdout());
}
