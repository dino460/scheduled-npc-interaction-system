mod matrix_generator;
use colored::*;


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

fn main() {

    //                              [0, 1]
    let path_distribution_weights = [2, 1];
    let job_distribution_weights  = [4, 1];

    let mut world : World = World {
        paths : matrix_generator::random_matrix_with_weights(path_distribution_weights.to_vec(), MATRIX_SIZE),
        altered_paths : (0..MATRIX_SIZE).map(|_| { (0..MATRIX_SIZE).map(|_| 0).collect()}).collect(),
        jobs : matrix_generator::random_matrix_with_weights(job_distribution_weights.to_vec(), MATRIX_SIZE),
        world : (0..MATRIX_SIZE).map(|_| { (0..MATRIX_SIZE).map(|_| 0).collect()}).collect()
    };

    /*
        Does the smoothing process in a predetermined number of passes
        Does not directly alter paths, only uses it as base
        altered_paths is the final matrix with the smoothing results
    */
    world.altered_paths = matrix_generator::smooth_binary_matrix_ones(&world.paths, SMOOTH_PASS_MAX);
    
    world.world = matrix_generator::sum_matrices(&world.altered_paths, &world.world);

    
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
}
