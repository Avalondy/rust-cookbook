use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};
use rand_distr::Normal;

fn generate_random_numbers() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

fn generate_random_numbers_within_a_range() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));
}

fn generate_random_numbers_from_uniform() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {throw}");
        if throw == 6 {
            break;
        }
    }
}

fn generate_random_numbers_with_given_distribution() {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(2.0, 3.0).unwrap();
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);
}

fn main() {
    generate_random_numbers();
    generate_random_numbers_within_a_range();
    generate_random_numbers_from_uniform();
    generate_random_numbers_with_given_distribution();
}
