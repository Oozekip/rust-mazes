use super::grid::Grid;
use super::Direction;
use super::Point;

mod recursive_backtracker;
mod binary_tree;
mod prims_algorithm;

pub use self::recursive_backtracker::RecursiveBacktracker;
pub use self::prims_algorithm::Prim;
pub use self::binary_tree::BinaryTree;

pub trait MazeGenerator
{
    fn generate<T: Grid>(&mut T);
}
