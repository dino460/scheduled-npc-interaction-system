mod matrix_generator;
mod path_finding;
mod gui;
mod structures;
mod file_manipulation;
mod world_manipulation;
mod tests;

use tests::*;
use gui::*;
use structures::*;
use file_manipulation::*;
use world_manipulation::*;

use std::io::{self, stdout, Write};
use std::time::Instant;
use colored::*;


const SMOOTH_PASS_MAX : usize = 2;


fn main() -> std::io::Result<()>{

    let mut matrix_size : usize = 50;
    let mut print_generation_result : bool = true;
    let mut show_commands : bool = true;

    let mut source      : (usize, usize) = (0, 0);
    let mut destination : (usize, usize) = (0, 0);
    let mut path        : Vec<(usize, usize)> = vec![];


    //TODO: Make it possible to change weights during runtime
    //                              [0, 1]
    let job_distribution_weights  = [4, 1];
    let path_distribution_weights = [2, 1];

    let weight_distribution_weights = [4, 3, 2, 1];
    
    let mut input : String = "".to_string();
    
    let mut world : World = World {
            paths          : (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect(),
            smoothed_paths : (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect(),
            jobs           : (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect(),
            world          : (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect(),
            weights        : (0..matrix_size).map(|_| { (0..matrix_size).map(|_| 0).collect()}).collect()
        };

    print_title();
    print_greeting_tutorial();

    loop {
        let hide_gen_indicator = if print_generation_result { 
            print_generation_result.to_string().truecolor(WARNING_COLOR_LUT[2].0, WARNING_COLOR_LUT[2].1, WARNING_COLOR_LUT[2].2).to_string() 
        } else { 
            print_generation_result.to_string().truecolor(WARNING_COLOR_LUT[0].0, WARNING_COLOR_LUT[0].1, WARNING_COLOR_LUT[0].2).to_string() 
        };
        let command_indication = if show_commands { 
            show_commands.to_string().truecolor(WARNING_COLOR_LUT[2].0, WARNING_COLOR_LUT[2].1, WARNING_COLOR_LUT[2].2).to_string() 
        } else { 
            show_commands.to_string().truecolor(WARNING_COLOR_LUT[0].0, WARNING_COLOR_LUT[0].1, WARNING_COLOR_LUT[0].2).to_string() 
        };

        input = match read_input::<String>(
            input.clone(), 
            &("[".to_string() + &hide_gen_indicator + "|" + &command_indication + "|" + &matrix_size.to_string() + "] "), 
            "<<< Type you command >>>".cyan().to_string().as_str()) 
        {
            Ok(value)  => value,
            Err(value) => value.to_string()
        };

        if GENERATE_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {

            path.clear();
            destination = (0, 0);
            source = (0, 0);


            let now = Instant::now(); 
            process_world_generation(&mut world, matrix_size, weight_distribution_weights, path_distribution_weights, job_distribution_weights);
            let elapsed = now.elapsed();


            if print_generation_result { print_matrix_with_path(&world.world, destination, source, &path) }
            println!("{} {} {} s", 
                INFO_SYMBOL.truecolor(WARNING_COLOR_LUT[2].0, WARNING_COLOR_LUT[2].1, WARNING_COLOR_LUT[2].2), 
                "Generated successfully in".truecolor(WARNING_COLOR_LUT[2].0, WARNING_COLOR_LUT[2].1, WARNING_COLOR_LUT[2].2), 
                elapsed.as_secs_f32().to_string().bright_yellow());
            println!();

        } else if SAVE_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {
            
            let save_result = save_world_to_file(&world, input.clone(), matrix_size);

            match save_result {
                Ok(value) => {
                    if value == "halt" { continue; }
                },
                Err(value) => {
                    println!("{}", value);
                    break;
                }
            }

        } else if LOAD_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {

            let load_result = load_world_from_file(&mut world, &mut matrix_size, input.clone());

            match load_result {
                Ok(value) => {
                    if value == "halt" { continue; }
                },
                Err(value) => {
                    println!("{}", value);
                    break;
                }
            }

        } else if FIND_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {

            path.clear();

            source.0 = match read_input::<usize>(input.clone(), "\t", "\t> Source X coordinate value:".yellow().to_string().as_str()) {
                Ok(value) => value,
                _         => continue,
            };
            if source.0 >= matrix_size {
                println!("\t{} {} 0..{}", 
                    WARNING_SYMBOL.truecolor(WARNING_COLOR_LUT[1].0, WARNING_COLOR_LUT[1].1, WARNING_COLOR_LUT[1].2).blink(), 
                    "Value out of bounds".truecolor(WARNING_COLOR_LUT[1].0, WARNING_COLOR_LUT[1].1, WARNING_COLOR_LUT[1].2), 
                    matrix_size - 1);
                continue;
            }
            println!();

            source.1 =  match read_input::<usize>(input.clone(), "\t", "\t> Source Y coordinate value:".yellow().to_string().as_str()) {
                Ok(value) => value,
                _         => continue,
            };
            if source.1 >= matrix_size {
                println!("\t{} {} 0..{}", 
                    WARNING_SYMBOL.truecolor(WARNING_COLOR_LUT[1].0, WARNING_COLOR_LUT[1].1, WARNING_COLOR_LUT[1].2).blink(), 
                    "Value out of bounds".truecolor(WARNING_COLOR_LUT[1].0, WARNING_COLOR_LUT[1].1, WARNING_COLOR_LUT[1].2), 
                    matrix_size - 1);
                continue;
            }
            println!();

            destination.0 = match read_input::<usize>(input.clone(), "\t", "\t> Destination X coordinate value:".yellow().to_string().as_str()) {
                Ok(value) => value,
                _         => continue,
            };
            if destination.0 >= matrix_size {
                println!("\t{} {} 0..{}", 
                    WARNING_SYMBOL.truecolor(WARNING_COLOR_LUT[1].0, WARNING_COLOR_LUT[1].1, WARNING_COLOR_LUT[1].2).blink(), 
                    "Value out of bounds".truecolor(WARNING_COLOR_LUT[1].0, WARNING_COLOR_LUT[1].1, WARNING_COLOR_LUT[1].2), 
                    matrix_size - 1);
                continue;
            }
            println!();

            destination.1 = match read_input::<usize>(input.clone(), "\t", "\t> Destination Y coordinate value:".yellow().to_string().as_str()) {
                Ok(value) => value,
                _         => continue,
            };
            if destination.0 >= matrix_size {
                println!("\t{} {} 0..{}", 
                    WARNING_SYMBOL.truecolor(WARNING_COLOR_LUT[1].0, WARNING_COLOR_LUT[1].1, WARNING_COLOR_LUT[1].2).blink(), 
                    "Value out of bounds".truecolor(WARNING_COLOR_LUT[1].0, WARNING_COLOR_LUT[1].1, WARNING_COLOR_LUT[1].2), 
                    matrix_size - 1);
                continue;
            }
            println!();

            let mut dest = Point { i: Some(destination.0 as i32), j: Some(destination.1 as i32), distance: None, previous: None };

            let use_weights : bool = match read_input_to_bool(input.clone(), "\t", &("\t> Use weights? [y, yes | n, no]".yellow().to_string())) {
                Ok(value) => value,
                _         => continue,
            };
            println!();

            let use_diagonals : bool = match read_input_to_bool(input.clone(), "\t", &("\t> Compute diagonal movement? [y, yes | n, no]".yellow().to_string() + " (High performance impact)".red().to_string().as_str())) {
                Ok(value) => value,
                _         => continue,
            };
            println!();


            let now = Instant::now();
            let path_exists = path_finding::pathfinder(
                &world.world, 
                &world.weights,
                &Point { i: Some(source.0 as i32), j: Some(source.1 as i32), distance: None, previous: None }, 
                &mut dest,
                use_diagonals,
                use_weights,
                1.0
            );
            let elapsed = now.elapsed();

            println!("Path from ({}, {}) to ({}, {}) : {}", 
                source.0, source.1,
                destination.0, destination.1,
                if path_exists { 
                    path_exists.to_string().truecolor(WARNING_COLOR_LUT[2].0, WARNING_COLOR_LUT[2].1, WARNING_COLOR_LUT[2].2) 
                } else { 
                    path_exists.to_string().truecolor(WARNING_COLOR_LUT[0].0, WARNING_COLOR_LUT[0].1, WARNING_COLOR_LUT[0].2)
                }
            );

            println!("Time to process: {} s", elapsed.as_secs_f32().clamp(0.0001, 1000.0).to_string().bright_yellow());

            let mut path_start = dest.previous.clone();

            while path_start != None {
                path.push((path_start.clone().unwrap().i.unwrap() as usize, path_start.clone().unwrap().j.unwrap() as usize));
                path_start = path_start.unwrap().previous.clone();
            }
        
        } else if SET_SIZE_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {

            matrix_size = read_input::<usize>(input.clone(), "", "> Type matrix size (matrix is always NxN)").unwrap(); /*input.parse::<usize>().unwrap()*/

            generate_zero_world(&mut world, matrix_size);

            path.clear();
            destination = (0, 0);
            source = (0, 0);

        } else if BENCHMARK_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {

            let now = Instant::now();
            let benchmarks : Vec<Benchmark> = benchmark_all(&mut world, 2000, 50, 51, false, 0.6, 0.7, 0.01, weight_distribution_weights, path_distribution_weights, job_distribution_weights);
            let elapsed = now.elapsed();

            println!("\nTotal benchmark time: {}\n", elapsed.as_secs_f32().clamp(0.0001, 1000.0));
            for benchmark in benchmarks {
                println!("{}\n", benchmark);
            }
            println!();

            matrix_size = world.world.len();

        } else if SHOW_ALL_COMMANDS_OPTIONS.contains(&input.to_lowercase().as_str()) {
            print_all_commands();
        } else if RESET_SCREEN_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {
            print_title();
            if show_commands { print_greeting_tutorial(); }
        } else if input.to_lowercase() == "hide all" {
            show_commands = false;
            print_generation_result = false;
        } else if HIDE_COMMAND_OPTIONS.contains(&input.to_lowercase().as_str()) {
            show_commands = !show_commands;
        }else if HIDE_GEN_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {
            print_generation_result = !print_generation_result;
        } else if CLEAR_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {
            clear_screen();
        } else if PRINT_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {
            print_matrix_with_path(&world.world, destination, source, &path);
        } else if EXIT_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) {
            println!("\n{}", "Exiting...\n".truecolor(WARNING_COLOR_LUT[0].0, WARNING_COLOR_LUT[0].1, WARNING_COLOR_LUT[0].2).bold());
            let _ = stdout().flush();
            break;
        }
        else {
            println!("{}{}{}{}", 
                WARNING_SYMBOL.truecolor(WARNING_COLOR_LUT[1].0, WARNING_COLOR_LUT[1].1, WARNING_COLOR_LUT[1].2).blink(), 
                " The '".truecolor(WARNING_COLOR_LUT[1].0, WARNING_COLOR_LUT[1].1, WARNING_COLOR_LUT[1].2), 
                input.bold(), 
                "' command does not exist.".truecolor(WARNING_COLOR_LUT[1].0, WARNING_COLOR_LUT[1].1, WARNING_COLOR_LUT[1].2));
            if show_commands { print_greeting_tutorial(); }
        }
    }

    Ok(())
}


fn read_input<T: std::str::FromStr + 'static>(
    mut input : String, at_text : &str, 
    pre_text : &str
) -> Result<T, &'static str> where <T as std::str::FromStr>::Err: std::fmt::Debug {
    
    println!("{}", pre_text);
    print!("{}({}): ", at_text, std::any::type_name::<T>().to_string().italic().dimmed());
    let _ = stdout().flush();
    input.clear();
    io::stdin().read_line(&mut input).expect("failed to read line");
    input.pop();

    if EXIT_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) { 
        return Result::Err(EXIT_INPUT_OPTIONS[0]);
    }

    let result = match input.parse::<T>() {
        Ok(value) => value,
        _ => return Result::Err(EXIT_INPUT_OPTIONS[0])
    };

    return Result::Ok(result);
}

fn read_input_to_bool(mut input : String, at_text : &str, pre_text : &str) -> Result<bool, &'static str> {
    
    println!("{}", pre_text);
    print!("{}({}): ", at_text, std::any::type_name::<bool>().to_string().italic().dimmed());
    let _ = stdout().flush();
    input.clear();
    io::stdin().read_line(&mut input).expect("failed to read line");
    input.pop();

    if EXIT_INPUT_OPTIONS.contains(&input.to_lowercase().as_str()) { 
        return Result::Err(EXIT_INPUT_OPTIONS[0]);
    }

    return match input.as_str() {
        "yes"   => Result::Ok(true),
        "y"     => Result::Ok(true),
        "true"  => Result::Ok(true),
        "no"    => Result::Ok(false),
        "n"     => Result::Ok(false),
        "false" => Result::Ok(true),
        _       => Result::Ok(false)
    };
}
