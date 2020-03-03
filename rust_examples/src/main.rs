
fn main() {
    println!("Simple rust examples.");
let seed:usize = 34235623;
    let bit = rust_examples::random_generator::new_linear_feedback_shift(seed);
    for number in bit {
        println!("{}",number);
    }
}
