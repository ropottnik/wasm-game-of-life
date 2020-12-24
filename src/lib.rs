mod utils;

use bit_vec::BitVec;
use parsers::parsers::parse_rle_string;
use wasm_bindgen::prelude::*;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate web_sys;

// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     };
// }

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: BitVec,
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
    pub fn get_cells(&self) -> &BitVec {
        &self.cells
    }

    pub fn set_cells(&mut self, cells_alive: Vec<(u32, u32)>) {
        for (x, y) in cells_alive {
            let x_scaled = x % self.width;
            let y_scaled = y % self.height;
            let idx = self.get_index(x_scaled, y_scaled);
            self.cells.set(idx, true);
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
                if self.cells.get(idx).unwrap() {
                    count += 1;
                }
            }
        }
        count
    }
}

/// public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn new(height: u32, width: u32) -> Universe {
        utils::set_panic_hook();
        Universe {
            height,
            width,
            cells: BitVec::from_elem((width * height) as usize, false),
        }
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = BitVec::from_elem((width * self.height) as usize, false);
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = BitVec::from_elem((self.width * height) as usize, false);
    }

    pub fn set_rle_shape(&mut self, rle_string: &str, x: u32, y: u32) {
        let mut shape = Shape::from_rle_string(rle_string);
        shape.shift((x, y));
        self.set_cells(shape.alive_cells);
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells.get(idx).unwrap();
                let live_neighbor_count = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbor_count) {
                    // Rule 1: Any live cell with fewer than two live neighbors
                    // dies, as if caused by underpopulation.
                    (true, x) if x < 2 => false,
                    // Rule 2: Any live cell with two or three live neighbors
                    // lives on to the next generation.
                    (true, 2) | (true, 3) => true,
                    // Rule 3: Any live cell with more than three live
                    // neighbors dies, as if from overpopulation.
                    (true, x) if x > 3 => false,
                    // Rule 4: Any dead cell with exactly three live neighbors
                    // becomes a live cell, as if by reproduction.
                    (false, 3) => true,
                    // all other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next.set(idx, next_cell);
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
        let mut carriage = 0;
        for cell in self.cells.iter() {

            let symbol = if cell { '◼' } else { ' ' };
            write!(f, "{}", symbol)?;
            carriage = carriage + 1;
            if carriage == self.width - 1 {
                write!(f, "\n")?;
                carriage = 0;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{ Shape, Universe};

    #[test]
    fn blank_universe_works() {
        let u = Universe::new(4, 4);
        for c in u.cells.iter() {
            assert_eq!(c, false);
        }
    }

    #[test]
    fn universe_with_absolute_pattern_works() {
        let mut u = Universe::new(2, 2);
        u.set_cells(vec![(0, 0), (1, 1)]);
        assert_eq!(u.cells[0], true);
        assert_eq!(u.cells[1], false);
        assert_eq!(u.cells[2], false);
        assert_eq!(u.cells[3], true);
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
