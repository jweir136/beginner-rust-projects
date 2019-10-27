use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let echo_string = &args[1];

    println!("{}", echo_string);
}
