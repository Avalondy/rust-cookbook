fn sort_vectors_of_numbers() {
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort();
    println!("{vec:?}");

    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    vec.sort_by(|x, y| x.partial_cmp(y).unwrap());
    println!("{vec:?}");
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

fn sort_vectors_of_structs() {
    let mut people = vec![
        Person::new("John", 5),
        Person::new("Zoe", 25),
        Person::new("Al", 60),
        Person::new("John", 1),
    ];

    // Sort people by name, and age if necessary
    people.sort();
    println!("{people:#?}");

    // Sort by age instead
    people.sort_by(|a, b| b.age.cmp(&a.age));
    println!("{people:#?}");
}

fn main() {
    sort_vectors_of_numbers();
    sort_vectors_of_structs();
}
