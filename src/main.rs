use std::env;
use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
  
    
    let config = Config::new(&args);
    
    println!("archivo: {}", config.filename);
    println!("query: {}", config.query);
    
    mini_grep::run(config);
}

