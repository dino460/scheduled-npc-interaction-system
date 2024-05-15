use rand::prelude::*;
use rand::distributions::WeightedIndex;


pub fn random_matrix_with_weights(weights : Vec<usize>, matrix_size : usize) -> Vec<Vec<usize>> {
	let mut rng = thread_rng();
	let weight_distribution = WeightedIndex::new(&weights).unwrap();

	let generated_matrix : Vec<Vec<usize>> = (0..matrix_size).map(|_| {
		(0..matrix_size).map(|_| weight_distribution.sample(&mut rng)).collect()
	}).collect();

	return generated_matrix;
}

pub fn smooth_binary_matrix_ones(matrix_to_smooth : & Vec<Vec<usize>>, number_of_smooth_passes : usize) -> Vec<Vec<usize>> {
	/*
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
    let mut result_matrix : Vec<Vec<usize>> = matrix_to_smooth.clone();
    let mut carrier_matrix : Vec<Vec<usize>> = matrix_to_smooth.clone();

    for _ in 0..number_of_smooth_passes {
        for i in 1..(matrix_to_smooth.len()-1) {
            for j in 1..(matrix_to_smooth.len()-1) {
                // Checks if connection can be made
                
                if result_matrix[i][j] == 0 && 
                ((result_matrix[i-1][j] == 1 && (result_matrix[i+1][j] == 1 || result_matrix[i][j+1] == 1 || result_matrix[i][j-1] == 1)) || 
                (result_matrix[i][j-1] == 1 && (result_matrix[i+1][j] == 1 || result_matrix[i][j+1] == 1)) ||
                (result_matrix[i+1][j] == 1 && result_matrix[i][j+1] == 1)) {
                	
                    carrier_matrix[i][j] = 1;
                }
            }
        }
        result_matrix = carrier_matrix.clone();
    }

    return result_matrix;
}

pub fn sum_matrices(matrix_1 : & Vec<Vec<usize>>, matrix_2 : & Vec<Vec<usize>>) -> Vec<Vec<usize>> {
	let mut result_matrix : Vec<Vec<usize>> = (0..matrix_1.len()).map(|_| {
		(0..matrix_1.len()).map(|_| 0).collect()
	}).collect();

	for i in 0..matrix_1.len() {
		for j in 0..matrix_1.len() {
			result_matrix[i][j] = matrix_1[i][j] + matrix_2[i][j];
		}
	}

	return result_matrix;
}