use super::generator::{generate_maze, MazeType};
use super::grid::BasicGrid;

/// Generates a normal sized maze
#[test]
fn gen_normal_maze()
{
    test_gen(32, 32)
}

/// Generates an empty maze
#[test]
fn gen_empty_maze()
{
    test_gen(0, 0);
}

// Generates a huge maze (Failing, stack overflow)
// #[test]
// fn gen_huge_maze()
// {
//     test_gen(512, 512);
// }

/// Generates a very small maze
#[test]
fn gen_tiny_maze()
{
    test_gen(2, 2);
}

/// Generates a rectangular maze (Failing)
#[test]
fn gen_rectangular_maze()
{
    test_gen(32, 16);
}

fn test_gen(width: usize, height: usize)
{
    let mut test_grid = BasicGrid::new(width, height);

    generate_maze(&mut test_grid, MazeType::RecusiveBacktracker);

    println!("{}", test_grid);
}