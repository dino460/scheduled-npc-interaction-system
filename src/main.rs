use rand::Rng;


#[derive(Clone, Copy, Debug)]
struct Job {
    job_type : u8
}

#[derive(Debug)]
struct World {
    paths : [[u32; 10]; 10],
    jobs : [[Job; 10]; 10]
}


fn main() {
    let world : World = World {
        paths : [[rand::thread_rng().gen_range(0..2); 10]; 10],
        jobs : [[Job { job_type : rand::thread_rng().gen_range(0..2) }; 10]; 10]
    };

    for i in 0..10 {
        for j in 0..10 {
            print!("{} ", world.paths[i][j]);
        }
        print!("\n");
    }
    
    println!();
    
    for i in 0..10 {
        for j in 0..10 {
            print!("{} ", world.jobs[i][j].job_type);
        }
        print!("\n");
    }
}
