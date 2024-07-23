use crate::generate_zero_world;
use crate::gui::*;
use crate::World;
use crate::read_input;

use colored::Colorize;
use std::io::{Read, Write};
use std::fs::File;


pub fn save_world_to_file(
	world : &World, 
	mut input : String,
    matrix_size : usize
) -> Result<String, std::io::Error> {
	let mut world_matrix_string : String = String::new();

    world_matrix_string.push_str(&matrix_size.to_string());
    world_matrix_string.push_str("\n");

    world.world.iter().for_each(|i| 
        { 
            i.iter().for_each(|j| 
                { 
                    world_matrix_string.push_str(&j.to_string());
                    world_matrix_string.push_str(" ");
                });
            world_matrix_string.push_str("\n");
        });
    world.paths.iter().for_each(|i| 
        { 
            i.iter().for_each(|j| 
                { 
                    world_matrix_string.push_str(&j.to_string());
                    world_matrix_string.push_str(" ");
                });
            world_matrix_string.push_str("\n");
        });
    world.jobs.iter().for_each(|i| 
        { 
            i.iter().for_each(|j| 
                { 
                    world_matrix_string.push_str(&j.to_string());
                    world_matrix_string.push_str(" ");
                });
            world_matrix_string.push_str("\n");
        });
    world.weights.iter().for_each(|i| 
        { 
            i.iter().for_each(|j| 
                { 
                    world_matrix_string.push_str(&j.to_string());
                    world_matrix_string.push_str(" ");
                });
            world_matrix_string.push_str("\n");
        });
    
    input = match read_input::<String>(input.clone(), "", &("> Type the name of the file to save".to_string() + &" (with extension)".bright_yellow().to_string())) {
        Ok(value) => value,
        _         => return Result::Ok("halt".to_string())	,
    };

    if EXIT_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) { return Result::Ok("halt".to_string()); }
    
    let mut file = File::create(input.clone())?;
    let _ = file.write_all(world_matrix_string.as_bytes());
    println!("{} {}\n", 
        INFO_SYMBOL.truecolor(WARNING_COLOR_LUT[2].0, WARNING_COLOR_LUT[2].1, WARNING_COLOR_LUT[2].2), 
        ("File saved as '".to_owned() + &input + "'").truecolor(WARNING_COLOR_LUT[2].0, WARNING_COLOR_LUT[2].1, WARNING_COLOR_LUT[2].2));

    return Result::Ok("ok".to_string());
}

pub fn load_world_from_file(
	world       : &mut World,
	matrix_size : &mut usize,
	mut input   : String
) -> Result<String, std::io::Error> {

	input = match read_input::<String>(input.clone(), "", &("> Type the name of the file to load".to_string() + &" (with extension)".bright_yellow().to_string())) {
        Ok(value) => value,
        _         => return Ok("halt".to_string()),
    };

    if EXIT_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) { return Ok("halt".to_string()); }

    let file : Option<File> = match File::open(input.clone()) {
        Ok(file) => Some(file),
        _ => {
            println!("{} {}\n", 
                ERROR_SYMBOL.truecolor(WARNING_COLOR_LUT[0].0, WARNING_COLOR_LUT[0].1, WARNING_COLOR_LUT[0].2).blink(), 
                format!("File '{}' does not exist.", input).truecolor(WARNING_COLOR_LUT[0].0, WARNING_COLOR_LUT[0].1, WARNING_COLOR_LUT[0].2));
            None
        }
    };
    let mut contents : String = String::new();
    file.expect("REASON").read_to_string(&mut contents)?;
    
    let mut content_vec : Vec<&str>;

    if cfg!(windows) {
        content_vec = contents.split("\r\n").collect();
    } else {
        content_vec = contents.split("\n").collect();    
    }
    

    *matrix_size = content_vec[0].parse::<usize>().unwrap();
    content_vec.remove(0);

    generate_zero_world(world, *matrix_size);

    // let mut holder_matrix : Vec<Vec<usize>> = (0..*matrix_size).map(|_| { (0..*matrix_size).map(|_| 0).collect()}).collect();
    let mut i : usize = 0;
    let mut j : usize = 0;
    let mut file_position : usize = 0;

    for line in content_vec {
        for num in line.split(" ") {
            if num == "" { continue; }

            if i >= *matrix_size { 
                file_position += i; 
                i = 0;
            }

            if file_position < *matrix_size { 
                world.world[i][j] = num.parse::<usize>().unwrap();
            } else if file_position < *matrix_size * 2 {
                world.paths[i][j] = num.parse::<usize>().unwrap();
            } else if file_position < *matrix_size * 3 {
                world.jobs[i][j] = num.parse::<usize>().unwrap();
            } else if file_position < *matrix_size * 4 {
                world.weights[i][j] = num.parse::<usize>().unwrap();
            } else {
                break;
            }

            j += 1;
        }

        j = 0;
        i += 1;
    }

    // world.world = holder_matrix.clone();

    println!("{} {}\n", 
        INFO_SYMBOL.truecolor(WARNING_COLOR_LUT[2].0, WARNING_COLOR_LUT[2].1, WARNING_COLOR_LUT[2].2), 
        ("File '".to_owned() + &input + "' successfully loaded").truecolor(WARNING_COLOR_LUT[2].0, WARNING_COLOR_LUT[2].1, WARNING_COLOR_LUT[2].2));

    Ok("ok".to_string())
}
