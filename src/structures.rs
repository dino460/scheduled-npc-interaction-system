use std::rc::Rc;

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
    pub distance : Option<i32>,
    pub previous : Option<Rc<Point>>
}