use colored::*;

/*
    VERSIONING RULES:
    Scheduled NPC Interaction System v[W.X.Y.Z] (greek letter)
        v : decoration -> stands for "version"

        W : major version/revision/rework
        X : major feature drop
        Y : minor feature drop/major bug fix drop
        Z : minor changes/ minor bug fixes

        greek letter : denotes software/branch major state
            α => alpha   -> early production, main features not fully implemented and unsuitable for proper use
            β => beta    -> pre-release, main features implemented but not fully tested, may lack some minor features
            χ => release -> full release, ready for production and roll-out
            ν => nightly -> early release, somewhere between beta and release, suitable to test new minor or major features
*/
pub const VERSION_NUMBER  : &str      = "0.1.1.0";
pub const VERSION_TYPES   : [&str; 4] = ["α", "β", "χ", "ν"];
pub const CURRENT_VERSION : usize     = 0;

pub const ERROR_SYMBOL   : &str = "(!)";
pub const INFO_SYMBOL    : &str = "(*)";
pub const WARNING_SYMBOL : &str = "(?)";

pub const GENERATE_INPUT_OPTIONS     : [&str; 3] = ["generate", "gen", "g"];
pub const EXIT_INPUT_OPTIONS         : [&str; 3] = ["exit", "quit", "q"];
pub const SAVE_INPUT_OPTIONS         : [&str; 2] = ["save", "s"];
pub const LOAD_INPUT_OPTIONS         : [&str; 2] = ["load", "l"];
pub const CLEAR_INPUT_OPTIONS        : [&str; 3] = ["clear", "clr", "c"];
pub const PRINT_INPUT_OPTIONS        : [&str; 2] = ["print", "p"];
pub const FIND_INPUT_OPTIONS         : [&str; 2] = ["find", "f"];
pub const SET_SIZE_INPUT_OPTIONS     : [&str; 3] = ["set", "ss", "set size"];
pub const HIDE_GEN_INPUT_OPTIONS     : [&str; 4] = ["hide gen", "hidegen", "hg", "hidg"];
pub const HIDE_COMMAND_OPTIONS       : [&str; 5] = ["hide cmd", "hidecmd", "hc", "hidc", "hidcmd"];
pub const SHOW_ALL_COMMANDS_OPTIONS  : [&str; 3] = ["show all", "shall", "sa"];
pub const RESET_SCREEN_INPUT_OPTIONS : [&str; 3] = ["reset", "rst", "r"];


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

pub fn print_title() {
    clear_screen();
    println!();
    println!("{}", "███████╗███╗   ██╗██╗███████╗".bright_red());
    println!("{}", "██╔════╝████╗  ██║██║██╔════╝".bright_red());
    println!("{}", "███████╗██╔██╗ ██║██║███████╗".bright_red());
    println!("{}", "╚════██║██║╚██╗██║██║╚════██║".bright_red());
    println!("{}", "███████║██║ ╚████║██║███████║".bright_red());
    println!("{}", "╚══════╝╚═╝  ╚═══╝╚═╝╚══════╝".bright_red());
    println!("{}", "~by dino460".bright_red().italic());
    println!();
    println!("{}{}{}{}{}", 
        "Welcome to the ".italic().bright_red(), 
        "Scheduled NPC Interaction System ".italic().bright_red().bold(), 
        "v"/*.italic()*/.bright_red()/*.underline()*/, 
        VERSION_NUMBER/*.italic()*/.bright_red()/*.underline()*/, 
        VERSION_TYPES[CURRENT_VERSION].italic().bright_red()/*.underline()*/
    );
}

pub fn print_greeting_tutorial() {
    println!();
    println!("{}", "COMMANDS:".bold());
    println!("========================================================================");
    println!("{}{:?}", " - Generate new random map      : ".italic().dimmed(), GENERATE_INPUT_OPTIONS);
    println!("{}{:?}", " - Save current map to file     : ".italic().dimmed(), SAVE_INPUT_OPTIONS);
    println!("{}{:?}", " - Load map from file           : ".italic().dimmed(), LOAD_INPUT_OPTIONS);
    println!("{}{:?}", " - Find path between two points : ".italic().dimmed(), FIND_INPUT_OPTIONS);
    println!("{}{:?}", " - Set map size (NxN matrix)    : ".italic().dimmed(), SET_SIZE_INPUT_OPTIONS);
    println!("{}{:?}", " - Print current map            : ".italic().dimmed(), PRINT_INPUT_OPTIONS);
    println!("{}{:?}", " - Show all commands            : ".italic().dimmed(), SHOW_ALL_COMMANDS_OPTIONS);
    println!("{}{:?}", " - Reset screen to greeting     : ".italic().dimmed(), RESET_SCREEN_INPUT_OPTIONS);
    println!("{}{:?}", " - Clear console                : ".italic().dimmed(), CLEAR_INPUT_OPTIONS);
    println!("{}{:?}", " - Exit program                 : ".italic().dimmed(), EXIT_INPUT_OPTIONS);
    println!("========================================================================");
    // println!();
    println!("{}", "Input field:".bold());
    println!("\" {}{}{}{}{} \"", 
        "[show map on gen|show commands] ".italic().green(), 
        "(".italic().cyan(), 
        "input::type".italic().dimmed().cyan(), 
        "):".italic().cyan(), 
        " Your input comes here".italic().blue());
    println!();
}

pub fn print_all_commands() {
    clear_screen();
    println!();
    println!("{}", "COMMANDS (simplified list):".bold());
    println!("========================================================================");
    println!("{}{:?}", " - Generate new random map      : ".italic().dimmed(), GENERATE_INPUT_OPTIONS);
    println!("{}{:?}", " - Save current map to file     : ".italic().dimmed(), SAVE_INPUT_OPTIONS);
    println!("{}{:?}", " - Load map from file           : ".italic().dimmed(), LOAD_INPUT_OPTIONS);
    println!("{}{:?}", " - Find path between two points : ".italic().dimmed(), FIND_INPUT_OPTIONS);
    println!("{}{:?}", " - Set map size (NxN matrix)    : ".italic().dimmed(), SET_SIZE_INPUT_OPTIONS);
    println!("{}{:?}", " - Print current map            : ".italic().dimmed(), PRINT_INPUT_OPTIONS);
    println!("{}{:?}", " - Show all commands            : ".italic().dimmed(), SHOW_ALL_COMMANDS_OPTIONS);
    println!("{}{:?}", " - Toggle showing map on gen    : ".italic().dimmed(), HIDE_GEN_INPUT_OPTIONS);
    println!("{}{:?}", " - Toggle showing commands      : ".italic().dimmed(), HIDE_COMMAND_OPTIONS);
    println!("{}{:?}", " - Reset screen to greeting     : ".italic().dimmed(), RESET_SCREEN_INPUT_OPTIONS);
    println!("{}{:?}", " - Clear console                : ".italic().dimmed(), CLEAR_INPUT_OPTIONS);
    println!("{}{:?}", " - Exit program                 : ".italic().dimmed(), EXIT_INPUT_OPTIONS);
    println!("========================================================================");
}

pub fn clear_screen() {
    std::process::Command::new("clear").status().unwrap();
}
