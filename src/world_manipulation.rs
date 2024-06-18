use crate::World;
use crate::matrix_generator::*;
use crate::SMOOTH_PASS_MAX;


pub fn process_world_generation(
    mut world : &mut World, 
    matrix_size : usize,
    weight_distribution_weights : [usize; 4], 
    path_distribution_weights : [usize; 2], 
    job_distribution_weights : [usize; 2],
) {
    generate_world(&mut world, matrix_size, weight_distribution_weights, path_distribution_weights, job_distribution_weights);

    /*
        Does the smoothing process in a number of passes
        Does not directly alter paths, only uses it as base
        smoothed_paths is the final matrix with the smoothing results
    */
    world.smoothed_paths = smooth_binary_matrix_ones(&world.paths, SMOOTH_PASS_MAX).clone();
    // Blocks diagonal connections that would technically pass through walls
    world.smoothed_paths = smooth_diagonals(& world.smoothed_paths).clone();
    // Adds results of smoothing into world matrix
    world.world = sum_matrices(&world.smoothed_paths, &world.world).clone();

    for i in 0..world.weights.len() {
        for j in 0..world.weights[i].len() {
            if world.world[i][j] != 0 {
                world.world[i][j] += world.weights[i][j];
            }
        }
    }
}

fn generate_world(
    world : &mut World, 
    matrix_size : usize,
    weight_distrubution_weights : [usize; 4],
    path_distribution_weights : [usize; 2],
    job_distribution_weights : [usize; 2]
) {
    *world = World {
        paths          : random_matrix_with_weights(path_distribution_weights.to_vec(), matrix_size),
        smoothed_paths : (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect(),
        jobs           : random_matrix_with_weights(job_distribution_weights.to_vec(), matrix_size),
        world          : (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect(),
        weights        : random_matrix_with_weights(weight_distrubution_weights.to_vec(), matrix_size)
    };
}


pub fn generate_zero_world(
    world : &mut World, 
    matrix_size : usize
) {
    *world = World {
        paths          : (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect(),
        smoothed_paths : (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect(),
        jobs           : (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect(),
        world          : (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect(),
        weights        : (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect()
    };
}