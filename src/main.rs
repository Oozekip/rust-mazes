extern crate mazes;

use mazes::Maze;
use mazes::MazeType;
//use mazes::maze::Direction

fn main()
{
    let m = Maze::generate_maze(32, 32, MazeType::RecusiveBacktracker);

    println!("{}", &m);
}
