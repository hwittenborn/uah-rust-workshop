/// Defining a new `enum` to store classes.
enum Classes {
    Math,
    English,
    ComputerScience
}

/// A function that returns a `Classes` instance. The type of return variables MUST be marked with
/// the `->` syntax.
fn get_favorite_class() -> Classes {
    return Classes::Math;
}

fn main() {
    // Get one of the possible options for a class.
    let class = get_favorite_class();

    match class {
        Classes::Math => println!("My favorite class is Math!"),
        Classes::English => println!("My favorite class is English!"),
        Classes::ComputerScience => println!("My favorite class is Computer Science!"),
    }
}
