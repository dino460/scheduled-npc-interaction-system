use rand::prelude::*;
use rand::distributions::WeightedIndex;
use colored::*;


const matrix_size : usize = 20;

#[derive(Clone, Copy, Debug)]
struct Job {
    job_type : usize
}

#[derive(Debug)]
struct World {
    paths : [[usize; matrix_size]; matrix_size],
    paths_changeable : [[usize; matrix_size]; matrix_size],
    jobs  : [[Job; matrix_size]; matrix_size],
    world : [[usize; matrix_size]; matrix_size]
}


fn main() {

    let path_distribution_weights = [2, 1];
    let path_distribution = WeightedIndex::new(&path_distribution_weights).unwrap();
    
    let job_distribution_weights = [4, 1];
    let job_distribution = WeightedIndex::new(&job_distribution_weights).unwrap();

    let mut rng = thread_rng();


    let mut world : World = World {
        paths : [(); matrix_size].map(|_| { 
                [(); matrix_size].map(|_| path_distribution.sample(&mut rng)) 
            }
        ),
        paths_changeable : [[0; matrix_size]; matrix_size],
        jobs : [(); matrix_size].map(|_| {
                [(); matrix_size].map(|_| Job { 
                        job_type : job_distribution.sample(&mut rng) 
                    }
                )
            }
        ),
        world : [[0; matrix_size]; matrix_size]
    };

    world.world = world.paths.clone(); // Cloning path data into final world matrix
    world.paths_changeable = world.paths.clone();

    // i = line
    // j = column


    // * Patch paths to improve connectivity
    /*
        Does the process in a predetermined number of passes
        Does not directly alter paths, only uses it as base
        paths_changeable is the direct reference, being modified each pass/iteration
        world is the final matrix with the results

        A connection is created when there are at least two directly adjacent 1's
        
        0 1 0    0 1 0
        0 0 1 -> 0 1 1  creates connection
        0 0 0    0 0 0

        0 1 0    0 1 0
        0 0 0 -> 0 1 0  creates connection
        0 1 0    0 1 0

        0 0 1    0 0 1
        1 0 0 -> 1 0 0  doesn't create connection
        0 0 0    0 0 0
    */
    let max_passes = 2;
    for _ in 0..max_passes {
        for i in 1..(matrix_size-1) {
            for j in 1..(matrix_size-1) {
                // Checks if connection can be made
                
                if world.paths_changeable[i][j] == 0 && 
                ((world.paths_changeable[i-1][j] == 1 && (world.paths_changeable[i+1][j] == 1 || world.paths_changeable[i][j+1] == 1 || world.paths_changeable[i][j-1] == 1)) || 
                (world.paths_changeable[i][j-1] == 1 && (world.paths_changeable[i+1][j] == 1 || world.paths_changeable[i][j+1] == 1)) ||
                (world.paths_changeable[i+1][j] == 1 && world.paths_changeable[i][j+1] == 1)) {
                    // Create connection in world matrix
                    world.world[i][j] = 1;
                }
            }
        }
        // Writes changes to the modifiable paths matrix
        // Is done this way to avoid connections from happening from other connections this iteration
        world.paths_changeable = world.world.clone();
    }
    
    // * Combine world and jobs
    for i in 0..matrix_size {
        for j in 0..matrix_size {
            if world.paths[i][j] == 1 {
                world.world[i][j] = world.paths[i][j] + world.jobs[i][j].job_type;
            }
        }
    }

    
    // * Print the generated matrices for better visualization
    println!();
    println!("{}", "World matrix");
    for i in 0..matrix_size {
        for j in 0..matrix_size {
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
