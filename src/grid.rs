use std::fmt::{Debug, Display};

pub struct Grid<T> {
    pub dimensions: (usize, usize), // cols, rows
    grid: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(default_elem: T, dim_x: usize, dim_y: usize) -> Self
    where
        T: Clone,
    {
        let grid = vec![default_elem; dim_x * dim_y];
        let dimensions = (dim_x, dim_y);
        Self { dimensions, grid }
    }

    pub fn at(&self, x: usize, y: usize) -> &T {
        &self.grid[y * self.dimensions.0 + x]
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.grid[y * self.dimensions.0 + x]
    }

    pub fn parse(input: &str) -> Grid<char> {
        let dimensions = (input.lines().count(), input.lines().next().unwrap().len());
        let grid = input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();

        Grid { dimensions, grid }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.grid.iter()
    }
}

impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.dimensions.0 {
            for c in 0..self.dimensions.1 {
                write!(f, "{:?}", self.at(row, c))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.dimensions.0 {
            for c in 0..self.dimensions.1 {
                write!(f, "{}", self.at(row, c))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
