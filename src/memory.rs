use crate::ruleset::HeadDirection;
use std::fmt::Display;

#[derive(Debug)]
pub struct Memory2D<const ROWS: usize, const COLS: usize> {
    data: [[u8; COLS]; ROWS],
    head: (u8, u8),
}

impl<const ROWS: usize, const COLS: usize> Default for Memory2D<ROWS, COLS> {
    fn default() -> Self {
        Memory2D {
            data: [[0u8; COLS]; ROWS],
            head: (0, 0),
        }
    }
}

impl<const ROWS: usize, const COLS: usize> Memory2D<ROWS, COLS> {
    pub fn new() -> Self {
        Memory2D {
            data: [[0; COLS]; ROWS],
            head: (0, 0),
        }
    }

    pub fn read(&mut self) -> u8 {
        let (row_index, column_index) = self.head;
        self.data[row_index as usize][column_index as usize]
    }

    pub fn write(&mut self, value: u8) {
        self.data[self.head.0 as usize][self.head.1 as usize] = value;
    }

    pub fn move_head(&mut self, direction: &HeadDirection) {
        match direction {
            HeadDirection::Right => {
                self.head.1 += 1;
                self.head.1 %= COLS as u8;
            }
            HeadDirection::Bottom => {
                self.head.0 += 1;
                self.head.0 %= ROWS as u8;
            }
            HeadDirection::Left => {
                self.head.1 += COLS as u8 - 1;
                self.head.1 %= COLS as u8;
            }
            HeadDirection::Top => {
                self.head.0 += ROWS as u8 - 1;
                self.head.0 %= ROWS as u8;
            }
        }
    }

    pub fn reset(&mut self) {
        self.data = [[0u8; COLS]; ROWS];
    }

    pub fn _get(&self) -> &[[u8; COLS]; ROWS] {
        &self.data
    }
}

// pub struct Memory2D_dynamic {
//     num_rows: u16,
//     num_cols: u16,
//     data: Vec<Vec<u8>>,
//     head: (u16, u16),
// }

// impl Memory2D_dynamic {
//     pub fn from_dimensions(rows: u16, columns: u16) -> Self {
//         Memory2D_dynamic {
//             num_rows: rows,
//             num_cols: columns,
//             data: vec![vec![0; columns as usize]; rows as usize],
//             head: (0, 0),
//         }
//     }
//     pub fn resize(&mut self, new_rows: u16, new_columns: u16) {
//         if new_rows != self.num_rows {
//             let mut last_row = self.data.last().unwrap().clone();

//             if new_columns != self.num_cols {
//                 let last_element_of_last_row = last_row.last().unwrap().clone();
//                 last_row.resize_with(new_columns as usize, || last_element_of_last_row.clone());
//                 self.data.iter_mut().for_each(|row| {
//                     let last_element = row.last().unwrap().clone();
//                     row.resize_with(new_columns as usize, || last_element.clone());
//                 })
//             }

//             self.data
//                 .resize_with(new_rows as usize, || last_row.clone());
//         }
//     }

//     pub fn read(&mut self) -> u8 {
//         let (row_index, column_index) = self.head;
//         self.data[row_index as usize][column_index as usize]
//     }

//     pub fn write(&mut self, value: u8) {
//         self.data[self.head.0 as usize][self.head.1 as usize] = value;
//     }

//     pub fn move_head(&mut self, direction: &HeadDirection) {
//         match direction {
//             HeadDirection::Right => {
//                 self.head.1 += 1;
//                 self.head.1 %= self.num_cols;
//             }
//             HeadDirection::Bottom => {
//                 self.head.0 += 1;
//                 self.head.0 %= self.num_rows;
//             }
//             HeadDirection::Left => {
//                 self.head.1 += self.num_cols - 1;
//                 self.head.1 %= self.num_cols;
//             }
//             HeadDirection::Top => {
//                 self.head.0 += self.num_rows - 1;
//                 self.head.0 %= self.num_rows;
//             }
//         }
//     }

//     pub fn reset(&mut self) {
//         self.data = vec![vec![0u8; self.num_cols as usize]; self.num_rows as usize];
//     }

//     pub fn _get(&self) -> &[Vec<u8>] {
//         &self.data
//     }
// }
