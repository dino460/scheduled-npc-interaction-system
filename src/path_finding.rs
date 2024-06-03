use crate::structures::Point;
use std::rc::Rc;


pub fn pathfinder(
	matrix : & Vec<Vec<usize>>, 
	source : & Point, 
	destination : &mut Point,
	point_matrix : &mut Vec<Vec<Point>>,
	use_diagonals : bool
	) -> bool {

	let mut queue : Vec<Point> = vec![];

	let mut matrix_editable : Vec<Vec<usize>> = matrix.clone();

	for i in 0..matrix.len() {
		for j in 0..matrix[i].len() {
			point_matrix[i][j] = Point { 
				i : Some(i as i32), 
				j : Some(j as i32), 
				distance : if i == source.i.unwrap() as usize && j == source.j.unwrap() as usize {
							Some(0)
						} else {
							Some(1000000)
						},
				previous : None
			};

			if i == source.i.unwrap() as usize && j == source.j.unwrap() as usize { queue.push(point_matrix[i][j].clone()); }
		}
	}

	while !queue.is_empty() {
		let mut test_element : Point = queue.first().unwrap().clone();
		
		let mut index_of_chosen : usize = 0;

		for i in 0..queue.len() {
			if queue.get(0).unwrap().distance < test_element.distance { 
				test_element = queue.get(i).unwrap().clone();
				index_of_chosen = i;
			}
		}
		// println!("{} {} : {:?}", test_element.i.unwrap(), test_element.j.unwrap(), test_element.previous);
		
		queue.remove(index_of_chosen);

		if test_element.i.unwrap() == destination.i.unwrap() && test_element.j.unwrap() == destination.j.unwrap() {
			*destination = test_element;
			return true;
		}


		let i = test_element.i.unwrap();
		let j = test_element.j.unwrap();

		if i >= 0 && i < matrix.len() as i32 && j >= 0 && j < matrix.len() as i32 {
			
			if matrix_editable[i as usize][j as usize] == 0 {
				continue;
			}

			matrix_editable[i as usize][j as usize] = 0;

			if i > 0 {
				if test_element.distance.unwrap() + 1 < point_matrix[(i - 1) as usize][j as usize].distance.unwrap() {
					point_matrix[(i - 1) as usize][j as usize].distance = Some(test_element.distance.unwrap() + 1);
					point_matrix[(i - 1) as usize][j as usize].previous = Some(Rc::new(point_matrix[i as usize][j as usize].clone()));
				}
				queue.push(point_matrix[(i - 1) as usize][j as usize].clone());
			}
			if i + 1 < matrix.len() as i32 {
				if test_element.distance.unwrap() + 1 < point_matrix[(i + 1) as usize][j as usize].distance.unwrap() {
					point_matrix[(i + 1) as usize][j as usize].distance = Some(test_element.distance.unwrap() + 1);
					point_matrix[(i + 1) as usize][j as usize].previous = Some(Rc::new(point_matrix[i as usize][j as usize].clone()));
				}
				queue.push(point_matrix[(i + 1) as usize][j as usize].clone());
			}
			if j > 0 {
				if test_element.distance.unwrap() + 1 < point_matrix[i as usize][(j - 1) as usize].distance.unwrap() {
					point_matrix[i as usize][(j - 1) as usize].distance = Some(test_element.distance.unwrap() + 1);
					point_matrix[i as usize][(j - 1) as usize].previous = Some(Rc::new(point_matrix[i as usize][j as usize].clone()));
				}
				queue.push(point_matrix[i as usize][(j - 1) as usize].clone());
			}
			if j + 1 < matrix.len() as i32 {
				if test_element.distance.unwrap() + 1 < point_matrix[i as usize][(j + 1) as usize].distance.unwrap() {
					point_matrix[i as usize][(j + 1) as usize].distance = Some(test_element.distance.unwrap() + 1);
					point_matrix[i as usize][(j + 1) as usize].previous = Some(Rc::new(point_matrix[i as usize][j as usize].clone()));
				}
				queue.push(point_matrix[i as usize][(j + 1) as usize].clone());
			}

			if use_diagonals {
				if i > 0 && j > 0 {
					if test_element.distance.unwrap() + 1 < point_matrix[(i - 1) as usize][(j - 1) as usize].distance.unwrap() {
						point_matrix[(i - 1) as usize][(j - 1) as usize].distance = Some(test_element.distance.unwrap() + 1);
						point_matrix[(i - 1) as usize][(j - 1) as usize].previous = Some(Rc::new(point_matrix[i as usize][j as usize].clone()));
					}
					queue.push(point_matrix[(i - 1) as usize][(j - 1) as usize].clone());
				}
				if i > 0 && j + 1 < matrix.len() as i32 {
					if test_element.distance.unwrap() + 1 < point_matrix[(i - 1) as usize][(j + 1) as usize].distance.unwrap() {
						point_matrix[(i - 1) as usize][(j + 1) as usize].distance = Some(test_element.distance.unwrap() + 1);
						point_matrix[(i - 1) as usize][(j + 1) as usize].previous = Some(Rc::new(point_matrix[i as usize][(j + 1) as usize].clone()));
					}
					queue.push(point_matrix[(i - 1) as usize][(j + 1) as usize].clone());
				}
				if i + 1 < matrix.len() as i32 && j > 0 {
					if test_element.distance.unwrap() + 1 < point_matrix[(i + 1) as usize][(j - 1) as usize].distance.unwrap() {
						point_matrix[(i + 1) as usize][(j - 1) as usize].distance = Some(test_element.distance.unwrap() + 1);
						point_matrix[(i + 1) as usize][(j - 1) as usize].previous = Some(Rc::new(point_matrix[i as usize][j as usize].clone()));
					}
					queue.push(point_matrix[(i + 1) as usize][(j - 1) as usize].clone());
				}
				if i + 1 < matrix.len() as i32 && j + 1 < matrix.len() as i32 {
					if test_element.distance.unwrap() + 1 < point_matrix[(i + 1) as usize][(j + 1) as usize].distance.unwrap() {
						point_matrix[(i + 1) as usize][(j + 1) as usize].distance = Some(test_element.distance.unwrap() + 1);
						point_matrix[(i + 1) as usize][(j + 1) as usize].previous = Some(Rc::new(point_matrix[i as usize][j as usize].clone()));
					}
					queue.push(point_matrix[(i + 1) as usize][(j + 1) as usize].clone());
				}
			}
		}
	}

	return false;	
}