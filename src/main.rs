fn main() {
    println!("Learning Rust - Vectors!");

    create_vector();
    pass_all_data_types_to_vectors();
}

pub fn create_vector() {
    // Create a new mutable vector of type i32.
    let mut new_vector: Vec<i32> = Vec::new();

    // Print out the content of the vector.
    println!("Vector content: {:?}", new_vector);

    // Push a value into the vector.
    new_vector.push(7);

    // Print out the new content of the vector.
    println!("Vector content: {:?}", new_vector);
}

pub fn pass_all_data_types_to_vectors() {
    // integer values.
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    println!("Numbers content: {:?}", numbers);

    // floating point values.
    let temperatures: Vec<f64> = vec![36.7, 27.7, 15.7];
    println!("Temperatures content: {:?}", temperatures);

    // strings.
    let names: Vec<String> = vec!["Bob".to_string(), "Alice".to_string()];
    println!("Names content: {:?}", names);

    // booleans.
    let flags: Vec<bool> = vec![true, false, true, false];
    println!("Flags content: {:?}", flags);


    // custom structs.
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let people: Vec<Person> = vec![
        Person { name: "Bob".to_string(), age: 27 },
        Person { name: "Alice".to_string(), age: 37 },
    ];
    println!("People content: {:?}", people);
    println!("Person 1 name: {:?}", people[0].name);
    println!("Person 1 age: {:?}", people[0].age);

    // enums.
    #[derive(Debug)]
    enum FilterOptions {
        Free,
        Paid,
        Bonus,
    }

    let filters: Vec<FilterOptions> = vec![FilterOptions::Free, FilterOptions::Paid, FilterOptions::Bonus];
    println!("Filters content: {:?}", filters);

    // mixed types using trait objects.
    let mixed: Vec<Box<dyn std::fmt::Debug>> = vec![
        Box::new(47),
        Box::new("Alice".to_string()),
    ];
    println!("Mixed content: {:?}", mixed);
}