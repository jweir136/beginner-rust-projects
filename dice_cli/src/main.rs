use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n_sides: i32 = args[1].parse::<i32>().unwrap();

    println!("{}", roll_dice(n_sides));
}

fn roll_dice(n_sides: i32) -> i32 {
    rand::thread_rng().gen_range(1, n_sides + 1)
}
