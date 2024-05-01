use std::env;
use std::process;

use ggway::Query;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = Query::new(&args);

    if let Err(e) = ggway::run(query) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
