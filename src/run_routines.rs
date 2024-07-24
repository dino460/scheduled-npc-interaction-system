use core::time::Duration;
use std::time::Instant;
use std::thread;

use colored::Colorize;

use crate::gui;
use crate::structures::NPC;

pub fn run(
	matrix 			  : &Vec<Vec<usize>>,
	npcs_to_run       : &mut Vec<NPC>,
	fps 			  : i32
) {

	let mut end_loop : bool = false;


	while !end_loop {
		let /*mut*/ sleep_duration : Duration = Duration::from_millis((1.0/(fps as f32) * 1000.0) as u64);
		let now = Instant::now();
		for npc in &mut *npcs_to_run {
			gui::clear_screen();
			if npc_can_run(npc, fps) {
				if npc.position > 0 {
					npc.position -= 1;
					npc.path.pop();
				} else {
					end_loop = true;
				}
			}
			let matrix_clone = matrix.clone();
			let npc_path_clone = npc.path.clone();
			let npc_path_position_clone = npc.path[npc.position].clone();
			let npc_path_destination_clone = npc.path[0].clone();
			thread::spawn(move || { 
				gui::print_matrix_with_path(&matrix_clone, npc_path_position_clone, npc_path_destination_clone, &npc_path_clone) 
			});
		
			// println!("{:?}", npc.path[npc.position]);

			// sleep_duration -= npc.pathfinding_time;
		}
		let elapsed = now.elapsed();
		// println!("{:?} | {:?}", elapsed, sleep_duration);
		
		if sleep_duration > Duration::new(0, 0) { thread::sleep(sleep_duration - elapsed) };
	}

}

pub fn npc_can_run(
	npc : &mut NPC,
	fps        : i32
) -> bool {

	npc.timer += 1.0 / (fps as f32);

	if npc.timer >= (1.0 / npc.speed) {
		npc.timer = 0.0;
		return true;
	}

	false
}
