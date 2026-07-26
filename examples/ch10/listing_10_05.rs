use std::fmt;

fn main() {
    struct Grid<T, const N: usize> { data: [[T; N]; N] }
    impl<T: fmt::Display, const N: usize> Grid<T, N> {
        fn print(&self) {
            for row in &self.data {
                for item in row { print!("{item} "); }
                println!();
            }
        }
    }
    let grid = Grid::<i32, 3> { data: [[1, 2, 3], [4, 5, 6], [7, 8, 9]] };
    grid.print();
}
