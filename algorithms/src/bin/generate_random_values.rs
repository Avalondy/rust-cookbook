#![allow(dead_code)]

use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};
use rand_distr::{Alphanumeric, Normal};

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

mod custom_type {
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Distribution<Point> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
            let (x, y) = rng.gen();
            Point { x, y }
        }
    }

    pub fn generate_random_values_of_a_custom_type() {
        let mut rng = rand::thread_rng();
        let rand_tuple = rng.gen::<(i32, bool, f64)>();
        let rand_point: Point = rng.gen();
        println!("Random tuple: {:?}", rand_tuple);
        println!("Random Point: {:?}", rand_point);
    }
}

fn generate_random_alphanumeric_characters() {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    println!("Random alphanumeric password: {}", rand_string);
}

fn generate_random_from_user_defined_characters() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                    abcdefghijklmnopqrstuvwxyz\
                    0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();
    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    println!("{:?}", password);
}

fn main() {
    generate_random_numbers();
    generate_random_numbers_within_a_range();
    generate_random_numbers_from_uniform();
    generate_random_numbers_with_given_distribution();
    custom_type::generate_random_values_of_a_custom_type();
    generate_random_alphanumeric_characters();
    generate_random_from_user_defined_characters();
}
