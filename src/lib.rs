extern crate rand;

use std::fmt;

pub mod grid;
pub mod generator;

#[cfg(test)]
mod tests;

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

/// The 'Tile' type. Used to describe the state of a grid position
#[derive(Copy, Clone)]
pub enum Tile
{
    // An empty grid tile with no connections
    Empty,

    // A connected tile, with data on how it is linked
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
    /// Creates a new, unlinked, tile
    pub fn new() -> Tile
    {
        Tile::Connected {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    /// Checks if a tile has any links available
    pub fn is_available(&self) -> bool
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
