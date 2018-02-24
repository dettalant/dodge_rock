mod args;

fn main() {
    // debug modeならtrue、そうでないならfalse
    let is_debug = args::new();
    println!("is_debug: {}", is_debug);
}
