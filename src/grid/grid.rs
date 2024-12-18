pub struct Grid<T>
where
    T: Clone + std::fmt::Debug,
{
    data: Vec<Vec<T>>,
}
/**
     1 2 3 4 5 6 7 8 9
*/
impl<T> Grid<T>
where
    T: Clone + std::fmt::Debug,
{
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row).and_then(|r| r.get(col))
    }
    pub fn iter_rows(&self) -> impl Iterator<Item = Vec<&T>> {
        self.data.iter().map(|r| r.iter().collect())
    }

    pub fn iter_cols(&self) -> impl Iterator<Item = Vec<&T>> {
        (0..self.data[0].len())
            .map(move |col| self.data.iter().filter_map(|row| row.get(col)).collect())
    }

    pub fn diags_left(&self) -> Vec<Vec<&T>> {
        let row_len = self.data.len() as i32;
        let col_len = self.data[0].len() as i32;
        let mut diagonals = Vec::new();

        for d in 0..(row_len + col_len - 1) {
            let mut start_row = if d < col_len { 0 } else { d - col_len + 1 };
            let mut start_col = if d < col_len { d } else { col_len - 1 };

            let mut diag = Vec::new();
            while start_row < row_len && start_col >= 0 {
                diag.push(&self.data[start_row as usize][start_col as usize]);
                start_row += 1;
                start_col -= 1;
            }
            diagonals.push(diag);
        }
        diagonals
    }
    pub fn diags_right(&self) -> Vec<Vec<&T>> {
        let row_len = self.data.len() as i32;
        let col_len = self.data[0].len() as i32;
        let mut diagonals = Vec::new();

        for d in 0..(row_len + col_len - 1) {
            println!("{d}");
            let mut start_row = if d < col_len { row_len - 1 } else { 0 };
            let mut start_col = if d < col_len { d } else { col_len - 1 };

            let mut diag = Vec::new();
            while start_row < row_len && start_col >= 0 && start_row >= 0 {
                diag.push(&self.data[start_row as usize][start_col as usize]);
                start_row -= 1;
                start_col += 1;
            }
            diagonals.push(diag);
        }
        diagonals
    }
}

pub enum DiagnonalDirection {
    Start,
    End,
}

impl From<Vec<String>> for Grid<char> {
    fn from(value: Vec<String>) -> Self {
        let str = value
            .into_iter()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Self { data: str }
    }
}

// 1 2 3
// 4 5 6
// 7 8 9
// ->
// 1  (0,0)
// 2, 4 (0,1) (1,0)
// 3, 5, 7 (0,2) (1,1) (2,0)

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_from_vec() {
        let g = Grid::from(vec![
            "123".to_string(),
            "456".to_string(),
            "789".to_string(),
        ]);
        assert!(g.iter_rows().next() == Some(vec![&'1', &'2', &'3']));
        assert!(g.iter_cols().next() == Some(vec![&'1', &'4', &'7']));
    }

    #[test]
    fn iterate_diag_left() {
        let g = Grid::from(vec![
            "123".to_string(),
            "456".to_string(),
            "789".to_string(),
        ]);
        let mut it = g.diags_left().into_iter();
        assert_eq!(it.next(), Some(vec![&'1']));
        assert_eq!(it.next(), Some(vec![&'2', &'4']));
        assert_eq!(it.next(), Some(vec![&'3', &'5', &'7']));
    }
    #[test]
    fn iterate_diag_right() {
        let g = Grid::from(vec![
            "123".to_string(),
            "456".to_string(),
            "789".to_string(),
        ]);
        let mut it = g.diags_right().into_iter();
        assert_eq!(it.next(), Some(vec![&'3']));
        assert_eq!(it.next(), Some(vec![&'2', &'6']));
        assert_eq!(it.next(), Some(vec![&'1', &'5', &'9']));
    }
}
