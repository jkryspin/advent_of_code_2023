mod day;
pub mod template;

pub use day::*;
use ndarray::{Array2, Axis};

pub trait GridCreator<T> {
    fn create_grid(&self) -> Array2<T>;
}
impl GridCreator<char> for &str {
    fn create_grid(&self) -> Array2<char> {
        self.lines().collect::<Vec<_>>().create_grid()
    }
}

impl GridCreator<char> for Vec<&str> {
    fn create_grid(&self) -> Array2<char> {
        let mut grid = Array2::<char>::default((self.len(), self[0].len()));
        for (i, mut row) in grid.axis_iter_mut(Axis(0)).enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                let c = self.get(i).unwrap().chars().nth(j).unwrap();
                *col = c;
            }
        }
        return grid;
    }
}
