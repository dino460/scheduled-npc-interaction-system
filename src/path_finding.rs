pub fn bfs_pathfinder(
	matrix : & Vec<Vec<usize>>, 
	source : (i32, i32), 
	destination : (i32, i32)
	) -> bool/*Vec<(i32, i32)>*/ {

	let mut queue : Vec<(i32, i32)> = vec![];

	queue.push(source);
	let mut matrix_editable : Vec<Vec<usize>> = matrix.clone();

	while !queue.is_empty() {
		let test_element : (i32, i32) = *queue.last().unwrap();
		println!("{:?}", queue.pop());

		let i = test_element.0;
		let j = test_element.1;

		if i >= 0 && i < matrix.len() as i32 && j >= 0 && j < matrix.len() as i32 {
			
			if matrix_editable[i as usize][j as usize] == 0 {
				continue;
			}

			if test_element == destination {
				return true;
			}

			matrix_editable[i as usize][j as usize] = 0;
			
			queue.push((i - 1, j));
			queue.push((i + 1, j)); 
			queue.push((i, j - 1)); 
			queue.push((i, j + 1));
		}
	}

	return false;	
}