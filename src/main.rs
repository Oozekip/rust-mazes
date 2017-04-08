extern crate mazes;

use mazes::generator::{generate_maze, MazeType};
use mazes::grid::BasicGrid;

fn main()
{
    let mut grid = BasicGrid::new(170, 512);

    generate_maze(&mut grid, MazeType::RecusiveBacktracker);

    println!("{}", grid);
}