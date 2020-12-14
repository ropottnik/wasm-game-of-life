mod utils;

use parsers::parsers::{parse_rle_string, RleSymbol};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shape {
    pub alive_cells: Vec<(u32, u32)>,
}

impl Shape {
    pub fn from_rle_string(rle_string: &str) -> Self {
        let mut offset = (0, 0);
        let mut alive_cells = vec![];

        for atom in parse_rle_string(rle_string).unwrap().1 {
            atom.grow_pattern(&mut offset, &mut alive_cells)
        }

        Self { alive_cells }
    }

    pub fn shift(&mut self, shift: (u32, u32)) {
        for cell in &mut self.alive_cells {
            cell.0 = cell.0 + shift.0;
            cell.1 = cell.1 + shift.1;
        }
    }
}

impl Universe {
    pub fn blank(height: u32, width: u32) -> Universe {
        Universe {
            height,
            width,
            cells: vec![Cell::Dead; (height * width) as usize],
        }
    }

    pub fn set_absolute_pattern(&mut self, cells_alive: Vec<(u32, u32)>) {
        for (x, y) in cells_alive {
            let x_scaled = x % self.width;
            let y_scaled = y % self.height;
            let idx = self.get_index(x_scaled, y_scaled);
            self.cells[idx] = Cell::Alive;
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

/// public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn blank_pub() -> Universe {
        Universe::blank(150, 200)
    }

    pub fn set_rle_shape(&mut self, rle_string: &str) {
        self.set_absolute_pattern(Shape::from_rle_string(rle_string).alive_cells);
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbor_count = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbor_count) {
                    // Rule 1: Any live cell with fewer than two live neighbors
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbors
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbors dies, as if from overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbors
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // all other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { ' ' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cell, Shape, Universe};

    #[test]
    fn blank_universe_works() {
        let u = Universe::blank(4, 4);
        for c in u.cells {
            assert_eq!(c, Cell::Dead);
        }
    }

    #[test]
    fn universe_with_absolute_pattern_works() {
        let mut u = Universe::blank(2, 2);
        u.set_absolute_pattern(vec![(0, 0), (1, 1)]);
        assert_eq!(u.cells[0], Cell::Alive);
        assert_eq!(u.cells[1], Cell::Dead);
        assert_eq!(u.cells[2], Cell::Dead);
        assert_eq!(u.cells[3], Cell::Alive);
    }

    #[test]
    fn shifting_patterns_works() {
        let mut pattern = Shape {
            alive_cells: vec![(0, 0), (1, 1), (13, 1)],
        };
        pattern.shift((1, 2));
        assert_eq!(pattern.alive_cells, vec![(1, 2), (2, 3), (14, 3)]);
    }

    #[test]
    fn parsing_pattern_works() {
        assert_eq!(
            Shape::from_rle_string("2bobo23$4b2o"),
            Shape {
                alive_cells: vec![(2, 0), (4, 0), (4, 23), (5, 23)],
            }
        );
    }
}

pub(self) mod parsers;
