extern crate rand;

use rand::Rng;

use super::Direction;
use super::MazeGenerator;
use super::super::grid::Grid;

pub struct BinaryTree;

impl MazeGenerator for BinaryTree
{
    fn generate<T: Grid>(grid: &mut T)
    {
        let (width, height) = grid.get_dimensions();

        if width == 0 || height == 0
        {
            return;
        }

        let mut dirs = vec![];

        for i in 0..width
        {
            for j in 0..height
            {
                if j < height - 1
                {
                    dirs.push(Direction::Up);
                }

                if i < width - 1
                {
                    dirs.push(Direction::Right);
                }

                if dirs.len() > 0
                {
                    let picked = rand::thread_rng().gen_range(0..dirs.len());
                    grid.carve_path((i, j), dirs[picked])
                        .expect("Failed to carve a path");
                    dirs.clear();
                }
            }
        }
    }
}
