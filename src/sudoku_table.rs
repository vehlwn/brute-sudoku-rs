use serde::{Deserialize, Serialize};

const _EMPTY_CHAR: char = '.';
pub const EMPTY_NUM: u32 = 0;

#[derive(Serialize, Deserialize)]
pub enum Format {
    Compact,
    Indent,
}

#[derive(Debug, Clone)]
pub struct SudokuTable {
    m_grid: [u32; 9 * 9],
}

impl Default for SudokuTable {
    fn default() -> Self {
        return Self {
            m_grid: [EMPTY_NUM; 9 * 9],
        };
    }
}

impl SudokuTable {
    pub fn reset(&mut self) {
        self.m_grid.fill(0);
    }

    pub fn try_parse(&mut self, s: &str) -> bool {
        self.reset();
        let mut i: usize = 0;
        for c in s.chars() {
            if i == self.m_grid.len() {
                break;
            } else if c.is_ascii_whitespace() {
                continue;
            } else if c == _EMPTY_CHAR {
                self.m_grid[i] = EMPTY_NUM;
            } else if c.is_ascii_digit() {
                self.m_grid[i] = c.to_digit(10).unwrap();
            } else {
                return false;
            }
            i += 1;
        }
        return true;
    }

    pub fn to_string(&self, format: Format) -> String {
        let mut ret = String::new();
        match format {
            Format::Compact => {
                for x in self.m_grid {
                    if x == EMPTY_NUM {
                        ret.push(_EMPTY_CHAR);
                    } else {
                        ret.push_str(&x.to_string());
                    }
                }
            }
            Format::Indent => {
                for (i, x) in self.m_grid.iter().cloned().enumerate() {
                    if x == EMPTY_NUM {
                        ret.push(_EMPTY_CHAR);
                    } else {
                        ret.push_str(&x.to_string());
                    }
                    ret.push_str(" ");
                    // 1 block vertically
                    if (i + 1) % (9 * 3) == 0 {
                        ret.push_str("\n\n");
                    }
                    // 1 line
                    else if (i + 1) % 9 == 0 {
                        ret.push_str("\n");
                    }
                    // 1 block within 1 line
                    else if (i + 1) % 3 == 0 {
                        ret.push_str("  ");
                    }
                }
            }
        }
        return ret;
    }

    pub fn try_set(&mut self, pos: usize, val: u32) -> bool {
        if self.is_legal(pos, val) {
            self.m_grid[pos] = val;
            return true;
        } else {
            return false;
        }
    }

    pub fn is_legal(&self, pos: usize, val: u32) -> bool {
        let row = pos / 9;
        let col = pos % 9;
        return self.is_row_ok(row, col, val)
            && self.is_col_ok(row, col, val)
            && self.is_sqr_ok(row, col, val);
    }

    pub fn is_row_ok(&self, row: usize, col: usize, val: u32) -> bool {
        return self
            .row(row)
            .enumerate()
            .all(|(i, old_val)| i == col || old_val != val || old_val == EMPTY_NUM);
    }

    pub fn is_col_ok(&self, row: usize, col: usize, val: u32) -> bool {
        return self
            .col(col)
            .enumerate()
            .all(|(j, old_val)| j == row || old_val != val || old_val == EMPTY_NUM);
    }

    pub fn is_sqr_ok(&self, row: usize, col: usize, val: u32) -> bool {
        let row_corner = row / 3 * 3;
        let col_corner = col / 3 * 3;
        for i in row_corner..row_corner + 3 {
            for j in col_corner..col_corner + 3 {
                let old_val = self.m_grid[i * 9 + j];
                if (i != row || j != col) && old_val == val && old_val != EMPTY_NUM {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn row(&self, i: usize) -> impl Iterator<Item = u32> + '_ {
        return self.row_slice(i).iter().cloned();
    }

    pub fn row_slice(&self, i: usize) -> &[u32] {
        let row_start = i * 9;
        return &self.m_grid[row_start..row_start + 9];
    }

    pub fn col(&self, j: usize) -> impl Iterator<Item = u32> + '_ {
        return self.m_grid.iter().cloned().skip(j).step_by(9);
    }

    pub fn verify_all(&self) -> bool {
        return self
            .m_grid
            .iter()
            .cloned()
            .enumerate()
            .all(|(pos, val)| val == EMPTY_NUM || self.is_legal(pos, val));
    }

    pub fn get_sorted_rows_indexes(&self) -> Vec<usize> {
        let count_empty =
            |row: &[u32]| row.iter().cloned().filter(|&x| x == EMPTY_NUM).count();
        let mut tmp = (0..9)
            .map(|i| (i, self.row_slice(i)))
            .collect::<Vec<(usize, &[u32])>>();
        tmp.sort_by(|(_pos_a, row_a), (_pos_b, row_b)| {
            count_empty(row_a).cmp(&count_empty(row_b))
        });
        return tmp.iter().cloned().map(|(i, _slice)| i).collect();
    }

    pub fn get_empty_cell(
        &self,
        start_elem: usize,
        rows_indexes: &Vec<usize>,
    ) -> Option<usize> {
        let start_row = start_elem / 9;
        let start_col = start_elem % 9;
        if let Some((_x, i)) = self
            .row(start_row)
            .zip(0..9)
            .skip(start_col)
            .find(|&(x, _i)| x == EMPTY_NUM)
        {
            return Some(i + start_row * 9);
        }
        let mut it = rows_indexes.iter().cloned().cycle();
        it.find(|&x| x == start_row).expect(&format!("Unreachable! rows_indexes ({:?}) does not contain requested start_row ({}), start_elem = {}", rows_indexes, start_row, start_elem));
        for (_row_counter, sorted_row_index) in (1..9).zip(it) {
            if let Some(pos) =
                self.row(sorted_row_index).position(|x| x == EMPTY_NUM)
            {
                return Some(pos + sorted_row_index * 9);
            }
        }
        return None;
    }
}

impl std::ops::Index<usize> for SudokuTable {
    type Output = u32;

    fn index(&self, i: usize) -> &Self::Output {
        return &self.m_grid[i];
    }
}
impl std::ops::IndexMut<usize> for SudokuTable {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        return &mut self.m_grid[i];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const UNFILL_STR: &str = "
        . . .  9 . .  . 3 .
        3 . 6  . 2 .  . 4 .
        2 . 4  . . 3  1 . 6

        . 7 .  . 5 1  . 8 .
        . 3 1  . 6 .  . 5 7
        5 . 9  . . .  6 . .

        4 1 .  . . 2  . 7 8
        7 6 3  . . 5  4 . .
        9 2 8  . . 4  . . 1
        ";

    const FILL_STR: &str = "
        5 3 4  6 7 8  9 1 2
        6 7 2  1 9 5  3 4 8
        1 9 8  3 4 2  5 6 7

        8 5 9  7 6 1  4 2 3
        4 2 6  8 5 3  7 9 1
        7 1 3  9 2 4  8 5 6

        9 6 1  5 3 7  2 8 4
        2 8 7  4 1 9  6 3 5
        3 4 5  2 8 6  1 7 9
        ";

    #[test]
    fn try_parse_empty() {
        let s = [_EMPTY_CHAR; 9 * 9].iter().collect::<String>();
        let mut t = SudokuTable::default();
        assert!(t.try_parse(&s));
    }

    #[test]
    fn try_parse_unfill() {
        let s = UNFILL_STR;
        let mut t = SudokuTable::default();
        assert!(t.try_parse(&s));
    }

    #[test]
    fn try_parse_fill() {
        let s = FILL_STR;
        let mut t = SudokuTable::default();
        assert!(t.try_parse(&s));
    }

    #[test]
    fn row() {
        let s = FILL_STR;
        let mut t = SudokuTable::default();
        assert!(t.try_parse(&s));
        assert_eq!(
            t.row(1).collect::<Vec<u32>>(),
            vec![6, 7, 2, 1, 9, 5, 3, 4, 8]
        );
        for (i, arr) in [
            (0, [5, 3, 4, 6, 7, 8, 9, 1, 2]),
            (1, [6, 7, 2, 1, 9, 5, 3, 4, 8]),
            (2, [1, 9, 8, 3, 4, 2, 5, 6, 7]),
            (3, [8, 5, 9, 7, 6, 1, 4, 2, 3]),
            (4, [4, 2, 6, 8, 5, 3, 7, 9, 1]),
            (5, [7, 1, 3, 9, 2, 4, 8, 5, 6]),
            (6, [9, 6, 1, 5, 3, 7, 2, 8, 4]),
            (7, [2, 8, 7, 4, 1, 9, 6, 3, 5]),
            (8, [3, 4, 5, 2, 8, 6, 1, 7, 9]),
        ]
        .iter()
        .cloned()
        {
            assert_eq!(t.row(i).collect::<Vec<u32>>(), arr);
            assert_eq!(t.row_slice(i), arr);
        }
    }

    #[test]
    fn col() {
        let s = FILL_STR;
        let mut t = SudokuTable::default();
        assert!(t.try_parse(&s));
        assert_eq!(
            t.row(1).collect::<Vec<u32>>(),
            vec![6, 7, 2, 1, 9, 5, 3, 4, 8]
        );
        for (j, arr) in [
            (0, [5, 6, 1, 8, 4, 7, 9, 2, 3]),
            (1, [3, 7, 9, 5, 2, 1, 6, 8, 4]),
            (2, [4, 2, 8, 9, 6, 3, 1, 7, 5]),
            (3, [6, 1, 3, 7, 8, 9, 5, 4, 2]),
            (4, [7, 9, 4, 6, 5, 2, 3, 1, 8]),
            (5, [8, 5, 2, 1, 3, 4, 7, 9, 6]),
            (6, [9, 3, 5, 4, 7, 8, 2, 6, 1]),
            (7, [1, 4, 6, 2, 9, 5, 8, 3, 7]),
            (8, [2, 8, 7, 3, 1, 6, 4, 5, 9]),
        ]
        .iter()
        .cloned()
        {
            assert_eq!(t.col(j).collect::<Vec<u32>>(), arr);
        }
    }

    #[test]
    fn verify_empty() {
        let s = [_EMPTY_CHAR; 9 * 9].iter().collect::<String>();
        let mut t = SudokuTable::default();
        assert!(t.try_parse(&s));
        assert!(t.verify_all());
    }

    #[test]
    fn verify_unfill() {
        let s = UNFILL_STR;
        let mut t = SudokuTable::default();
        assert!(t.try_parse(&s));
        assert!(t.verify_all());
    }

    #[test]
    fn verify_fill() {
        let s = FILL_STR;
        let mut t = SudokuTable::default();
        assert!(t.try_parse(&s));
        assert!(t.verify_all());
    }

    #[test]
    fn index_unfill() {
        let s = UNFILL_STR;
        let mut t = SudokuTable::default();
        assert!(t.try_parse(&s));
        assert_eq!(t[0], EMPTY_NUM);
        assert_eq!(t[1], EMPTY_NUM);
        assert_eq!(t[2], EMPTY_NUM);
        assert_eq!(t[3], 9);
        assert_eq!(t[4], EMPTY_NUM);
        assert_eq!(t[5], EMPTY_NUM);
        assert_eq!(t[6], EMPTY_NUM);
        assert_eq!(t[7], 3);
        assert_eq!(t[8], EMPTY_NUM);
        assert_eq!(t[11], 6);
    }

    #[test]
    fn sorted_row_indexes() {
        let s = UNFILL_STR;
        let mut t = SudokuTable::default();
        assert!(t.try_parse(&s));
        let v = t.get_sorted_rows_indexes();
        assert_eq!(v, [2, 4, 6, 7, 8, 1, 3, 5, 0]);
    }

    #[test]
    fn empty_cell() {
        let s = UNFILL_STR;
        let mut t = SudokuTable::default();
        assert!(t.try_parse(&s));
        let v = t.get_sorted_rows_indexes();
        let mut start_cell = v[0] * 9;
        assert_eq!(start_cell, 18);
        for correct_cell in [
            vec![2 * 9 + 1, 2 * 9 + 3, 2 * 9 + 4, 2 * 9 + 7], // 2'nd row
            vec![4 * 9 + 0, 4 * 9 + 3, 4 * 9 + 5, 4 * 9 + 6], // 4'th row
            vec![6 * 9 + 2, 6 * 9 + 3, 6 * 9 + 4, 6 * 9 + 6], // 6'th row
            vec![7 * 9 + 3, 7 * 9 + 4, 7 * 9 + 7, 7 * 9 + 8], // 7'th row
            vec![8 * 9 + 3, 8 * 9 + 4, 8 * 9 + 6, 8 * 9 + 7], // 8'th row
            vec![1 * 9 + 1, 1 * 9 + 3, 1 * 9 + 5, 1 * 9 + 6, 1 * 9 + 8], // 1'st row
            vec![3 * 9 + 0, 3 * 9 + 2, 3 * 9 + 3, 3 * 9 + 6, 3 * 9 + 8], // 3'rd row
            vec![
                5 * 9 + 1,
                5 * 9 + 3,
                5 * 9 + 4,
                5 * 9 + 5,
                5 * 9 + 7,
                5 * 9 + 8,
            ], // 5'th row
            vec![0, 1, 2, 4, 5, 6, 8],                        // 0'th row
        ]
        .iter()
        .flatten()
        .cloned()
        {
            start_cell = t.get_empty_cell(start_cell, &v).unwrap();
            assert_eq!(start_cell, correct_cell);
            t[start_cell] = 9;
        }
        assert!(t.get_empty_cell(0, &v).is_none());
    }

    #[test]
    fn empty_cell_cycled() {
        let s = UNFILL_STR;
        let mut t = SudokuTable::default();
        assert!(t.try_parse(&s));
        let v = t.get_sorted_rows_indexes();
        let mut start_cell = v[3] * 9;
        assert_eq!(start_cell, 7 * 9);
        for correct_cell in [
            vec![7 * 9 + 3, 7 * 9 + 4, 7 * 9 + 7, 7 * 9 + 8], // 7'th row
            vec![8 * 9 + 3, 8 * 9 + 4, 8 * 9 + 6, 8 * 9 + 7], // 8'th row
            vec![1 * 9 + 1, 1 * 9 + 3, 1 * 9 + 5, 1 * 9 + 6, 1 * 9 + 8], // 1'st row
            vec![3 * 9 + 0, 3 * 9 + 2, 3 * 9 + 3, 3 * 9 + 6, 3 * 9 + 8], // 3'rd row
            vec![
                5 * 9 + 1,
                5 * 9 + 3,
                5 * 9 + 4,
                5 * 9 + 5,
                5 * 9 + 7,
                5 * 9 + 8,
            ], // 5'th row
            vec![0, 1, 2, 4, 5, 6, 8],                        // 0'th row
            vec![2 * 9 + 1, 2 * 9 + 3, 2 * 9 + 4, 2 * 9 + 7], // 2'nd row
            vec![4 * 9 + 0, 4 * 9 + 3, 4 * 9 + 5, 4 * 9 + 6], // 4'th row
            vec![6 * 9 + 2, 6 * 9 + 3, 6 * 9 + 4, 6 * 9 + 6], // 6'th row
        ]
        .iter()
        .flatten()
        .cloned()
        {
            start_cell = t.get_empty_cell(start_cell, &v).unwrap();
            assert_eq!(start_cell, correct_cell);
            t[start_cell] = 9;
        }
        assert!(t.get_empty_cell(0, &v).is_none());
    }
}
