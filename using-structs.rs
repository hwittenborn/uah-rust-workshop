/// This defines a `struct` to store data about a person.
struct Person {
    /// A person's name. The type of the data comes AFTER the variable name.
    name: String,
    /// A person's age. The `usize` type defines an unsigned integer (i.e. only positive values).
    age: usize
}

fn main() {
    // Create an instance of the `Person`.
    let person = Person {
        name: String::from("Programmer Dude"),
        age: 20,
    };

    println!("Hello, world! My name is {}, and my age is {}", person.name, person.age);
}
