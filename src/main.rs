use crate::parser::parse;

mod parser;

const INSTANCE: &str = "./instances/Hospital1-Emergency-Normal.xml";

fn main() {
    
    // Parse the instance
    let instance = parse(INSTANCE);
    
    // View fields
    println!("{:?}", instance.schedule_period);
}
