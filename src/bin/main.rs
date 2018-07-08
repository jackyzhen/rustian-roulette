extern crate rustian_roulette;

use rustian_roulette as rr;
use std::env;

fn main() {
    let config = rr::cmd::Config::new(env::args());
    let path_str = format!("{}", config.path.display());
    let file = rr::pick_file(config.path.as_path());
    let file = file.as_path();
    let file_str = file.to_str().unwrap();

    rr::printer::print_prob(&path_str, config.chambers);
    rr::printer::confirmation();
    rr::printer::count_down(5, file_str);

    if rr::fire(file, config.chambers) {
        rr::printer::rest_in_peace(file_str);
    } else {
        rr::printer::alive(file_str);
    }
}
