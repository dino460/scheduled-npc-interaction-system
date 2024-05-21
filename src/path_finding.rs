use std::collections::VecDeque;

fn bfs_pathfinder(
	matrix : & Vec<Vec<usize>>, 
	source : (usize, usize), 
	destination : (usize, usize)
	) -> Vec<(usize, usize)> {

	let mut shortest_path : Vec<(usize, usize)> = vec![];
	
	let mut visited_cells : Vec<Vec<bool>> = (0..matrix.len()).map(|_| { (0..matrix.len()).map(|_| false ).collect()}).collect();
	
	for i in 0..matrix.len() {
		for j in 0..matrix[i].len() {
			visited_cells[i][j] = if matrix[i][j] == 0 { true } else { false };
		}
	}

	let mut queue : Vec<(usize, usize)> = vec![];
	queue.push(source);

	while !queue.is_empty() {
		let coordinate_to_check : (usize, usize) = queue.pop().unwrap();
		
		// Destination found
		if coordinate_to_check == destination {
			return shortest_path;
		} 

		// // moving up
        // if (coordinate_to_check.0 - 1 >= 0 &&
        //     visited[p.row - 1][p.col] == false) {
        //     q.push(QItem(p.row - 1, p.col, p.dist + 1));
        //     visited[p.row - 1][p.col] = true;
        // }
 
        // // moving down
        // if (p.row + 1 < N &&
        //     visited[p.row + 1][p.col] == false) {
        //     q.push(QItem(p.row + 1, p.col, p.dist + 1));
        //     visited[p.row + 1][p.col] = true;
        // }
 
        // // moving left
        // if (p.col - 1 >= 0 &&
        //     visited[p.row][p.col - 1] == false) {
        //     q.push(QItem(p.row, p.col - 1, p.dist + 1));
        //     visited[p.row][p.col - 1] = true;
        // }
 
        //  // moving right
        // if (p.col + 1 < M &&
        //     visited[p.row][p.col + 1] == false) {
        //     q.push(QItem(p.row, p.col + 1, p.dist + 1));
        //     visited[p.row][p.col + 1] = true;
        // }
	}


	return shortest_path;
}