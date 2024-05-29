use crate::gui::*;
use crate::World;
use crate::read_input;

use colored::Colorize;
use std::io::{Read, Write};
use std::fs::File;


pub fn save_world_to_file(
	world : &World, 
	mut input : String
) -> Result<String, std::io::Error> {
	let mut world_matrix_string : String = String::new();

    world.world.iter().for_each(|i| 
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
    println!("{} {}\n", INFO_SYMBOL.green(), ("File saved as '".to_owned() + &input + "'").green());

    return Result::Ok("ok".to_string());
}

pub fn load_world_from_file(
	world : &mut World,
	matrix_size : usize,
	mut input : String
) -> Result<String, std::io::Error> {
	input = match read_input::<String>(input.clone(), "", &("> Type the name of the file to load".to_string() + &" (with extension)".bright_yellow().to_string())) {
        Ok(value) => value,
        _         => return Ok("halt".to_string()),
    };

    if EXIT_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) { return Ok("halt".to_string()); }

    let file : Option<File> = match File::open(input.clone()) {
        Ok(file) => Some(file),
        _ => {
            println!("{} {}\n", ERROR_SYMBOL.red().blink(), format!("File '{}' does not exist.", input).red());
            None
        }
    };
    let mut contents : String = String::new();

    file.expect("REASON").read_to_string(&mut contents)?;

    let mut holder_matrix : Vec<Vec<usize>> = (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect();
    let mut i : usize = 0;
    let mut j : usize = 0;

    for line in contents.split("\n") {
        for num in line.split(" ") {
            if num == "" { continue; }
            holder_matrix[i][j] = num.parse::<usize>().unwrap();
            j += 1;
        }
        j = 0;
        i += 1;
        if i >= matrix_size { break }
    }

    world.world = holder_matrix.clone();

    println!("{} {}\n", INFO_SYMBOL.green(), ("File '".to_owned() + &input + "' successfully loaded").green());

    Ok("ok".to_string())
}
