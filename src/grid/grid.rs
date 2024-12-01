struct Grid<T>
where
    T: Clone,
{
    data: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: Clone,
{
    fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row).and_then(|r| r.get(col))
    }
    fn iter_rows(&self) -> impl Iterator<Item = Vec<&T>> {
        self.data.iter().map(|r| r.iter().collect())
    }
    fn iter_cols(&self) -> impl Iterator<Item = Vec<&T>> {
        (0..self.data[0].len())
            .map(move |col| self.data.iter().filter_map(|row| row.get(col)).collect())
    }
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
}
