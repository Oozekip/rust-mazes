use super::super::generator::MazeGenerator;
use super::super::grid::{Grid, BasicGrid};
use super::super::Tile;

pub fn test_gen<T: MazeGenerator>(width: usize, height: usize)
{
    let mut test_grid = BasicGrid::new(width, height);

    T::generate(&mut test_grid);

    for j in 0..height
    {
        for i in 0..width
        {
            let tile = test_grid
                .get_tile_data((i, j))
                .expect("Failed to get tile data");
            assert!(tile != Tile::new() || (width <= 1 && height <= 1),
                    "A tile in the maze is not connected");
        }
    }
}
