use super::super::generator::RecursiveBacktracker;

use super::test_utils::test_gen;

/// Generates a normal sized maze
#[test]
fn gen_normal_maze()
{
    test_gen::<RecursiveBacktracker>(32, 32)
}

/// Generates an empty maze
#[test]
fn gen_empty_maze()
{
    test_gen::<RecursiveBacktracker>(0, 0)
}

// Generates a huge maze
#[test]
fn gen_huge_maze()
{
    test_gen::<RecursiveBacktracker>(512, 512);
}

/// Generates a very small maze
#[test]
fn gen_tiny_maze()
{
    test_gen::<RecursiveBacktracker>(1, 1);
}

/// Generates a rectangular maze
#[test]
fn gen_rectangular_maze()
{
    test_gen::<RecursiveBacktracker>(32, 16);
}
