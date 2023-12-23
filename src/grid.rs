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
        debug_assert!(x < self.dimensions.0, "x {} out of bounds", x);
        debug_assert!(y < self.dimensions.1, "y {} out of bounds", y);
        &self.grid[y * self.dimensions.0 + x]
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.grid[y * self.dimensions.0 + x]
    }

    pub fn parse(input: &str, parse_char: impl Fn(char) -> T) -> Grid<T> {
        let dimensions = (input.lines().count(), input.lines().next().unwrap().len());
        let grid = input
            .chars()
            .filter(|c| *c != '\n')
            .map(parse_char)
            .collect::<Vec<T>>();

        Grid { dimensions, grid }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.grid.iter()
    }

    pub fn iter_pts(&self, mut f: impl FnMut(usize, usize, &T)) {
        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                f(x, y, self.at(x, y));
            }
        }
    }

    #[allow(unused)]
    pub fn to_string(&self, format_elem: impl Fn(&T) -> String) -> String {
        let mut s = String::new();
        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                s.push_str(&format_elem(self.at(x, y)));
            }
            s.push('\n');
        }
        s
    }
}

impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.dimensions.1 {
            for c in 0..self.dimensions.0 {
                write!(f, "{:?}", self.at(c, row))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.dimensions.1 {
            for c in 0..self.dimensions.0 {
                write!(f, "{}", self.at(c, row))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.dimensions.1 {
            for c in 0..self.dimensions.0 {
                write!(f, "{}", *self.at(c, row) as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Clone for Grid<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            dimensions: self.dimensions,
            grid: self.grid.clone(),
        }
    }
}
