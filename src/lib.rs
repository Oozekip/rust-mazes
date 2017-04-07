extern crate rand;

use rand::Rng;
use std::fmt;

/// The 'Direction' type. Used to describe a direction to move in
#[derive(Copy, Clone)]
pub enum Direction
{
    /// Move up
    Up,
    /// Move down
    Down,
    /// Move left
    Left,
    /// Move right
    Right,
    /// Stay in place
    Stay,
}

/// Types of generators to use for mazes
pub enum MazeType
{
    /// Generate using recursive backtracking. Creates long, winding paths with
    /// few dead ends
    RecusiveBacktracker,
}

/// Structure holding maze data
pub struct Maze
{
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl fmt::Display for Maze
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

impl Maze
{
    /// Creates an empty maze of the given size
    pub fn new(width: usize, height: usize) -> Maze
    {
        let mut maze_temp = Maze {
            tiles: vec![],
            width: width,
            height: height,
        };


        for i in 0..width
        {
            // Create new rows
            maze_temp.tiles.push(vec![]);

            // Push new tiles to the row
            for _ in 0..height
            {
                maze_temp.tiles[i].push(Tile::new());
            }
        }


        maze_temp
    }

    /// Creates a new maze of the given size and generates data using the given type
    pub fn generate_maze(width: usize, height: usize, m_type: MazeType) -> Maze
    {
        let mut new_maze = Maze::new(width, height);

        match m_type
        {
            MazeType::RecusiveBacktracker =>
            {
                let start_x = rand::thread_rng().gen_range(0, width);
                let start_y = rand::thread_rng().gen_range(0, height);
                // Select random starting square

                new_maze.recurse((start_x, start_y));
            }
        }

        new_maze
    }

    /// Carves a path starting at the given point and moving in the
    /// given direction, if possible
    pub fn carve(&mut self, (x, y): (usize, usize), dir: Direction) -> (usize, usize)
    {

        let mut tiles = &mut self.tiles;

        // Determine directon to carve into
        let (x_c, y_c): (usize, usize) = match dir
        {
            Direction::Up => (x, y + 1),
            Direction::Right => (x + 1, y),

            // Set direction to same as current in case of underflow
            Direction::Down => if y != 0 { (x, y - 1) } else { (x, y) },
            Direction::Left => if x != 0 { (x - 1, y) } else { (x, y) },

            Direction::Stay => return (x, y),
        };

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
        if (y_c != y || x_c != x) && (y_c < self.height && x_c < self.width)
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
        }

        (x_c, y_c)
    }

    fn recurse(&mut self, (x, y): (usize, usize))
    {
        loop
        {
            let dir = self.get_direction(x, y);

            match dir
            {
                Direction::Stay => break,
                _ =>
                {
                    let tile = self.carve((x, y), dir);

                    self.recurse(tile);
                }
            }
        }
    }

    fn get_direction(&self, x: usize, y: usize) -> Direction
    {
        let tiles = &self.tiles;

        let mut available = [Direction::Stay; 4];
        let mut count = 0;

        // Check up
        if (y < self.height - 1) && (tiles[y + 1][x].is_available())
        {
            available[count] = Direction::Up;
            count += 1;
        }

        // Check down
        if (y > 0) && (tiles[y - 1][x].is_available())
        {
            available[count] = Direction::Down;
            count += 1;
        }

        // Check left
        if (x > 0) && (tiles[y][x - 1].is_available())
        {
            available[count] = Direction::Left;
            count += 1;
        }

        // Check right
        if (x < self.width - 1) && (tiles[y][x + 1].is_available())
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
}

enum Tile
{
    Empty,
    Connected
    {
        up: bool,
        down: bool,
        left: bool,
        right: bool,
    },
}

impl Tile
{
    fn new() -> Tile
    {
        Tile::Connected {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    fn is_available(&self) -> bool
    {
        match self
        {
            &Tile::Connected {
                 up: u,
                 down: d,
                 left: l,
                 right: r,
             } => !u && !d && !l && !r,
            _ => false,
        }
    }
}

impl fmt::Display for Tile
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let fmt_char: char = match self
        {
            // ━┃┏┓┗┛┣┫┳┻╋
            // ═║╔╗╚╝╠╣╦╩╬
            &Tile::Connected {
                 up: u,
                 down: d,
                 left: l,
                 right: r,
             } =>
            {
                if u && d && l && r
                {
                    '╋'
                }
                else if u && d && l
                {
                    '┫'
                }
                else if u && d && r
                {
                    '┣'
                }
                else if u && l && r
                {
                    '┻'
                }
                else if d && l && r
                {
                    '┳'
                }
                else if u && l
                {
                    '┛'
                }
                else if u && r
                {
                    '┗'
                }
                else if d && l
                {
                    '┓'
                }
                else if d && r
                {
                    '┏'
                }
                else if u || d
                {
                    '┃'
                }
                else if l || r
                {
                    '━'
                }
                else
                {
                    '█'
                }
            }

            &Tile::Empty => '░',
        };

        write!(f, "{}", fmt_char)
    }
}
