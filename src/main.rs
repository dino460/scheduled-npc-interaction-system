mod matrix_generator;
mod path_finding;
use colored::*;
use std::io::{self, Write, Read};
use std::fs::File;


const MATRIX_SIZE : usize = 10;
const SMOOTH_PASS_MAX : usize = 2;

#[derive(Clone, Copy, Debug)]
struct Job {
    job_type : usize
}

#[derive(Debug)]
struct World {
    paths         : Vec<Vec<usize>>,
    altered_paths : Vec<Vec<usize>>,
    jobs          : Vec<Vec<usize>>,
    world         : Vec<Vec<usize>>
}

const GENERATE_INPUT_OPTIONS : [&str; 3] = ["generate", "gen", "g"];
const EXIT_INPUT_OPTIONS     : [&str; 3] = ["exit", "quit", "q"];
const SAVE_INPUT_OPTIONS     : [&str; 2] = ["save", "s"];
const LOAD_INPUT_OPTIONS     : [&str; 2] = ["load", "l"];
const CLEAR_INPUT_OPTIONS    : [&str; 2] = ["clear", "clr"];
const PRINT_INPUT_OPTIONS    : [&str; 2] = ["print", "p"];
const FIND_INPUT_OPTIONS     : [&str; 2] = ["find", "f"];



fn main() -> std::io::Result<()>{

    //                              [0, 1]
    let job_distribution_weights  = [4, 1];
    let path_distribution_weights = [2, 1];
    
    let mut input : String = "".to_string();
    
    let mut world : World = World {
            paths : (0..MATRIX_SIZE).map(|_| { (0..MATRIX_SIZE).map(|_| 0).collect()}).collect(),
            altered_paths : (0..MATRIX_SIZE).map(|_| { (0..MATRIX_SIZE).map(|_| 0).collect()}).collect(),
            jobs : (0..MATRIX_SIZE).map(|_| { (0..MATRIX_SIZE).map(|_| 0).collect()}).collect(),
            world : (0..MATRIX_SIZE).map(|_| { (0..MATRIX_SIZE).map(|_| 0).collect()}).collect()
        };
 
    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("failed to read line");
        input.pop();


        if GENERATE_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {
            world = World {
                paths : matrix_generator::random_matrix_with_weights(path_distribution_weights.to_vec(), MATRIX_SIZE),
                altered_paths : (0..MATRIX_SIZE).map(|_| { (0..MATRIX_SIZE).map(|_| 0).collect()}).collect(),
                jobs : matrix_generator::random_matrix_with_weights(job_distribution_weights.to_vec(), MATRIX_SIZE),
                world : (0..MATRIX_SIZE).map(|_| { (0..MATRIX_SIZE).map(|_| 0).collect()}).collect()
            };
        
            /*
                Does the smoothing process in a number of passes
                Does not directly alter paths, only uses it as base
                altered_paths is the final matrix with the smoothing results
            */
            world.altered_paths = matrix_generator::smooth_binary_matrix_ones(&world.paths, SMOOTH_PASS_MAX).clone();
            
            world.world = matrix_generator::sum_matrices(&world.altered_paths, &world.world).clone();

            print_matrix(&world.world);

        } else if SAVE_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {
            
            let mut world_matrix_string : String = String::new();

            world.world.iter().for_each(|i| 
                { 
                    i.iter().for_each(|j| 
                        { 
                            world_matrix_string.push_str(&j.to_string());
                            world_matrix_string.push_str(" ");
                        });
                    world_matrix_string.push_str("\n");
                });
            
            input.clear();
            io::stdin().read_line(&mut input).expect("failed to read line");
            input.pop();

            if EXIT_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) { break; }
            
            let mut file = File::create(input.clone())?;

            let _ = file.write_all(world_matrix_string.as_bytes());

            println!("File saved as {}", input);

        } else if LOAD_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {

            input.clear();
            io::stdin().read_line(&mut input).expect("failed to read line");
            input.pop();

            if EXIT_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) { break; }

            let mut file = File::open(input.clone())?;

            let mut contents : String = String::new();

            file.read_to_string(&mut contents)?;

            let mut holder_matrix : Vec<Vec<usize>> = (0..MATRIX_SIZE).map(|_| { (0..MATRIX_SIZE).map(|_| 0).collect()}).collect();
            let mut i : usize = 0;
            let mut j : usize = 0;

            for line in contents.split("\n") {
                for num in line.split(" ") {
                    if num == "" { continue; }
                    holder_matrix[i][j] = num.parse::<usize>().unwrap();
                    j += 1;
                }
                j = 0;
                i += 1;
                if i >= MATRIX_SIZE { break }
            }

            world.world = holder_matrix.clone();

            println!("File successfully loaded");

        } else if FIND_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {
            
            input.clear();
            io::stdin().read_line(&mut input).expect("failed to read line");
            input.pop();
            let x_source = input.parse::<usize>().unwrap();

            input.clear();
            io::stdin().read_line(&mut input).expect("failed to read line");
            input.pop();
            let y_source = input.parse::<usize>().unwrap();

            world.world[x_source][y_source] = 2;

            input.clear();
            io::stdin().read_line(&mut input).expect("failed to read line");
            input.pop();
            let x_dest = input.parse::<usize>().unwrap();

            input.clear();
            io::stdin().read_line(&mut input).expect("failed to read line");
            input.pop();
            let y_dest = input.parse::<usize>().unwrap();

            world.world[x_dest][y_dest] = 3;

            print_matrix(&world.world);

            println!("{}", path_finding::bfs_pathfinder(&world.world, (x_source, y_source, 0), (x_dest, y_dest, 0)));
        
        } else if CLEAR_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {
            std::process::Command::new("clear").status().unwrap();
        } else if PRINT_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {
            print_matrix(&world.world);
        } else if EXIT_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {
            break;
        }
        else {
            println!("The '{}' command does not exist.", input);
        }
    }

    Ok(())
}

fn print_matrix(matrix : &Vec<Vec<usize>>) {
    // * Print the generated matrices for better visualization
    println!();
    println!("{}", "World matrix");
    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            match matrix[i][j] {
                0 => print!("{} ", matrix[i][j].to_string().red()),
                1 => print!("{} ", matrix[i][j].to_string().green()),
                2 => print!("{} ", matrix[i][j].to_string().blue()),
                3 => print!("{} ", matrix[i][j].to_string().yellow()),
                4 => print!("{} ", matrix[i][j].to_string().magenta()),
                _ => print!("{} ", matrix[i][j])
            }
        }
        println!();
    }
    println!();
}
