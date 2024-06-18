use std::rc::Rc;
use std::fmt;


#[derive(Clone, Copy, Debug)]
pub struct Job {
    pub job_type : usize
}

#[derive(Debug)]
pub struct World {
    pub paths          : Vec<Vec<usize>>,
    pub smoothed_paths : Vec<Vec<usize>>,
    pub jobs           : Vec<Vec<usize>>,
    pub weights        : Vec<Vec<usize>>,
    pub world          : Vec<Vec<usize>>
}

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub struct Point {
    pub i        : Option<i32>,
    pub j        : Option<i32>,
    pub distance : Option<f32>,
    pub previous : Option<Rc<Point>>
}

#[derive(Debug)]
pub struct Benchmark {
    pub matrix_size   : Option<usize>,
    pub building_time : Option<f32>,
    pub finding_time  : Option<f32>,
    pub heuristic_d   : Option<f32>
}

impl fmt::Display for Benchmark {
        
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {

        write!(
            f, "Matrix size: {}\n Gen time: {}\n Find time: {}\n D: {}", 
            self.matrix_size.unwrap(), 
            self.building_time.unwrap(), 
            self.finding_time.unwrap(),
            self.heuristic_d.unwrap()
        )
    }
}