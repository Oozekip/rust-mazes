extern crate rand;

use rand::Rng;

use super::grid::Grid;
use super::Direction;

/// Types of generators to use for mazes
pub enum MazeType
{
    /// Generate using recursive backtracking. Creates long, winding paths with
    /// few dead ends
    RecusiveBacktracker,
}

/// Generates a maze from an existing grid using the provided generator
pub fn generate_maze<T: Grid>(grid: &mut T, maze_type: MazeType)
{
    let (width, height) = grid.get_dimensions();

    if width == 0 || height == 0 { return; }  // Can't generate on an emtpy grid

    match maze_type
    {
        MazeType::RecusiveBacktracker =>
        {
            let start_x = rand::thread_rng().gen_range(0, width);
            let start_y = rand::thread_rng().gen_range(0, height);
            // Select random starting square

            recursive_backtracker(grid, (start_x, start_y));
        }
    }
}

fn recursive_backtracker<T: Grid>(grid: &mut T, (x, y): (usize, usize))
{
    let mut stack = vec![(x, y)];

    while !stack.is_empty() 
    {
        let tile = stack[stack.len() - 1];
        let dir = get_direction(grid, tile.0, tile.1);

        if let Direction::Stay = dir
        {
            stack.pop();
            continue;
        }

        stack.push(grid.carve_path(tile, dir).expect("Failed to carve path"));
    }
}


fn get_direction<T: Grid>(grid: &T, x: usize, y: usize) -> Direction
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
        let ind = rand::thread_rng().gen_range(0, count);
        available[ind]
    }
    else
    {
        Direction::Stay
    }
}
