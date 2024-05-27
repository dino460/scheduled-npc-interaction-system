use colored::*;


pub const GENERATE_INPUT_OPTIONS : [&str; 3] = ["generate", "gen", "g"];
pub const EXIT_INPUT_OPTIONS     : [&str; 3] = ["exit", "quit", "q"];
pub const SAVE_INPUT_OPTIONS     : [&str; 2] = ["save", "s"];
pub const LOAD_INPUT_OPTIONS     : [&str; 2] = ["load", "l"];
pub const CLEAR_INPUT_OPTIONS    : [&str; 3] = ["clear", "clr", "c"];
pub const PRINT_INPUT_OPTIONS    : [&str; 2] = ["print", "p"];
pub const FIND_INPUT_OPTIONS     : [&str; 2] = ["find", "f"];
pub const SET_SIZE_INPUT_OPTIONS : [&str; 3] = ["set", "ss", "set size"];
pub const STOP_INPUT_OPTIONS     : [&str; 1] = ["stop"];
pub const SHOW_GEN_INPUT_OPTIONS : [&str; 2] = ["show", "sh"];


pub fn print_matrix(matrix : &Vec<Vec<usize>>) {
    // * Print the generated matrices for better visualization
    println!();
    println!("{}", "World matrix");
    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            match matrix[i][j] {
                0 => print!("{} ", matrix[i][j].to_string().red()),
                1 => print!("{} ", matrix[i][j].to_string().green()),
                2 => print!("{} ", matrix[i][j].to_string().blue()),
                3 => print!("{} ", matrix[i][j].to_string().bright_yellow()),
                4 => print!("{} ", matrix[i][j].to_string().bright_blue()),
                _ => print!("{} ", matrix[i][j])
            }
        }
        println!();
    }
    println!();
}