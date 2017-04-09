extern crate mazes;

use std::io;
use std::io::Write;

use mazes::generator::{MazeGenerator, Prim, RecursiveBacktracker, BinaryTree};
use mazes::grid::BasicGrid;

fn main()
{
    let width;
    let height;

    // Get width
    loop
    {
        print!("Enter the grid width: ");
        io::stdout().flush().expect("Failed to write output");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to write read input");

        if let Result::Ok(val) = input.trim().parse()
        {
            width = val;
            break;
        }
        else
        {
            println!("Invalid width.");
        }
    }

    // Get height
    loop
    {
        print!("Enter the grid height: ");
        io::stdout().flush().expect("Failed to write output");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to write read input");

        if let Result::Ok(val) = input.trim().parse()
        {
            height = val;
            break;
        }
        else
        {
            println!("Invalid height");
        }
    }

    let mut grid = BasicGrid::new(width, height);

    // Get generator type
    loop
    {
        println!("Chose a generator to use: ");
        println!("1: Recursive backtracker");
        println!("2: Binary Tree");
        println!("3: Prims Algorithm");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to write read input");

        if let Result::Ok(val) = input.trim().parse()
        {
            match val
            {
                1 => RecursiveBacktracker::generate(&mut grid),
                2 => BinaryTree::generate(&mut grid),
                3 => Prim::generate(&mut grid),
                _ =>
                {
                    println!("Invalid generator type.");
                    continue;
                }
            }

            break; // Only gets here if generation was successful
        }
        else
        {
            println!("Invalid generator type.");
        }
    }


    println!("{}", grid);
}
