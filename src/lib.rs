mod utils;

use bitvec::prelude::*;
use std::fmt;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    num_rows: u32,
    num_cols: u32,
    cells: BitVec,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        let num_rows = 64;
        let num_cols = 64;
        let cells = bitvec![0; (num_rows * num_cols) as usize];

        Self {
            num_rows,
            num_cols,
            cells,
        }
    }

    pub fn interesting() -> Self {
        let mut this = Self::new();

        for r in 0..this.num_rows {
            for c in 0..this.num_cols {
                let idx = this.get_index(r, c);
                if idx % 2 == 0 || idx % 7 == 0 {
                    this.cells.set(idx, true);
                }
            }
        }

        this
    }

    pub fn with_glider() -> Self {
        let mut this = Self::new();

        let (r, c) = (this.num_rows / 2, this.num_cols / 2);
        let glider = [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
        for (dr, dc) in glider {
            let idx = this.get_index(r + dr, c + dc);
            this.cells.set(idx, true);
        }

        this
    }

    pub fn random() -> Self {
        let mut this = Self::new();

        for r in 0..this.num_rows {
            for c in 0..this.num_cols {
                let idx = this.get_index(r, c);
                if js_sys::Math::random() >= 0.5 {
                    this.cells.set(idx, true);
                }
            }
        }

        this
    }

    pub fn num_rows(&self) -> u32 {
        self.num_rows
    }

    pub fn num_cols(&self) -> u32 {
        self.num_cols
    }

    pub fn cells(&self) -> *const usize {
        self.cells.as_raw_slice().as_ptr()
    }

    pub fn tick(&mut self) {
        let mut new_cells = self.cells.clone();

        for r in 0..self.num_rows {
            for c in 0..self.num_cols {
                let idx = self.get_index(r, c);
                let cell = if self.cells[idx] {
                    Cell::Alive
                } else {
                    Cell::Dead
                };
                let live_nbrs = self.live_neighbor_count(r, c);

                let new_cell = match (cell, live_nbrs) {
                    // just right
                    (Cell::Alive, 2 | 3) => Cell::Alive,
                    // under-/over-population
                    (Cell::Alive, _) => Cell::Dead,
                    // reproduction
                    (Cell::Dead, 3) => Cell::Alive,
                    _ => cell,
                };

                new_cells.set(idx, new_cell == Cell::Alive);
            }
        }

        self.cells = new_cells;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.num_cols + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for dr in [self.num_rows - 1, 0, 1] {
            for dc in [self.num_cols - 1, 0, 1] {
                if (dr, dc) == (0, 0) {
                    continue;
                }

                let r2 = (row + dr) % self.num_rows;
                let c2 = (col + dc) % self.num_cols;
                let idx = self.get_index(r2, c2);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_bitslice().chunks(self.num_cols as usize) {
            for cell in line {
                let c = if *cell { '◼' } else { '◻' };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
