use crate::generate_zero_world;
// use crate::print_matrix_with_path;
use crate::process_world_generation;
use crate::path_finding;
use crate::Point;
use crate::Benchmark;
use crate::World;

use std::time::Instant;


pub fn benchmark_all(
	world                 : &mut World,
	number_of_repetitions : usize,
	min_matrix_size		  : usize,
	max_matrix_size		  : usize,
	consider_diagonals    : bool,
	weight_distribution_weights : [usize; 4], 
    path_distribution_weights : [usize; 2], 
    job_distribution_weights : [usize; 2]
) -> Vec<Benchmark> { // Return is structure as 

	let mut benchmark_results : Vec<Benchmark> = vec![];

	for size in min_matrix_size..max_matrix_size {
		let mut benchmark : Benchmark = Benchmark { matrix_size: Some(size), building_time: Some(0.0), finding_time: Some(0.0) };
		let mut repetitions : isize = 0;

		loop {
    		// let mut path : Vec<(usize, usize)> = vec![];

            generate_zero_world(world, size);
            let mut now = Instant::now();
            process_world_generation(world, size, weight_distribution_weights, path_distribution_weights, job_distribution_weights);
            let mut elapsed = now.elapsed();
            benchmark.building_time = Some(benchmark.building_time.unwrap() + elapsed.as_secs_f32());

            let origin_dest_tuple = get_furthest_dest_from_origin(&world);
            let mut destination = Point { i: Some(origin_dest_tuple.1.0 as i32), j: Some(origin_dest_tuple.1.1 as i32), distance: None, previous: None };

            now = Instant::now();
            let path_exists : bool = path_finding::pathfinder(
                &world.world, 
                &world.weights,
                &Point { i: Some(origin_dest_tuple.0.0 as i32), j: Some(origin_dest_tuple.0.1 as i32), distance: None, previous: None }, 
                &mut destination,
                consider_diagonals,
                true
            );

            elapsed = now.elapsed();

            if path_exists {
            	benchmark.finding_time = Some(benchmark.finding_time.unwrap() + elapsed.as_secs_f32());
            	repetitions += 1;
        	}
            // let mut path_start = destination.previous.clone();

            // while path_start != None {
            //     path.push((path_start.clone().unwrap().i.unwrap() as usize, path_start.clone().unwrap().j.unwrap() as usize));
            //     path_start = path_start.unwrap().previous.clone();
            // }

            // print_matrix_with_path(&world.world, (destination.i.unwrap().try_into().unwrap(), destination.j.unwrap().try_into().unwrap()), (origin_dest_tuple.0.0, origin_dest_tuple.0.1), &path)
    		
    		if repetitions >= number_of_repetitions.try_into().unwrap() { break; }
		}
		benchmark.building_time = Some(benchmark.building_time.unwrap() / number_of_repetitions as f32);
		benchmark.finding_time = Some(benchmark.finding_time.unwrap() / number_of_repetitions as f32);

		benchmark_results.push(benchmark);

		if benchmark_results.last().unwrap().finding_time.unwrap() > 1.0/60.0 { break; }
	}

	return benchmark_results;
}

fn get_furthest_dest_from_origin(
	world : & World
) -> ((usize, usize), (usize, usize)) {

	let mut origin      : (usize, usize) = (0, 0);
	let mut destination : (usize, usize) = (0, 0);

	for i in 0..world.world.len() {
		for j in 0..world.world[i].len() {
			if world.world[i][j] != 0 {
				origin = (i, j);
			}
		}
	}

	for i in (0..world.world.len()).rev() {
		for j in (0..world.world[i].len()).rev() {
			if world.world[i][j] != 0 {
				destination = (i, j);
			}
		}
	}

	return (origin, destination);
}
