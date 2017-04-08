
use std::fmt;

use super::Tile;
use super::Direction;

/// Defines the 2D square Grid trait for using the maze generator
pub trait Grid
{
    /// Gets tile data at a given position on a grid
    fn get_tile_data(&self, (usize, usize)) -> Result<Tile, &'static str>;

    /// Gets the width and height respectively of the grid
    fn get_dimensions(&self) -> (usize, usize);

    /// Carves a path starting at the given point and moving in the
    /// given direction, if possible
    fn carve_path(&mut self, (usize, usize), Direction) -> Result<(usize, usize), &'static str>;
}

/// Basic grid class used for examples
pub struct BasicGrid
{
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl fmt::Display for BasicGrid
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let mut string_fmt = String::new();

        for row in (self.tiles.iter()).rev()
        {
            for tile in row
            {
                string_fmt += &format!("{}", tile);
            }

            string_fmt += &String::from("\n");
        }

        write!(f, "{}", string_fmt)
    }
}

impl Grid for BasicGrid
{
    fn get_tile_data(&self, (x, y): (usize, usize)) -> Result<Tile, &'static str>
    {
        let (width, height) = self.get_dimensions();

        if (x < width) && (y < height)
        {
            Result::Ok(self.tiles[y][x])
        }
        else
        {
            Result::Err("Invalid tile index")
        }
    }

    fn get_dimensions(&self) -> (usize, usize)
    {
        (self.width, self.height)
    }

    fn carve_path(&mut self,
                  (x, y): (usize, usize),
                  dir: Direction)
                  -> Result<(usize, usize), &'static str>
    {
        let (width, height) = self.get_dimensions();
        let tiles = &mut self.tiles;

        // Determine directon to carve into
        let (x_c, y_c): (usize, usize) = match dir
        {
            Direction::Up => (x, y + 1),
            Direction::Right => (x + 1, y),

            // Set direction to same as current in case of underflow
            Direction::Down => if y != 0 { (x, y - 1) } else { (x, y) },

            Direction::Left => if x != 0 { (x - 1, y) } else { (x, y) },

            Direction::Stay => return Result::Err("Told carve to stay"),
        };

        if x == x_c && y == y_c
        {
            return Result::Err("Cannot carve in desired dirrection");
        }

        {
            let mut from_tile = &mut tiles[y][x];

            // Set full tile as connected

            match *from_tile
            {
                Tile::Connected {
                    up: ref mut u,
                    down: ref mut d,
                    left: ref mut l,
                    right: ref mut r,
                } =>
                {
                    match dir
                    {
                        Direction::Up => *u = true,
                        Direction::Down => *d = true,
                        Direction::Left => *l = true,
                        Direction::Right => *r = true,
                        _ =>
                        {}
                    }
                }
                _ =>
                {}
            }
        }

        // start != end and in bounds
        if y_c < height && x_c < width
        {
            let mut to_tile = &mut tiles[y_c][x_c];

            match *to_tile
            {
                Tile::Connected {
                    up: ref mut u,
                    down: ref mut d,
                    left: ref mut l,
                    right: ref mut r,
                } =>
                {
                    match dir
                    {
                        Direction::Up => *d = true,
                        Direction::Down => *u = true,
                        Direction::Left => *r = true,
                        Direction::Right => *l = true,
                        _ =>
                        {}
                    }
                }
                _ =>
                {}
            }

            Result::Ok((x_c, y_c))

        }
        else
        {
            Result::Err("Cannot carave in desired dirrection")
        }
    }
}

impl BasicGrid
{
    /// Creates an empty maze of the given size
    pub fn new(width: usize, height: usize) -> BasicGrid
    {
        let mut maze_temp = BasicGrid {
            tiles: vec![],
            width: width,
            height: height,
        };


        for j in 0..height
        {
            // Create new rows
            maze_temp.tiles.push(vec![]);

            // Push new tiles to the row
            for _ in 0..width
            {
                maze_temp.tiles[j].push(Tile::new());
            }
        }


        maze_temp
    }
}
