use std::collections::VecDeque;

pub fn bfs_pathfinder(
	matrix : & Vec<Vec<usize>>, 
	source : (usize, usize, i32), 
	destination : (usize, usize, i32)
	) -> i32/*Vec<(usize, usize)>*/ {

	let mut shortest_path : Vec<(usize, usize)> = vec![];
	
	let mut visited_cells : Vec<Vec<bool>> = (0..matrix.len()).map(|_| { (0..matrix.len()).map(|_| false ).collect()}).collect();
	
	for i in 0..matrix.len() {
		for j in 0..matrix[i].len() {
			visited_cells[i][j] = if matrix[i][j] == 0 { true } else { false };
		}
	}

	let mut queue : Vec<(usize, usize, i32)> = vec![];
	queue.push(source);

	while !queue.is_empty() {
		let coordinate_to_check : (usize, usize, i32) = *queue.last().unwrap();
		queue.pop();

		/*
		00 01 02
		10 11 12
		20 21 22

		00
		-
		10 01
		( 01 )
		10 11 02
		( 02 )
		10 11 12
		( 12 )
		10 11 22
		( 22 )
		10 

		*/
		
		// Destination found
		if coordinate_to_check.0 == destination.0 &&
		   coordinate_to_check.1 == destination.1 {
			return coordinate_to_check.2;
		} 

		// moving up
        if coordinate_to_check.0 as isize - 1 >= 0 &&
            visited_cells[coordinate_to_check.0 - 1][coordinate_to_check.1] == false {
            queue.push((coordinate_to_check.0 - 1, coordinate_to_check.1, coordinate_to_check.2 + 1));
            visited_cells[coordinate_to_check.0 - 1][coordinate_to_check.1] = true;
        }
 
        // moving down
        if coordinate_to_check.0 + 1 < matrix.len() &&
            visited_cells[coordinate_to_check.0 + 1][coordinate_to_check.1] == false {
            queue.push((coordinate_to_check.0 + 1, coordinate_to_check.1, coordinate_to_check.2 + 1));
            visited_cells[coordinate_to_check.0 + 1][coordinate_to_check.1] = true;
        }
 
        // moving left
        if coordinate_to_check.1 as isize - 1 >= 0 &&
            visited_cells[coordinate_to_check.0][coordinate_to_check.1 - 1] == false {
            queue.push((coordinate_to_check.0, coordinate_to_check.1 - 1, coordinate_to_check.2 + 1));
            visited_cells[coordinate_to_check.0][coordinate_to_check.1 - 1] = true;
        }
 
         // moving right
        if coordinate_to_check.1 + 1 < matrix.len() &&
            visited_cells[coordinate_to_check.0][coordinate_to_check.1 + 1] == false {
            queue.push((coordinate_to_check.0, coordinate_to_check.1 + 1, coordinate_to_check.2 + 1));
            visited_cells[coordinate_to_check.0][coordinate_to_check.1 + 1] = true;
        }
	}


	return -1;
}