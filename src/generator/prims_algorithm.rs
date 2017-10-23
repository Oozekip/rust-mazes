extern crate rand;

use std::collections::HashSet;

use rand::Rng;

use super::Point;
use super::Direction;
use super::super::grid::Grid;
use super::MazeGenerator;

pub struct Prim
{
    filled: HashSet<Point>,
    frontier: HashSet<Point>,
}

impl MazeGenerator for Prim
{
    fn generate<T: Grid>(grid: &mut T)
    {
        let (width, height) = grid.get_dimensions();

        if width == 0 || height == 0
        {
            return;
        }

        let mut state = Prim::new();

        let x = rand::thread_rng().gen_range(0, width);
        let y = rand::thread_rng().gen_range(0, height);

        state.filled.insert((x, y));

        state.get_frontier(grid, (x, y));

        while !state.frontier.is_empty()
        {
            // Pick random frontier tile
            state.add_random_frontier(grid);
        }

    }
}

impl Prim
{
    fn new() -> Prim
    {
        Prim {
            filled: HashSet::new(),
            frontier: HashSet::new(),
        }
    }

    fn get_frontier<T: Grid>(&mut self, grid: &T, (x, y): Point)
    {
        let (width, height) = grid.get_dimensions();

        let filled = &self.filled;
        let frontier = &mut self.frontier;

        // Check left
        if x > 0
        {
            if !filled.contains(&(x - 1, y))
            {
                frontier.insert((x - 1, y));
            }
        }

        // Check right
        if x < width - 1
        {
            if !filled.contains(&(x + 1, y))
            {
                frontier.insert((x + 1, y));
            }
        }

        // Check up
        if y < height - 1
        {
            if !filled.contains(&(x, y + 1))
            {
                frontier.insert((x, y + 1));
            }
        }

        // Check down
        if y > 0
        {
            if !filled.contains(&(x, y - 1))
            {
                frontier.insert((x, y - 1));
            }
        }
    }

    fn add_random_frontier<T: Grid>(&mut self, grid: &mut T)
    {
        let tile = self.pop_random_frontier()
            .expect("Invalid frontier state");

        self.place_tile_filled(grid, tile);

        self.get_frontier(grid, tile);
    }

    fn pop_random_frontier(&mut self) -> Result<Point, &'static str>
    {
        // Get size of frontier
        let max_val = self.frontier.len();

        // get random index for value
        let index = rand::thread_rng().gen_range(0, max_val);
        let mut point = Option::None;

        for (i, v) in self.frontier.iter().enumerate()
        {
            if i == index
            {
                point = Option::Some(*v);
                break;
            }
        }

        if let Option::Some(p) = point
        {
            assert!(self.frontier.remove(&p));
            Result::Ok(p)
        }
        else
        {
            Result::Err("Frontier is empty")
        }
    }

    fn place_tile_filled<T: Grid>(&mut self, grid: &mut T, (x, y): Point)
    {
        let mut dirs = Vec::new();
        let (width, height) = grid.get_dimensions();
        let filled = &mut self.filled;

        // Check up
        if y < height - 1 && filled.contains(&(x, y + 1))
        {
            dirs.push(Direction::Up);
        }
        // Check down
        if y > 0 && filled.contains(&(x, y - 1))
        {
            dirs.push(Direction::Down);
        }
        // Check left
        if x > 0 && filled.contains(&(x - 1, y))
        {
            dirs.push(Direction::Left);
        }
        // Check right
        if x < width - 1 && filled.contains(&(x + 1, y))
        {
            dirs.push(Direction::Right);
        }

        if dirs.len() > 0
        {
            // Pick random direction
            let index = rand::thread_rng().gen_range(0, dirs.len());

            grid.carve_path((x, y), dirs[index])
                .expect("Failed to carve path");

            filled.insert((x, y));
        }
    }
}
