use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::{colors::symbol_style, ruleset::HeadDirection};

#[derive(Debug)]
pub struct FlexibleMemory2D {
    num_rows: u16,
    num_cols: u16,
    data: Vec<Vec<u8>>,
    head: (u16, u16),
}

impl FlexibleMemory2D {
    pub fn from_dimensions(rows: u16, columns: u16) -> Self {
        FlexibleMemory2D {
            num_rows: rows,
            num_cols: columns,
            data: vec![vec![0; columns as usize]; rows as usize],
            head: (0, 0),
        }
    }

    pub fn resize(&mut self, new_rows: u16, new_columns: u16) {
        if new_rows != self.num_rows {
            let last_row = self.data.last().unwrap().clone();

            self.data
                .resize_with(new_rows as usize, || last_row.clone());
        }

        if new_columns != self.num_cols {
            self.data.iter_mut().for_each(|row| {
                let last_element = *row.last().unwrap();
                row.resize_with(new_columns as usize, || last_element);
            })
        }

        self.num_rows = new_rows;
        self.num_cols = new_columns;

        self.head = (self.head.0 % self.num_rows, self.head.1 % self.num_cols);
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
                self.head.1 %= self.num_cols;
            }
            HeadDirection::Bottom => {
                self.head.0 += 1;
                self.head.0 %= self.num_rows;
            }
            HeadDirection::Left => {
                self.head.1 += self.num_cols - 1;
                self.head.1 %= self.num_cols;
            }
            HeadDirection::Top => {
                self.head.0 += self.num_rows - 1;
                self.head.0 %= self.num_rows;
            }
        }
    }

    pub fn reset(&mut self) {
        self.data = vec![vec![0u8; self.num_cols as usize]; self.num_rows as usize];
    }

    pub fn _get(&self) -> &[Vec<u8>] {
        &self.data
    }
}

impl Widget for &FlexibleMemory2D {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::vertical(
            (0..self.num_rows)
                .map(|_| Constraint::Length(1))
                .collect::<Vec<Constraint>>(),
        )
        .split(area);

        layout
            .iter()
            .zip(self.data.iter())
            .for_each(|(&row_layout, row)| {
                let text_row = row
                    .iter()
                    .map(|el| symbol_style(*el))
                    .collect::<Vec<Span>>();

                Paragraph::new(Line::from(text_row)).render(row_layout, buf);
            });
    }
}
