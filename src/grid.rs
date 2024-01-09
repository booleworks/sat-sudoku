use core::fmt;

pub struct Grid {
    number_rows: u8,
    number_cols: u8,
    box_size: u8,
    data: Vec<Vec<u8>>,
}

impl Grid {
    pub fn empty_9x9_grid() -> Grid {
        Grid {
            number_rows: 9,
            number_cols: 9,
            box_size: 3,
            data: vec![vec![0; 9]; 9],
        }
    }

    pub fn set_value(&mut self, row: u8, col: u8, value: u8) {
        self.data[row as usize - 1][col as usize - 1] = value;
    }

    pub fn set_row(&mut self, row: u8, col: &str) {
        for (col, value) in col.split(' ').enumerate() {
            if value != "-" {
                self.set_value(row, col as u8 + 1, value.parse().unwrap());
            }
        }
    }

    pub fn get_filled_cells(&self) -> Vec<(u8, u8, u8)> {
        let mut filled = Vec::new();
        for row in 1..=self.number_rows {
            for col in 1..=self.number_cols {
                let value = self.data[row as usize - 1][col as usize - 1];
                if value != 0 {
                    filled.push((row, col, value));
                }
            }
        }
        filled
    }

    pub fn rows(&self) -> Vec<Row> {
        (1..=self.number_rows)
            .map(|row| Row {
                row,
                number_cols: self.number_cols,
            })
            .collect()
    }

    pub fn cols(&self) -> Vec<Column> {
        (1..=self.number_cols)
            .map(|col| Column {
                col,
                number_rows: self.number_rows,
            })
            .collect()
    }

    pub fn boxes(&self) -> Vec<GridBox> {
        let mut res = Vec::new();
        for row in 0..(self.number_rows / self.box_size) {
            for col in 0..(self.number_cols / self.box_size) {
                res.push(GridBox {
                    row: (row * self.box_size) + 1,
                    col: (col * self.box_size) + 1,
                    box_size: self.box_size,
                });
            }
        }
        res
    }

    pub fn possible_values(&self) -> Vec<u8> {
        (1..=(self.box_size * self.box_size)).collect()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str("┏━━━━━━━┯━━━━━━━┯━━━━━━━┓\n");
        for (row_num, row) in self.data.iter().enumerate() {
            if row_num != 0 && row_num % self.box_size as usize == 0 {
                result.push_str("┠───────┼───────┼───────┨\n");
            }
            result.push_str("┃ ");
            for (col_num, value) in row.iter().enumerate() {
                if col_num != 0 && col_num % self.box_size as usize == 0 {
                    result.push_str("│ ");
                }
                if *value == 0 {
                    result.push('-');
                } else {
                    result.push_str(&value.to_string());
                }
                result.push(' ')
            }
            result.push_str("┃\n");
        }
        result.push_str("┗━━━━━━━┷━━━━━━━┷━━━━━━━┛\n");
        write!(f, "{}", result)
    }
}

pub struct Row {
    pub row: u8,
    pub number_cols: u8,
}

impl Row {
    pub fn cells(&self) -> Vec<(u8, u8)> {
        (1..=self.number_cols).map(|col| (self.row, col)).collect()
    }
}

pub struct Column {
    pub col: u8,
    pub number_rows: u8,
}

impl Column {
    pub fn cells(&self) -> Vec<(u8, u8)> {
        (1..=self.number_rows).map(|row| (row, self.col)).collect()
    }
}

pub struct GridBox {
    pub col: u8,
    pub row: u8,
    pub box_size: u8,
}

impl GridBox {
    pub fn cells(&self) -> Vec<(u8, u8)> {
        let mut cells = Vec::new();
        for row_offset in 0..self.box_size {
            for col_offset in 0..self.box_size {
                cells.push((self.row + row_offset, self.col + col_offset))
            }
        }
        cells
    }
}
