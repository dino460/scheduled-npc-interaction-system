use crate::World;
use colored::*;
use std::time::Instant;
use crate::matrix_generator::*;
use crate::SMOOTH_PASS_MAX;
use crate::gui::*;


pub fn process_world_generation(
    mut world : &mut World, 
    matrix_size : usize, 
    path_distribution_weights : [usize; 2], 
    job_distribution_weights : [usize; 2],
    print_generation_result : bool
) {
    let now = Instant::now();
    generate_world(&mut world, matrix_size, path_distribution_weights, job_distribution_weights);
    /*
        Does the smoothing process in a number of passes
        Does not directly alter paths, only uses it as base
        altered_paths is the final matrix with the smoothing results
    */
    world.altered_paths = smooth_binary_matrix_ones(&world.paths, SMOOTH_PASS_MAX).clone();
    world.world = sum_matrices(&world.altered_paths, &world.world).clone();
    let elapsed = now.elapsed();

    if print_generation_result { print_matrix(&world.world); }
    println!("{} {} {} s", INFO_SYMBOL.green(), "Generated successfully in".green(), elapsed.as_secs_f32().to_string().bright_yellow());
    println!();
}

fn generate_world(
    world : &mut World, 
    matrix_size : usize,
    path_distribution_weights : [usize; 2],
    job_distribution_weights : [usize; 2]
) {
    *world = World {
        paths : random_matrix_with_weights(path_distribution_weights.to_vec(), matrix_size),
        altered_paths : (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect(),
        jobs : random_matrix_with_weights(job_distribution_weights.to_vec(), matrix_size),
        world : (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect()
    };
}

pub fn generate_zero_world(
    world : &mut World, 
    matrix_size : usize
) {
    *world = World {
        paths : (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect(),
        altered_paths : (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect(),
        jobs : (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect(),
        world : (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect()
    };
}