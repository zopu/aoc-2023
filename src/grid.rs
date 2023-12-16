use std::fmt::{Debug, Display};

pub struct Grid<T> {
    pub dimensions: (usize, usize),
    grid: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(default_elem: T, dim_x: usize, dim_y: usize) -> Self
    where
        T: Clone,
    {
        let grid = vec![vec![default_elem; dim_x]; dim_y];
        let dimensions = (dim_x, dim_y);
        Self { dimensions, grid }
    }

    pub fn at(&self, x: usize, y: usize) -> &T {
        &self.grid[y][x]
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.grid[y][x]
    }

    pub fn parse(input: &str) -> Grid<char> {
        let grid = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let dimensions = (grid[0].len(), grid.len());
        Grid { dimensions, grid }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.grid.iter().flat_map(|row| row.iter())
    }
}

impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for c in row {
                write!(f, "{:?}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
