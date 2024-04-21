struct Personality {
    aggressiveness : Option<f32>,
    empathy : Option<f32>,
    lazyness : Option<f32>
}

struct Person {
    personality : Personality
}

fn main() {
    let test = Person {
        personality : Personality {
            aggressiveness : Some(0.1),
            empathy : Some(0.8),
            lazyness : Some(-0.3)
        }
    };

    println!("{:?}", test.personality.aggressiveness.unwrap());
}
