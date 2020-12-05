use pt::core::Core;
use std::error::Error;
use pt::back::Back;

fn main() -> Result<(), Box<dyn Error>>{
    let mut core = Core::new("0.0.0.0:8189".to_string());
    let mut back = Back::new(&mut core);
    back.run()
}
