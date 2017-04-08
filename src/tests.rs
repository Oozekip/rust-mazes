use super::generator::{generate_maze, MazeType};
use super::grid::{Grid, BasicGrid};
use super::Tile;

/// Generates a normal sized maze
#[test]
fn gen_normal_maze()
{
    test_gen(32, 32)
}

#[test]
fn tile_equality()
{
    let empty_tile = Tile::Empty;
    let full_tile = Tile::new();
    let partial_tile = Tile::Connected{up: true, down: true, left: false, right: false};

    assert!(empty_tile != full_tile, "Empty tile tested equal to full tile");
    assert!(empty_tile != partial_tile, "Empty tile tested equal to partialy full tile");
    assert!(partial_tile != full_tile, "Partially full tile tested equal to full tile");
    assert!(partial_tile == partial_tile, "Tile tested inequal to itself");
}

/// Generates an empty maze
#[test]
fn gen_empty_maze()
{
    test_gen(0, 0);
}

// Generates a huge maze
#[test]
fn gen_huge_maze()
{
    test_gen(2048, 2048);
}

/// Generates a very small maze
#[test]
fn gen_tiny_maze()
{
    test_gen(1, 1);
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

    for j in 0..height
    {
        for i in 0..width
        {
            let tile = test_grid.get_tile_data((i, j)).expect("Failed to get tile data");
            assert!(tile != Tile::new() || (width <=1 && height <= 1), "A tile in the maze is not connected");
        }
    }
}