extern crate rand;

use rand::Rng;

use super::Point;
use super::Direction;
use super::MazeGenerator;
use super::super::grid::Grid;

pub struct RecursiveBacktracker;

impl MazeGenerator for RecursiveBacktracker
{
    fn generate<T: Grid>(grid: &mut T)
    {
        let (width, height) = grid.get_dimensions();

        if width == 0 || height == 0
        {
            return;
        }

        let x = rand::thread_rng().gen_range(0..width);
        let y = rand::thread_rng().gen_range(0..height);

        let mut stack = vec![(x, y)];

        while !stack.is_empty()
        {
            let tile = stack[stack.len() - 1];
            let dir = get_direction(grid, (tile.0, tile.1));

            if let Direction::Stay = dir
            {
                stack.pop();
                continue;
            }

            stack.push(grid.carve_path(tile, dir).expect("Failed to carve path"));
        }
    }
}

fn get_direction<T: Grid>(grid: &T, (x, y): Point) -> Direction
{
    let mut available = [Direction::Stay; 4];
    let mut count = 0;

    let (width, height) = grid.get_dimensions();

    // Check up
    if (y < height - 1) &&
       (grid.get_tile_data((x, y + 1))
            .expect("Invalid tile")
            .is_available())
    {
        available[count] = Direction::Up;
        count += 1;
    }

    // Check down
    if (y > 0) &&
       (grid.get_tile_data((x, y - 1))
            .expect("Invalid tile")
            .is_available())
    {
        available[count] = Direction::Down;
        count += 1;
    }

    // Check left
    if (x > 0) &&
       (grid.get_tile_data((x - 1, y))
            .expect("Invalid tile")
            .is_available())
    {
        available[count] = Direction::Left;
        count += 1;
    }

    // Check right
    if (x < width - 1) &&
       (grid.get_tile_data((x + 1, y))
            .expect("Invalid tile")
            .is_available())
    {
        available[count] = Direction::Right;
        count += 1;
    }

    if count > 0
    {
        // Generate a number between 0 and count
        let ind = rand::thread_rng().gen_range(0..count);
        available[ind]
    }
    else
    {
        Direction::Stay
    }
}
