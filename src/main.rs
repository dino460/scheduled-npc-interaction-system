mod matrix_generator;
use colored::*;
use std::io::{self, Write};
use std::fs::File;


const MATRIX_SIZE : usize = 50;
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
        
            
            // * Print the generated matrices for better visualization
            println!();
            println!("{}", "World matrix");
            for i in 0..world.world.len() {
                for j in 0..world.world.len() {
                    match world.world[i][j] {
                        0 => print!("{} ", world.world[i][j].to_string().red()),
                        1 => print!("{} ", world.world[i][j].to_string().green()),
                        2 => print!("{} ", world.world[i][j].to_string().blue()),
                        3 => print!("{} ", world.world[i][j].to_string().yellow()),
                        4 => print!("{} ", world.world[i][j].to_string().magenta()),
                        _ => print!("{} ", world.world[i][j])
                    }
                }
                println!();
            }
            println!();

        } else if SAVE_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {
            
            let mut world_matrix_string : String = String::new();

            world.world.iter().for_each(|i| 
                { 
                    i.iter().for_each(|j| 
                        { 
                            world_matrix_string.push_str(&j.to_string());
                        });
                    world_matrix_string.push_str("\n");
                });

            // println!("{}", world_matrix_string);
            
            input.clear();
            io::stdin().read_line(&mut input).expect("failed to read line");
            input.pop();

            if EXIT_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) { break; }
            
            let mut file = File::create(input.clone())?;

            let _ = file.write_all(world_matrix_string.as_bytes());

            println!("File saved as {}", input);

        } else if EXIT_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {
            break;
        } else if CLEAR_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {
            std::process::Command::new("clear").status().unwrap();
        }
        else {
            println!("The '{}' command does not exist.", input);
        }
    }

    Ok(())
}
