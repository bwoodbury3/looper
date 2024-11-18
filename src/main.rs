extern crate runner;

use std::env;

fn run(filename: &str) -> Result<(), String> {
    let mut looper = runner::Runner::new(filename)?;
    match looper.run() {
        Ok(v) => Ok(v),
        Err(_) => Err("Looper failed to initialize".to_string()),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    match run(&filename) {
        Ok(_) => {
            println!("Looper success");
        }
        Err(e) => {
            println!("{}", e.to_string());
        }
    };
}
