use rand::prelude::*;
use rand::distributions::WeightedIndex;
use colored::*;


#[derive(Clone, Copy, Debug)]
struct Job {
    job_type : usize
}

#[derive(Debug)]
struct World {
    paths : [[usize; 10]; 10],
    jobs  : [[Job; 10]; 10],
    world : [[usize; 10]; 10]
}


fn main() {
    let path_distribution_weights = [2, 1];
    let path_distribution = WeightedIndex::new(&path_distribution_weights).unwrap();
    
    let job_distribution_weights = [8, 1, 1, 1];
    let job_distribution = WeightedIndex::new(&job_distribution_weights).unwrap();

    let mut rng = thread_rng();


    let mut world : World = World {
        paths : [(); 10].map(|_| { 
                [(); 10].map(|_| path_distribution.sample(&mut rng) ) 
            }
        ),
        jobs : [(); 10].map(|_| {
                [(); 10].map(|_| Job { 
                        job_type : job_distribution.sample(&mut rng) 
                    }
                )
            }
        ),
        world : [[0; 10]; 10]
    };

    // i = line
    // j = column
    
    println!("\n{}", "World paths pre-patch");
    for i in 0..10 {
        for j in 0..10 {
            match world.paths[i][j] {
                0 => print!("{} ", world.paths[i][j].to_string().red()),
                1 => print!("{} ", world.paths[i][j].to_string().green()),
                2 => print!("{} ", world.paths[i][j].to_string().cyan()),
                _ => print!("{} ", world.paths[i][j])
            }
        }
        println!();
    }

    // * Patch paths to improve connectivity
    // Does the process in a predetermined number of passes

    let max_passes = 2;
    for _ in 0..max_passes {
        for i in 1..9 {
            for j in 1..9 {
                if world.paths[i][j] == 0 && (world.paths[i-1][j] == 1 || world.paths[i][j-1] == 1) && (world.paths[i+1][j] == 1 || world.paths[i][j+1] == 1) {
                    world.paths[i][j] = 1;
                }
            }
        }
    }

    // * Combine paths and jobs
    for i in 0..10 {
        for j in 0..10 {
            if world.paths[i][j] == 1 {
                world.world[i][j] = world.paths[i][j] + world.jobs[i][j].job_type;
            }
        }
    }


    // * Print the generated matrices for better visualization

    println!("\n{}", "World paths post-patch");
    for i in 0..10 {
        for j in 0..10 {
            match world.paths[i][j] {
                0 => print!("{} ", world.paths[i][j].to_string().red()),
                1 => print!("{} ", world.paths[i][j].to_string().green()),
                2 => print!("{} ", world.paths[i][j].to_string().cyan()),
                _ => print!("{} ", world.paths[i][j])
            }
        }
        println!();
    }
    
    println!();
    println!("{}", "World jobs");
    for i in 0..10 {
        for j in 0..10 {
            match world.jobs[i][j].job_type {
                0 => print!("{} ", world.jobs[i][j].job_type.to_string().red()),
                1 => print!("{} ", world.jobs[i][j].job_type.to_string().green()),
                2 => print!("{} ", world.jobs[i][j].job_type.to_string().cyan()),
                _ => print!("{} ", world.jobs[i][j].job_type)
            }
        }
        println!();
    }

    println!();
    println!("{}", "World matrix");
    for i in 0..10 {
        for j in 0..10 {
            match world.world[i][j] {
                0 => print!("{} ", world.world[i][j].to_string().red()),
                1 => print!("{} ", world.world[i][j].to_string().green()),
                2 => print!("{} ", world.world[i][j].to_string().cyan()),
                3 => print!("{} ", world.world[i][j].to_string().yellow()),
                4 => print!("{} ", world.world[i][j].to_string().blue()),
                5 => print!("{} ", world.world[i][j].to_string().magenta()),
                _ => print!("{} ", world.world[i][j])
            }
        }
        println!();
    }

    println!();
}
