use super::super::generator::BinaryTree;

use super::test_utils::test_gen;

/// Generates a normal sized maze
#[test]
fn gen_normal_maze()
{
    test_gen::<BinaryTree>(32, 32)
}

// Generates a huge maze
#[test]
fn gen_huge_maze()
{
    test_gen::<BinaryTree>(512, 512);
}

/// Generates an empty maze
#[test]
fn gen_empty_maze()
{
    test_gen::<BinaryTree>(0, 0)
}

/// Generates a very small maze
#[test]
fn gen_tiny_maze()
{
    test_gen::<BinaryTree>(1, 1);
}

/// Generates a rectangular maze
#[test]
fn gen_rectangular_maze()
{
    test_gen::<BinaryTree>(32, 16);
}
